mod flow;
mod io;
mod logic;
mod math;
mod misc;
mod stack;
mod variable;

pub use flow::FlowControl;
pub use io::DeviceIo;
pub use logic::Logic;
pub use math::Arithmetic;
pub use misc::Misc;
pub use stack::Stack;
pub use variable::VariableSelection;

/// An enum representing all possible Stationeers MIPS instructions.
/// Each variant is a different instruction and corresponds to a single line of MIPS code.
pub enum Instruction {
    DeviceIo(DeviceIo),
    FlowControl(FlowControl),
    VariableSelection(VariableSelection),
    Arithmetic(Arithmetic),
    Logic(Logic),
    Stack(Stack),
    Misc(Misc),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::DeviceIo(device_io) => write!(f, "{}", device_io),
            Instruction::FlowControl(flow_control) => write!(f, "{}", flow_control),
            Instruction::VariableSelection(variable_selection) => {
                write!(f, "{}", variable_selection)
            }
            Instruction::Arithmetic(arithmetic) => write!(f, "{}", arithmetic),
            Instruction::Logic(logic) => write!(f, "{}", logic),
            Instruction::Stack(stack) => write!(f, "{}", stack),
            Instruction::Misc(misc) => write!(f, "{}", misc),
        }
    }
}
