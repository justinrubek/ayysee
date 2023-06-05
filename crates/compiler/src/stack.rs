use crate::{
    codegen::CodeGenerator,
    util::{stack_pop, stack_push},
    Location,
};
use stationeers_mips::{instructions::Stack as StackInstruction, types::Register};
use std::collections::HashMap;

/// Utility struct for managing the stack.
pub(crate) struct Stack {
    rsp_offset: i32,

    pub(crate) locals: HashMap<String, Location>,
    saved_registers: Vec<Register>,
    /// Keeps track of the loops that are currently active.
    loops: Vec<String>,

    loop_counter: i32,
    if_counter: i32,
}

impl Stack {
    pub(crate) fn new() -> Self {
        Self {
            rsp_offset: 0,
            locals: HashMap::new(),
            saved_registers: Vec::new(),
            loops: Vec::new(),
            loop_counter: 0,
            if_counter: 0,
        }
    }

    /// Allocates space on the stack for a local variable.
    /// The variable will be initialized to 0.
    pub(crate) fn allocate_local(&mut self, name: String) {
        self.rsp_offset += 1;
        self.locals.insert(name, Location::Stack(self.rsp_offset));
    }

    /// Makes the stack aware of a local variable that has already been allocated.
    /// This will not allocate any space on the stack but will allow the stack to
    /// reference the variable.
    pub(crate) fn allocate_local_at(&mut self, name: String, location: Location) {
        match location {
            Location::Stack(offset) => self
                .locals
                .insert(name, Location::Stack(self.rsp_offset + offset)),
            Location::Register(register) => self.locals.insert(name, Location::Register(register)),
        };
    }

    /// Deallocates a local variable.
    pub(crate) fn deallocate_local(&mut self, name: String) {
        self.rsp_offset -= 1;
        self.locals.remove(&name);
    }

    /// Allocates space on the stack for a saved register.
    pub(crate) fn save_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset += 1;
        self.saved_registers.push(register);
        stack_push!(codegen, register);
    }

    /// Deallocates a saved register and restores its value.
    pub(crate) fn restore_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset -= 1;
        self.saved_registers.pop();
        stack_pop!(codegen, register);
    }

    /// Marks the beginning of a loop.
    pub(crate) fn new_loop(&mut self) -> String {
        let name = format!("loop_{}", self.loop_counter);
        self.loop_counter += 1;
        self.loops.push(name.clone());

        name
    }

    pub(crate) fn new_if(&mut self) -> String {
        let name = format!("if_{}", self.if_counter);
        self.if_counter += 1;

        name
    }

    /// Marks the end of a loop.
    pub(crate) fn end_loop(&mut self) -> Option<String> {
        self.loops.pop()
    }

    /// Clears values between passes.
    pub(crate) fn clear(&mut self) {
        self.loop_counter = 0;
        self.if_counter = 0;
    }
}
