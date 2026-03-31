use crate::types::{Device, Register, RegisterOrNumber};

/// Instructions for operating on the stack.
pub enum Stack {
    /// Clears the stack memory of the provided device.
    ///
    /// clr d?
    Clear { device: Device },
    /// Seeks device by id and clears its stack memory.
    ///
    /// clrd id(r?|num)
    ClearById { id: RegisterOrNumber },
    /// Reads stack value at address from device into register.
    ///
    /// get r? d? address(r?|num)
    Get {
        register: Register,
        device: Device,
        address: RegisterOrNumber,
    },
    /// Reads stack value at address from device id into register.
    ///
    /// getd r? id(r?|num) address(r?|num)
    GetById {
        register: Register,
        id: RegisterOrNumber,
        address: RegisterOrNumber,
    },
    /// Register = top of stack (does not pop).
    ///
    /// peek r?
    Peek { register: Register },
    /// Stores value at address in the IC's own stack.
    ///
    /// poke address(r?|num) value(r?|num)
    Poke {
        address: RegisterOrNumber,
        value: RegisterOrNumber,
    },
    /// Register = top of stack, then pop (decrement sp).
    ///
    /// pop r?
    Pop { register: Register },
    /// Push a onto the stack (increment sp).
    ///
    /// push a(r?|num)
    Push { a: RegisterOrNumber },
    /// Writes value to device stack at address.
    ///
    /// put d? address(r?|num) value(r?|num)
    Put {
        device: Device,
        address: RegisterOrNumber,
        value: RegisterOrNumber,
    },
    /// Writes value to device id stack at address.
    ///
    /// putd id(r?|num) address(r?|num) value(r?|num)
    PutById {
        id: RegisterOrNumber,
        address: RegisterOrNumber,
        value: RegisterOrNumber,
    },
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stack::Clear { device } => write!(f, "clr {device}"),
            Stack::ClearById { id } => write!(f, "clrd {id}"),
            Stack::Get {
                register,
                device,
                address,
            } => write!(f, "get {register} {device} {address}"),
            Stack::GetById {
                register,
                id,
                address,
            } => write!(f, "getd {register} {id} {address}"),
            Stack::Peek { register } => write!(f, "peek {register}"),
            Stack::Poke { address, value } => write!(f, "poke {address} {value}"),
            Stack::Pop { register } => write!(f, "pop {register}"),
            Stack::Push { a } => write!(f, "push {a}"),
            Stack::Put {
                device,
                address,
                value,
            } => write!(f, "put {device} {address} {value}"),
            Stack::PutById { id, address, value } => write!(f, "putd {id} {address} {value}"),
        }
    }
}
