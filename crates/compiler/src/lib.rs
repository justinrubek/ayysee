use ayysee_parser::ast::*;
use stationeers_mips::types::{Device, DeviceVariable};
use std::collections::HashMap;
use std::str::FromStr;

pub mod error;
use error::{Error, Result};

/// Converts an entire program into MIPS assembly code.
/// This is the public entry point used by the CLI and WASM frontends.
pub fn generate_program(program: Program) -> Result<String> {
    let mut compiler = Compiler::new();
    compiler.compile(program)
}

/// Represents a value that can be used as an instruction operand.
enum Operand {
    /// A register holding a computed value (index 0-15).
    Reg(u8),
    /// An immediate value: a literal number or a `define`d constant name.
    Imm(String),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::Reg(i) => write!(f, "r{}", i),
            Operand::Imm(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone)]
struct FunctionDef {
    parameters: Vec<Identifier>,
    body: Block,
}

/// Single-pass compiler that translates the AST into Stationeers MIPS assembly.
///
/// Design:
/// - variables are allocated to registers r0..r14 (growing upward).
/// - expression temporaries are allocated r15 downward, freed after each statement.
/// - functions are inlined at call sites to avoid jal/return overhead.
/// - named labels are used for all flow control.
/// - constants use `define`, device aliases use `alias`.
struct Compiler {
    /// Accumulated output lines of MIPS assembly.
    lines: Vec<String>,
    /// Variable name -> register index.
    variables: HashMap<String, u8>,
    /// Device alias name -> raw device string (e.g. "GasSensor" -> "d0").
    devices: HashMap<String, String>,
    /// Constant name -> value string (mirrors `define` declarations).
    constants: HashMap<String, String>,
    /// Collected function definitions, keyed by name.
    functions: HashMap<String, FunctionDef>,
    /// Next register index for permanent variables (grows from 0 upward).
    next_var_reg: u8,
    /// Next register index for temporaries (grows from 17 downward).
    /// Signed to detect exhaustion without underflow.
    next_temp_reg: i8,
    /// Monotonic counter for generating unique labels.
    label_counter: u32,
}

