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

impl From<DeviceIo> for Instruction {
    fn from(device_io: DeviceIo) -> Self {
        Instruction::DeviceIo(device_io)
    }
}

impl From<FlowControl> for Instruction {
    fn from(flow_control: FlowControl) -> Self {
        Instruction::FlowControl(flow_control)
    }
}

impl From<VariableSelection> for Instruction {
    fn from(variable_selection: VariableSelection) -> Self {
        Instruction::VariableSelection(variable_selection)
    }
}

impl From<Arithmetic> for Instruction {
    fn from(arithmetic: Arithmetic) -> Self {
        Instruction::Arithmetic(arithmetic)
    }
}

impl From<Logic> for Instruction {
    fn from(logic: Logic) -> Self {
        Instruction::Logic(logic)
    }
}

impl From<Stack> for Instruction {
    fn from(stack: Stack) -> Self {
        Instruction::Stack(stack)
    }
}

impl From<Misc> for Instruction {
    fn from(misc: Misc) -> Self {
        Instruction::Misc(misc)
    }
}
