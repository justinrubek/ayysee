use ayysee_parser::ast::*;
use stationeers_mips::types::Device;
use std::collections::HashMap;
use std::collections::HashSet;
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
    /// A register holding a computed value (index 0-17).
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

/// Register index where called-function parameters begin.
/// Main uses r0..FUNC_REG_BASE-1, functions use FUNC_REG_BASE..17.
const FUNC_REG_BASE: u8 = 10;

/// Single-pass compiler that translates the AST into Stationeers MIPS assembly.
///
/// Design:
/// - Variables are allocated to registers r0..r9 (growing upward) for main.
/// - Function parameters and locals use r10..r17 (FUNC_REG_BASE upward).
/// - Expression temporaries are allocated r17 downward, freed after each statement.
/// - Functions called more than once are emitted once with a label and called via jal/j ra.
/// - Named labels are used for all flow control.
/// - Constants use `define`, device aliases use `alias`.
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
    /// Stack of loop labels for `break` support.
    loop_stack: Vec<String>,
    /// Functions that have been compiled as callable (emitted after main).
    compiled_functions: HashSet<String>,
    /// Function bodies to emit after main.
    deferred_lines: Vec<String>,
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
            loop_stack: Vec::new(),
            compiled_functions: HashSet::new(),
            deferred_lines: Vec::new(),
        }
    }

    fn emit(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }

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

    fn alloc_temp(&mut self) -> u8 {
        assert!(
            self.next_temp_reg >= self.next_var_reg as i8,
            "out of registers: too many temporaries"
        );
        let reg = self.next_temp_reg as u8;
        self.next_temp_reg -= 1;
        reg
    }

    fn reset_temps(&mut self) {
        self.next_temp_reg = 17;
    }

    fn unique_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

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

    fn emit_move(&mut self, target: u8, val: &Operand) {
        match val {
            Operand::Reg(r) if *r == target => {}
            _ => self.emit(format!("move r{} {}", target, val)),
        }
    }

    /// Compile a device expression. Handles:
    /// - Device aliases (GasSensor -> "d0")
    /// - Raw device refs (d0 -> "d0")
    /// - Variables holding ReferenceIds (ref_id -> "r3")
    /// - Constants (SomeHash -> "SomeHash")
    fn compile_device_expr(&mut self, expr: &Expr) -> Result<String> {
        if let Expr::Identifier(id) = expr {
            let name: &str = id.as_ref();
            if let Some(device) = self.devices.get(name) {
                return Ok(device.clone());
            }
            if Device::from_str(name).is_ok() {
                return Ok(name.to_string());
            }
            if self.constants.contains_key(name) {
                return Ok(name.to_string());
            }
            if let Some(&reg) = self.variables.get(name) {
                return Ok(format!("r{}", reg));
            }
            Err(Error::UndefinedVariable(id.to_string()))
        } else {
            let val = self.compile_expr(expr)?;
            let reg = self.ensure_reg(val);
            Ok(format!("r{}", reg))
        }
    }

    /// Returns the device variable name. No validation is performed since
    /// the game has hundreds of logic types that change between updates.
    /// The game itself will report errors for invalid property names.
    fn resolve_device_var(&self, identifier: &Identifier) -> String {
        identifier.to_string()
    }

    fn compile(&mut self, program: Program) -> Result<String> {
        let mut has_main = false;

        // collect top-level declarations
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

        // append deferred function bodies after main
        self.lines.extend(self.deferred_lines.drain(..));

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
                let end_label = format!("{}_end", label);
                self.loop_stack.push(end_label.clone());
                self.emit(format!("{}:", label));
                self.compile_block(body)?;
                self.emit(format!("j {}", label));
                self.emit(format!("{}:", end_label));
                self.loop_stack.pop();
            }

            Statement::Break => {
                let end_label = self
                    .loop_stack
                    .last()
                    .cloned()
                    .ok_or(Error::BreakOutsideLoop)?;
                self.emit(format!("j {}", end_label));
            }

            Statement::Sleep { duration } => {
                let val = self.compile_expr(duration)?;
                self.emit(format!("sleep {}", val));
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

            Statement::Function { .. } => {}
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<Operand> {
        match expr {
            Expr::Constant(value) => Ok(Operand::Imm(value_to_string(value))),

            Expr::Identifier(id) => {
                let name: &str = id.as_ref();
                if self.constants.contains_key(name) {
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
                    BinaryOpcode::Mod => "mod",
                    BinaryOpcode::Pow => "pow",
                    BinaryOpcode::Equals => "seq",
                    BinaryOpcode::NotEquals => "sne",
                    BinaryOpcode::Greater => "sgt",
                    BinaryOpcode::GreaterEquals => "sge",
                    BinaryOpcode::Lower => "slt",
                    BinaryOpcode::LowerEquals => "sle",
                    BinaryOpcode::Conj => "and",
                    BinaryOpcode::Disj => "or",
                    BinaryOpcode::BitAnd => "and",
                    BinaryOpcode::BitOr => "or",
                    BinaryOpcode::BitXor => "xor",
                    BinaryOpcode::ShiftLeft => "sll",
                    BinaryOpcode::ShiftRight => "srl",
                };

                self.emit(format!("{} r{} {} {}", instr, result, left_val, right_val));
                Ok(Operand::Reg(result))
            }

            Expr::UnaryOp(op, operand) => {
                let val = self.compile_expr(operand)?;
                let result = self.alloc_temp();
                match op {
                    UnaryOpcode::Not => self.emit(format!("seqz r{} {}", result, val)),
                    UnaryOpcode::BitNot => self.emit(format!("not r{} {}", result, val)),
                }
                Ok(Operand::Reg(result))
            }

            Expr::Call(name, args) => self.compile_builtin_call(name, args),

            Expr::Ternary(condition, true_val, false_val) => {
                let cond = self.compile_expr(condition)?;
                let cond_reg = self.ensure_reg(cond);
                let t = self.compile_expr(true_val)?;
                let f = self.compile_expr(false_val)?;
                let result = self.alloc_temp();
                // select: result = t if cond != 0, else f
                self.emit(format!("select r{} r{} {} {}", result, cond_reg, t, f));
                Ok(Operand::Reg(result))
            }
        }
    }

    /// Compile a built-in function call (abs, sqrt, min, max, etc.).
    fn compile_builtin_call(&mut self, name: &Identifier, args: &[Box<Expr>]) -> Result<Operand> {
        let fname = name.to_string();
        let result = self.alloc_temp();

        match fname.as_str() {
            // 0-arg functions
            "rand" => {
                check_arg_count(&fname, 0, args.len())?;
                self.emit(format!("rand r{}", result));
            }

            // 1-arg functions
            "abs" | "sqrt" | "floor" | "ceil" | "round" | "trunc" | "exp" | "log" | "sin"
            | "cos" | "tan" | "asin" | "acos" | "atan" => {
                check_arg_count(&fname, 1, args.len())?;
                let a = self.compile_expr(&args[0])?;
                self.emit(format!("{} r{} {}", fname, result, a));
            }

            // 2-arg functions
            "min" | "max" | "atan2" | "pow" => {
                check_arg_count(&fname, 2, args.len())?;
                let a = self.compile_expr(&args[0])?;
                let b = self.compile_expr(&args[1])?;
                self.emit(format!("{} r{} {} {}", fname, result, a, b));
            }

            // 3-arg functions
            "lerp" => {
                check_arg_count(&fname, 3, args.len())?;
                let a = self.compile_expr(&args[0])?;
                let b = self.compile_expr(&args[1])?;
                let c = self.compile_expr(&args[2])?;
                self.emit(format!("lerp r{} {} {} {}", result, a, b, c));
            }

            _ => return Err(Error::UnknownBuiltin(fname)),
        }

        Ok(Operand::Reg(result))
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
                let dev = self.compile_device_expr(device)?;
                let var = self.resolve_device_var(device_variable);
                self.emit(format!("l r{} {} {}", target, dev, var));
            }

            DeviceStatement::Write {
                value,
                device,
                device_variable,
            } => {
                let val = self.compile_expr(value)?;
                let val_reg = self.ensure_reg(val);
                let dev = self.compile_device_expr(device)?;
                let var = self.resolve_device_var(device_variable);
                self.emit(format!("s {} {} r{}", dev, var, val_reg));
            }

            DeviceStatement::BatchRead {
                hash,
                device_variable,
                mode,
                local,
            } => {
                let local_name: &str = local.as_ref();
                let target = *self
                    .variables
                    .get(local_name)
                    .ok_or_else(|| Error::UndefinedVariable(local.to_string()))?;
                let hash_val = self.compile_expr(hash)?;
                let var = self.resolve_device_var(device_variable);
                let batch_mode = resolve_batch_mode(mode)?;
                self.emit(format!(
                    "lb r{} {} {} {}",
                    target, hash_val, var, batch_mode
                ));
            }

            DeviceStatement::BatchWrite {
                value,
                hash,
                device_variable,
            } => {
                let val = self.compile_expr(value)?;
                let val_reg = self.ensure_reg(val);
                let hash_val = self.compile_expr(hash)?;
                let var = self.resolve_device_var(device_variable);
                self.emit(format!("sb {} {} r{}", hash_val, var, val_reg));
            }

            DeviceStatement::SlotRead {
                device,
                slot,
                slot_variable,
                local,
            } => {
                let local_name: &str = local.as_ref();
                let target = *self
                    .variables
                    .get(local_name)
                    .ok_or_else(|| Error::UndefinedVariable(local.to_string()))?;
                let dev = self.compile_device_expr(device)?;
                let slot_val = self.compile_expr(slot)?;
                let svar: &str = slot_variable.as_ref();
                self.emit(format!("ls r{} {} {} {}", target, dev, slot_val, svar));
            }

            DeviceStatement::SlotWrite {
                value,
                device,
                slot,
                slot_variable,
            } => {
                let val = self.compile_expr(value)?;
                let val_reg = self.ensure_reg(val);
                let dev = self.compile_device_expr(device)?;
                let slot_val = self.compile_expr(slot)?;
                let svar: &str = slot_variable.as_ref();
                self.emit(format!("ss {} {} {} r{}", dev, slot_val, svar, val_reg));
            }
        }
        Ok(())
    }

    /// Compile a user-defined function call.
    /// Uses jal/j ra with parameters starting at r10.
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
            .ok_or_else(|| Error::UndefinedFunction(name.clone()))?;

        let mut arg_vals: Vec<Operand> = Vec::new();
        for arg in arguments {
            arg_vals.push(self.compile_expr(arg)?);
        }

        // move arguments to function parameter registers (r10, r11, ...)
        for (i, arg_val) in arg_vals.iter().enumerate() {
            let param_reg = FUNC_REG_BASE + i as u8;
            self.emit_move(param_reg, arg_val);
        }

        let label = format!("fn_{}", name);
        self.emit(format!("jal {}", label));

        // compile the function body once (deferred, emitted after main)
        if !self.compiled_functions.contains(&name) {
            self.compiled_functions.insert(name.clone());
            self.compile_function_body(&name, &label, &func)?;
        }

        Ok(())
    }

    /// Compile a function body into deferred_lines. Uses r10+ for params/locals.
    fn compile_function_body(
        &mut self,
        _name: &str,
        label: &str,
        func: &FunctionDef,
    ) -> Result<()> {
        // save caller state so the function gets its own scope
        let saved_vars = std::mem::take(&mut self.variables);
        let saved_next_var_reg = self.next_var_reg;
        let saved_lines = std::mem::take(&mut self.lines);
        let saved_loop_stack = std::mem::take(&mut self.loop_stack);

        self.next_var_reg = FUNC_REG_BASE;
        for param in &func.parameters {
            self.alloc_var(param.to_string());
        }

        self.emit(format!("{}:", label));
        self.compile_block(&func.body)?;
        self.emit("j ra");

        self.deferred_lines.extend(self.lines.drain(..));

        self.variables = saved_vars;
        self.next_var_reg = saved_next_var_reg;
        self.lines = saved_lines;
        self.loop_stack = saved_loop_stack;

        Ok(())
    }
}

fn check_arg_count(name: &str, expected: usize, got: usize) -> Result<()> {
    if got != expected {
        Err(Error::WrongArgCount(name.to_string(), expected, got))
    } else {
        Ok(())
    }
}

fn resolve_batch_mode(mode: &Identifier) -> Result<u8> {
    let s: &str = mode.as_ref();
    match s {
        "average" | "Average" => Ok(0),
        "sum" | "Sum" => Ok(1),
        "min" | "Minimum" => Ok(2),
        "max" | "Maximum" => Ok(3),
        _ => Err(Error::InvalidBatchMode(s.to_string())),
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