impl Compiler {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            variables: HashMap::new(),
            devices: HashMap::new(),
            constants: HashMap::new(),
            functions: HashMap::new(),
            next_var_reg: 0,
            next_temp_reg: 17,
            label_counter: 0,
        }
    }

    fn emit(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }

    /// Allocate a permanent register for a variable.
    /// Returns the existing register if the variable was already allocated.
    fn alloc_var(&mut self, name: String) -> u8 {
        if let Some(&existing) = self.variables.get(&name) {
            return existing;
        }
        assert!(
            self.next_var_reg < 18,
            "out of registers: too many variables"
        );
        let reg = self.next_var_reg;
        self.next_var_reg += 1;
        self.variables.insert(name, reg);
        reg
    }

    /// Allocate a temporary register for expression evaluation.
    fn alloc_temp(&mut self) -> u8 {
        assert!(
            self.next_temp_reg >= self.next_var_reg as i8,
            "out of registers: too many temporaries"
        );
        let reg = self.next_temp_reg as u8;
        self.next_temp_reg -= 1;
        reg
    }

    /// Free all temporary registers. Called at the start of each statement.
    fn reset_temps(&mut self) {
        self.next_temp_reg = 17;
    }

    /// Generate a unique label with the given prefix.
    fn unique_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Ensure an operand is in a register. Loads immediates into a temp if needed.
    fn ensure_reg(&mut self, op: Operand) -> u8 {
        match op {
            Operand::Reg(r) => r,
            Operand::Imm(s) => {
                let reg = self.alloc_temp();
                self.emit(format!("move r{} {}", reg, s));
                reg
            }
        }
    }

    /// Emit a move instruction, eliding self-moves
    fn emit_move(&mut self, target: u8, val: &Operand) {
        match val {
            Operand::Reg(r) if *r == target => {}
            _ => self.emit(format!("move r{} {}", target, val)),
        }
    }

    /// Resolve a device alias to its raw device string (e.g. "d0")
    fn resolve_device(&self, identifier: &Identifier) -> Result<String> {
        let name: &str = identifier.as_ref();
        if let Some(device) = self.devices.get(name) {
            Ok(device.clone())
        } else {
            // try as a raw device reference (d0, d1, etc.)
            Device::from_str(name)?;
            Ok(name.to_string())
        }
    }

    /// Validate and return a device variable name
    fn resolve_device_var(&self, identifier: &Identifier) -> Result<String> {
        let name: &str = identifier.as_ref();
        DeviceVariable::from_str(name)?;
        Ok(name.to_string())
    }

    /// Main compilation entry point
    fn compile(&mut self, program: Program) -> Result<String> {
        let mut has_main = false;

        // first: collect all top-level declarations (aliases, constants, functions)
        for stmt in &program.statements {
            match stmt {
                Statement::Function {
                    identifier,
                    parameters,
                    body,
                } => {
                    if identifier.to_string() == "main" {
                        has_main = true;
                    }
                    self.functions.insert(
                        identifier.to_string(),
                        FunctionDef {
                            parameters: parameters.clone(),
                            body: body.clone(),
                        },
                    );
                }
                Statement::Alias { identifier, alias } => {
                    let dev = identifier.to_string();
                    Device::from_str(&dev)?;
                    self.devices.insert(alias.to_string(), dev.clone());
                    self.emit(format!("alias {} {}", alias, dev));
                }
                Statement::Constant(identifier, value) => {
                    let val = value_to_string(value);
                    self.constants.insert(identifier.to_string(), val.clone());
                    self.emit(format!("define {} {}", identifier, val));
                }
                _ => {}
            }
        }

        if !has_main {
            return Err(Error::UndefinedMain);
        }

        let main_fn = self.functions.get("main").cloned().unwrap();
        self.compile_block(&main_fn.body)?;

        Ok(self.lines.join("\n"))
    }

    fn compile_block(&mut self, block: &Block) -> Result<()> {
        let Block::Statements(stmts) = block;
        for stmt in stmts {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        self.reset_temps();

        match stmt {
            Statement::Definition {
                identifier,
                expression,
            } => {
                let val = self.compile_expr(expression)?;
                let reg = self.alloc_var(identifier.to_string());
                self.emit_move(reg, &val);
            }

            Statement::Assignment {
                identifier,
                expression,
            } => {
                let name: &str = identifier.as_ref();
                let target = *self
                    .variables
                    .get(name)
                    .ok_or_else(|| Error::UndefinedVariable(identifier.to_string()))?;
                let val = self.compile_expr(expression)?;
                self.emit_move(target, &val);
            }

            Statement::Loop { body } => {
                let label = self.unique_label("loop");
                self.emit(format!("{}:", label));
                self.compile_block(body)?;
                self.emit(format!("j {}", label));
            }

            Statement::IfStatement(if_stmt) => {
                self.compile_if(if_stmt)?;
            }

            Statement::DeviceStatement(dev_stmt) => {
                self.compile_device(dev_stmt)?;
            }

            Statement::Yield => {
                self.emit("yield");
            }

            Statement::FunctionCall {
                identifier,
                arguments,
            } => {
                self.compile_function_call(identifier, arguments)?;
            }

            Statement::Block(block) => {
                self.compile_block(block)?;
            }

            // these are handled at the top-level scan, but can also appear inside function bodies
            Statement::Alias { identifier, alias } => {
                let dev = identifier.to_string();
                Device::from_str(&dev)?;
                self.devices.insert(alias.to_string(), dev.clone());
                self.emit(format!("alias {} {}", alias, dev));
            }

            Statement::Constant(identifier, value) => {
                let val = value_to_string(value);
                self.constants.insert(identifier.to_string(), val.clone());
                self.emit(format!("define {} {}", identifier, val));
            }

            Statement::Function { .. } => {
                // already collected during top-level scan.
            }
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<Operand> {
        match expr {
            Expr::Constant(value) => Ok(Operand::Imm(value_to_string(value))),

            Expr::Identifier(id) => {
                let name: &str = id.as_ref();
                if self.constants.contains_key(name) {
                    // Reference the `define`d name directly as an immediate.
                    Ok(Operand::Imm(id.to_string()))
                } else if let Some(&reg) = self.variables.get(name) {
                    Ok(Operand::Reg(reg))
                } else {
                    Err(Error::UndefinedVariable(id.to_string()))
                }
            }

            Expr::BinaryOp(left, op, right) => {
                let left_val = self.compile_expr(left)?;
                let right_val = self.compile_expr(right)?;
                let result = self.alloc_temp();

                let instr = match op {
                    BinaryOpcode::Add => "add",
                    BinaryOpcode::Sub => "sub",
                    BinaryOpcode::Mul => "mul",
                    BinaryOpcode::Div => "div",
                    BinaryOpcode::Equals => "seq",
                    BinaryOpcode::NotEquals => "sne",
                    BinaryOpcode::Greater => "sgt",
                    BinaryOpcode::GreaterEquals => "sge",
                    BinaryOpcode::Lower => "slt",
                    BinaryOpcode::LowerEquals => "sle",
                    BinaryOpcode::Conj => "and",
                    BinaryOpcode::Disj => "or",
                };

                self.emit(format!("{} r{} {} {}", instr, result, left_val, right_val));
                Ok(Operand::Reg(result))
            }

            Expr::UnaryOp(op, operand) => {
                let val = self.compile_expr(operand)?;
                let result = self.alloc_temp();
                match op {
                    UnaryOpcode::Not => {
                        // seqz: result = 1 if val == 0, else 0
                        self.emit(format!("seqz r{} {}", result, val));
                    }
                }
                Ok(Operand::Reg(result))
            }
        }
    }

    fn compile_if(&mut self, if_stmt: &IfStatement) -> Result<()> {
        match if_stmt {
            IfStatement::If { condition, body } => {
                let cond = self.compile_expr(condition)?;
                let cond_reg = self.ensure_reg(cond);
                let end_label = self.unique_label("endif");
                self.emit(format!("beqz r{} {}", cond_reg, end_label));
                self.compile_block(body)?;
                self.emit(format!("{}:", end_label));
            }

            IfStatement::IfElse {
                condition,
                body,
                else_body,
            } => {
                let cond = self.compile_expr(condition)?;
                let cond_reg = self.ensure_reg(cond);
                let else_label = self.unique_label("else");
                let end_label = self.unique_label("endif");
                self.emit(format!("beqz r{} {}", cond_reg, else_label));
                self.compile_block(body)?;
                self.emit(format!("j {}", end_label));
                self.emit(format!("{}:", else_label));
                self.compile_block(else_body)?;
                self.emit(format!("{}:", end_label));
            }
        }
        Ok(())
    }

    fn compile_device(&mut self, dev_stmt: &DeviceStatement) -> Result<()> {
        match dev_stmt {
            DeviceStatement::Read {
                device,
                device_variable,
                local,
            } => {
                let local_name: &str = local.as_ref();
                let target = *self
                    .variables
                    .get(local_name)
                    .ok_or_else(|| Error::UndefinedVariable(local.to_string()))?;
                let dev = self.resolve_device(device)?;
                let var = self.resolve_device_var(device_variable)?;
                self.emit(format!("l r{} {} {}", target, dev, var));
            }

            DeviceStatement::Write {
                value,
                device,
                device_variable,
            } => {
                let val = self.compile_expr(value)?;
                let val_reg = self.ensure_reg(val);
                let dev = self.resolve_device(device)?;
                let var = self.resolve_device_var(device_variable)?;
                self.emit(format!("s {} {} r{}", dev, var, val_reg));
            }
        }
        Ok(())
    }

    /// Inline a function call. Evaluates arguments, creates a fresh scope for the
    /// function's parameters and locals, compiles the body, then restores the caller's scope.
    fn compile_function_call(
        &mut self,
        identifier: &Identifier,
        arguments: &[Box<Expr>],
    ) -> Result<()> {
        let name = identifier.to_string();
        let func = self
            .functions
            .get(&name)
            .cloned()
            .ok_or_else(|| Error::UndefinedFunction(name))?;

        // evaluate arguments in caller's scope
        let mut arg_vals: Vec<Operand> = Vec::new();
        for arg in arguments {
            arg_vals.push(self.compile_expr(arg)?);
        }

        // save caller's variable scope
        let saved_vars = std::mem::take(&mut self.variables);
        let saved_next_var_reg = self.next_var_reg;

        // assign arguments to parameter registers in the function's scope
        for (param, arg_val) in func.parameters.iter().zip(arg_vals.iter()) {
            let reg = self.alloc_var(param.to_string());
            self.emit_move(reg, arg_val);
        }

        // compile function body inline.
        self.compile_block(&func.body)?;

        // restore caller's scope.
        self.variables = saved_vars;
        self.next_var_reg = saved_next_var_reg;

        Ok(())
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => format!("{}", f),
        Value::Boolean(b) => {
            if *b {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
    }
}
