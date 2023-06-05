use crate::error::Result;
use ayysee_parser::ast::{Identifier, Value};
use stationeers_mips::{
    instructions::{Instruction, Misc},
    types::Device,
};
use std::collections::HashMap;

pub(crate) struct CodeGenerator {
    /// the instructions that have been generated
    pub(crate) instructions: Vec<Instruction>,
    /// comments that have been added to specific instruction lines
    comments: HashMap<i32, String>,

    /// 'labels' that have been added to the code to mark a specific line of code for jumping to
    pub(crate) labels: HashMap<String, i32>,

    /// Device aliases mapped to their actual device
    devices: HashMap<Identifier, Device>,

    /// Constants that have been defined
    constants: HashMap<Identifier, Value>,
}

impl CodeGenerator {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
            comments: HashMap::new(),
            labels: HashMap::new(),
            devices: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    /// Adds an instruction to the list of instructions.
    pub(crate) fn add_instruction(
        &mut self,
        instruction: stationeers_mips::instructions::Instruction,
    ) {
        self.instructions.push(instruction);
    }

    /// Adds a comment to a given line.
    pub(crate) fn insert_comment(&mut self, comment: String, line: i32) {
        self.comments.insert(line, comment);
    }

    /// Adds a comment to the last instruction.
    pub(crate) fn add_comment(&mut self, comment: String) {
        self.insert_comment(comment, self.instructions.len() as i32 - 1);
    }

    /// Adds a comment on a separate line.
    pub(crate) fn add_comment_line(&mut self, comment: String) {
        self.add_instruction(Instruction::from(Misc::Comment { comment }));
    }

    /// Creates a new label and adds it to the list of labels.
    pub(crate) fn add_label(&mut self, label: String) {
        // implementation that inserts a label instruction:
        self.add_instruction(Instruction::from(Misc::Label {
            name: label.clone(),
        }));
        self.labels.insert(label, self.instructions.len() as i32);
    }

    /// Checks if a label exists.
    pub(crate) fn has_label(&self, label: &str) -> bool {
        self.labels.contains_key(label)
    }

    /// Gets the address of a label.
    /// This should only be called after a pass has been completed to ensure that
    /// the label exists.
    pub(crate) fn get_label(&self, label: &str) -> Result<i32> {
        self.labels
            .get(label)
            .copied()
            .ok_or_else(|| unreachable!("label {} does not exist", label))
    }

    /// Clears out data from the first pass.
    /// This should be called before the second pass.
    pub(crate) fn clear_first_pass(&mut self) {
        self.comments.clear();
        self.instructions.clear();
    }

    // TODO: Rewrites usages of identifiers to refer to the device.
    // This is intended to be used as a lines of code optimization.
    // In all places where a device is referred to via alias, the alias is replaced with the
    // device's true name.
    // fn overwrite_aliases

    /// Combines all of the instructions into a single string.
    /// This string can be executed by the MIPS emulator.
    pub(crate) fn get_code(&self) -> String {
        // Get the comments as a vector of strings matching the instructions vector in length.
        let mut comments: Vec<Option<String>> = vec![None; self.instructions.len()];
        for (line, comment) in self.comments.iter() {
            comments[*line as usize] = Some(comment.clone());
        }

        self.instructions
            .iter()
            .zip(comments)
            .map(|(instruction, comment)| match comment {
                Some(comment) => format!("{instruction} # {comment}"),
                None => format!("{instruction}"),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Adds an alias for a device.
    pub(crate) fn add_alias(&mut self, alias: Identifier, device: Device) {
        self.devices.insert(alias, device);
    }

    /// Gets the device that a given identifier refers to.
    /// This should only be called after a pass has been completed to ensure that the alias entry
    /// exists.
    pub(crate) fn get_device(&self, identifier: &Identifier) -> Result<Option<Device>> {
        Ok(self.devices.get(identifier).copied())
    }

    /// Adds a constant to the list of constants.
    pub(crate) fn add_constant(&mut self, identifier: Identifier, value: Value) {
        self.constants.insert(identifier, value);
    }

    /// Gets the value of a constant.
    pub(crate) fn get_constant(&self, identifier: &Identifier) -> Option<Value> {
        self.constants.get(identifier).copied()
    }
}
