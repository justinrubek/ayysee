use crate::{
    error::Error,
    types::{
        BatchMode, Device, DeviceVariable, NameHash, Reagent, ReagentMode, Register,
        RegisterOrNumber, Slot, SlotLogicType, TypeHash,
    },
};

/// Instructions for interacting with devices.
pub enum DeviceIo {
    /// Branch to line a if device d isn't set.
    ///
    /// bdns d? a(r?|num)
    BranchDeviceNotSet {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Jump execution to line a and store next line number in ra if device is not set.
    ///
    /// bdnsal d? a(r?|num)
    BranchDeviceNotSetAndLink {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Branch to line a if device d is set.
    ///
    /// bdse d? a(r?|num)
    BranchDeviceSet {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Jump execution to line a and store next line number in ra if device is set.
    ///
    /// bdseal d? a(r?|num)
    BranchDeviceSetAndLink {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Relative jump to line a if device is not set.
    ///
    /// brdns d? a(r?|num)
    BranchRelativeDeviceNotSet {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Relative jump to line a if device is set.
    ///
    /// brdse d? a(r?|num)
    BranchRelativeDeviceSet {
        device: Device,
        line: RegisterOrNumber,
    },
    /// Branch to line a if device is invalid for load instruction.
    ///
    /// bdnvl d? logicType a(r?|num)
    BranchDeviceNotValidLoad {
        device: Device,
        variable: DeviceVariable,
        line: RegisterOrNumber,
    },
    /// Branch to line a if device is invalid for store instruction.
    ///
    /// bdnvs d? logicType a(r?|num)
    BranchDeviceNotValidStore {
        device: Device,
        variable: DeviceVariable,
        line: RegisterOrNumber,
    },
    /// Loads device var into register.
    ///
    /// l r? d? var
    LoadDeviceVariable {
        register: Register,
        device: Device,
        variable: DeviceVariable,
    },
    /// Loads var from all output network devices with the provided type hash
    /// using the provided batch mode.
    ///
    /// lb r? type var batchMode
    LoadBatch {
        register: Register,
        type_hash: TypeHash,
        variable: DeviceVariable,
        batch_mode: BatchMode,
    },
    /// Loads var from network devices matching both type and name hashes.
    ///
    /// lbn r? deviceHash nameHash logicType batchMode
    LoadBatchName {
        register: Register,
        type_hash: TypeHash,
        name_hash: NameHash,
        variable: DeviceVariable,
        batch_mode: BatchMode,
    },
    /// Loads slot logic type from slot on network devices matching type and name hashes.
    ///
    /// lbns r? deviceHash nameHash slotIndex logicSlotType batchMode
    LoadBatchNameSlot {
        register: Register,
        type_hash: TypeHash,
        name_hash: NameHash,
        slot: Slot,
        slot_variable: SlotLogicType,
        batch_mode: BatchMode,
    },
    /// Loads slot logic type from slot on network devices matching type hash.
    ///
    /// lbs r? deviceHash slotIndex logicSlotType batchMode
    LoadBatchSlot {
        register: Register,
        type_hash: TypeHash,
        slot: Slot,
        slot_variable: SlotLogicType,
        batch_mode: BatchMode,
    },
    /// Loads reagent of device's reagentMode to register.
    ///
    /// lr r? d? reagentMode reagent
    LoadReagent {
        register: Register,
        device: Device,
        reagent_mode: ReagentMode,
        reagent: Reagent,
    },
    /// Loads slot var on device to register.
    ///
    /// ls r? d? int var
    LoadSlot {
        register: Register,
        device: Device,
        slot: Slot,
        variable: DeviceVariable,
    },
    /// Stores prefab hash corresponding to a reagent requirement.
    ///
    /// rmap r? d? reagentHash(r?|num)
    ReagentMap {
        register: Register,
        device: Device,
        reagent_hash: RegisterOrNumber,
    },
    /// Stores register to var on device.
    ///
    /// s d? var r?
    StoreDeviceVariable {
        device: Device,
        variable: DeviceVariable,
        register: Register,
    },
    /// Stores register value to var on all output network devices with the provided type hash.
    ///
    /// sb type var r?
    StoreBatch {
        type_hash: TypeHash,
        variable: DeviceVariable,
        register: Register,
    },
    /// Stores register value to var on network devices matching type and name hashes.
    ///
    /// sbn deviceHash nameHash logicType r?
    StoreBatchName {
        type_hash: TypeHash,
        name_hash: NameHash,
        variable: DeviceVariable,
        register: Register,
    },
    /// Stores register value to slot logic type on devices matching type hash.
    ///
    /// sbs deviceHash slotIndex logicSlotType r?
    StoreBatchSlot {
        type_hash: TypeHash,
        slot: Slot,
        slot_variable: SlotLogicType,
        register: Register,
    },
    /// Stores register value to device slot's LogicSlotType.
    ///
    /// ss d? slotIndex logicSlotType r?
    StoreSlot {
        device: Device,
        slot: Slot,
        slot_variable: SlotLogicType,
        register: Register,
    },
}

impl std::str::FromStr for DeviceIo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let command = parts
            .next()
            .ok_or_else(|| Error::ParseError(s.to_string()))?;

        match command {
            "bdns" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchDeviceNotSet { device, line })
            }
            "bdnsal" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchDeviceNotSetAndLink { device, line })
            }
            "bdse" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchDeviceSet { device, line })
            }
            "bdseal" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchDeviceSetAndLink { device, line })
            }
            "brdns" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchRelativeDeviceNotSet { device, line })
            }
            "brdse" => {
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let line = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::BranchRelativeDeviceSet { device, line })
            }
            "l" => {
                let register = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let device = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;
                let variable = parts
                    .next()
                    .ok_or_else(|| Error::ParseError(s.to_string()))?
                    .parse()?;

                Ok(DeviceIo::LoadDeviceVariable {
                    register,
                    device,
                    variable,
                })
            }
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

impl std::fmt::Display for DeviceIo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceIo::BranchDeviceNotSet { device, line } => {
                write!(f, "bdns {} {}", device, line)
            }
            DeviceIo::BranchDeviceNotSetAndLink { device, line } => {
                write!(f, "bdnsal {} {}", device, line)
            }
            DeviceIo::BranchDeviceSet { device, line } => write!(f, "bdse {} {}", device, line),
            DeviceIo::BranchDeviceSetAndLink { device, line } => {
                write!(f, "bdseal {} {}", device, line)
            }
            DeviceIo::BranchRelativeDeviceNotSet { device, line } => {
                write!(f, "brdns {} {}", device, line)
            }
            DeviceIo::BranchRelativeDeviceSet { device, line } => {
                write!(f, "brdse {} {}", device, line)
            }
            DeviceIo::BranchDeviceNotValidLoad {
                device,
                variable,
                line,
            } => write!(f, "bdnvl {} {} {}", device, variable, line),
            DeviceIo::BranchDeviceNotValidStore {
                device,
                variable,
                line,
            } => write!(f, "bdnvs {} {} {}", device, variable, line),
            DeviceIo::LoadDeviceVariable {
                register,
                device,
                variable,
            } => write!(f, "l {} {} {}", register, device, variable),
            DeviceIo::LoadBatch {
                register,
                type_hash,
                variable,
                batch_mode,
            } => write!(
                f,
                "lb {} {} {} {}",
                register, type_hash, variable, batch_mode
            ),
            DeviceIo::LoadBatchName {
                register,
                type_hash,
                name_hash,
                variable,
                batch_mode,
            } => write!(
                f,
                "lbn {} {} {} {} {}",
                register, type_hash, name_hash, variable, batch_mode
            ),
            DeviceIo::LoadBatchNameSlot {
                register,
                type_hash,
                name_hash,
                slot,
                slot_variable,
                batch_mode,
            } => write!(
                f,
                "lbns {} {} {} {} {} {}",
                register, type_hash, name_hash, slot, slot_variable, batch_mode
            ),
            DeviceIo::LoadBatchSlot {
                register,
                type_hash,
                slot,
                slot_variable,
                batch_mode,
            } => write!(
                f,
                "lbs {} {} {} {} {}",
                register, type_hash, slot, slot_variable, batch_mode
            ),
            DeviceIo::LoadReagent {
                register,
                device,
                reagent_mode,
                reagent,
            } => write!(f, "lr {} {} {} {}", register, device, reagent_mode, reagent),
            DeviceIo::LoadSlot {
                register,
                device,
                slot,
                variable,
            } => write!(f, "ls {} {} {} {}", register, device, slot, variable),
            DeviceIo::ReagentMap {
                register,
                device,
                reagent_hash,
            } => write!(f, "rmap {} {} {}", register, device, reagent_hash),
            DeviceIo::StoreDeviceVariable {
                device,
                variable,
                register,
            } => write!(f, "s {} {} {}", device, variable, register),
            DeviceIo::StoreBatch {
                type_hash,
                variable,
                register,
            } => write!(f, "sb {} {} {}", type_hash, variable, register),
            DeviceIo::StoreBatchName {
                type_hash,
                name_hash,
                variable,
                register,
            } => write!(
                f,
                "sbn {} {} {} {}",
                type_hash, name_hash, variable, register
            ),
            DeviceIo::StoreBatchSlot {
                type_hash,
                slot,
                slot_variable,
                register,
            } => write!(
                f,
                "sbs {} {} {} {}",
                type_hash, slot, slot_variable, register
            ),
            DeviceIo::StoreSlot {
                device,
                slot,
                slot_variable,
                register,
            } => write!(f, "ss {} {} {} {}", device, slot, slot_variable, register),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{DeviceIo, Instruction},
        types::{Device, Number, RegisterOrNumber},
    };

    #[test]
    fn serde_device_io_bdns() {
        let instruction = Instruction::DeviceIo(DeviceIo::BranchDeviceNotSet {
            device: Device::D0,
            line: RegisterOrNumber::Number(Number::Int(5)),
        });

        let instruction_str = format!("{}", instruction);
        println!("{}", instruction_str);

        assert_eq!(
            instruction_str, "bdns d0 5",
            "Instruction string does not match expected"
        );
    }
}
