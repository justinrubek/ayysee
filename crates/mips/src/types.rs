use crate::error::Error;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Device {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    Db,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Device::D0 => write!(f, "d0"),
            Device::D1 => write!(f, "d1"),
            Device::D2 => write!(f, "d2"),
            Device::D3 => write!(f, "d3"),
            Device::D4 => write!(f, "d4"),
            Device::D5 => write!(f, "d5"),
            Device::Db => write!(f, "db"),
        }
    }
}

impl std::str::FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d0" => Ok(Device::D0),
            "d1" => Ok(Device::D1),
            "d2" => Ok(Device::D2),
            "d3" => Ok(Device::D3),
            "d4" => Ok(Device::D4),
            "d5" => Ok(Device::D5),
            "db" => Ok(Device::Db),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    Ra,
    Sp,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Register::R0 => write!(f, "r0"),
            Register::R1 => write!(f, "r1"),
            Register::R2 => write!(f, "r2"),
            Register::R3 => write!(f, "r3"),
            Register::R4 => write!(f, "r4"),
            Register::R5 => write!(f, "r5"),
            Register::R6 => write!(f, "r6"),
            Register::R7 => write!(f, "r7"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::R11 => write!(f, "r11"),
            Register::R12 => write!(f, "r12"),
            Register::R13 => write!(f, "r13"),
            Register::R14 => write!(f, "r14"),
            Register::R15 => write!(f, "r15"),
            Register::R16 => write!(f, "r16"),
            Register::R17 => write!(f, "r17"),
            Register::Ra => write!(f, "ra"),
            Register::Sp => write!(f, "sp"),
        }
    }
}

impl std::str::FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r0" => Ok(Register::R0),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            "r4" => Ok(Register::R4),
            "r5" => Ok(Register::R5),
            "r6" => Ok(Register::R6),
            "r7" => Ok(Register::R7),
            "r8" => Ok(Register::R8),
            "r9" => Ok(Register::R9),
            "r10" => Ok(Register::R10),
            "r11" => Ok(Register::R11),
            "r12" => Ok(Register::R12),
            "r13" => Ok(Register::R13),
            "r14" => Ok(Register::R14),
            "r15" => Ok(Register::R15),
            "r16" => Ok(Register::R16),
            "r17" => Ok(Register::R17),
            "ra" => Ok(Register::Ra),
            "sp" => Ok(Register::Sp),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            16 => Register::R16,
            17 => Register::R17,
            _ => panic!("Invalid register value: {}", value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Int(int) => write!(f, "{}", int),
            Number::Float(float) => write!(f, "{}", float),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<i32>() {
            Ok(Number::Int(int))
        } else if let Ok(float) = s.parse::<f32>() {
            Ok(Number::Float(float))
        } else {
            Err(Error::ParseError(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub enum RegisterOrNumber {
    Register(Register),
    Number(Number),
}

impl std::fmt::Display for RegisterOrNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegisterOrNumber::Register(register) => write!(f, "{}", register),
            RegisterOrNumber::Number(number) => write!(f, "{}", number),
        }
    }
}

impl std::str::FromStr for RegisterOrNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(register) = s.parse::<Register>() {
            Ok(RegisterOrNumber::Register(register))
        } else if let Ok(number) = s.parse::<Number>() {
            Ok(RegisterOrNumber::Number(number))
        } else {
            Err(Error::ParseError(s.to_string()))
        }
    }
}

impl From<Register> for RegisterOrNumber {
    fn from(register: Register) -> Self {
        RegisterOrNumber::Register(register)
    }
}

impl From<Number> for RegisterOrNumber {
    fn from(number: Number) -> Self {
        RegisterOrNumber::Number(number)
    }
}

/// Represents a device logic type (property that can be read/written on devices).
///
/// Note: The game continues to add new logic types. This list covers common types
/// but may not be exhaustive for all devices. Unknown types can still be used
/// via the compiler's string-based code emission.
#[derive(Clone, Debug)]
pub enum DeviceVariable {
    Activate,
    AirRelease,
    Charge,
    ClearMemory,
    Color,
    CompletionRatio,
    CurrentResearchPodType,
    ElevatorLevel,
    ElevatorSpeed,
    Error,
    ExportCount,
    Filtration,
    ForceWrite,
    Harvest,
    Horizontal,
    HorizontalRatio,
    Idle,
    ImportCount,
    LineNumber,
    Lock,
    Maximum,
    MinimumWattsToContact,
    Mode,
    On,
    Open,
    OperationalTemperatureEfficiency,
    Output,
    Plant,
    PositionX,
    PositionY,
    PositionZ,
    Power,
    PowerActual,
    PowerGeneration,
    PowerPotential,
    PowerRequired,
    Pressure,
    PressureExternal,
    PressureInput,
    PressureInternal,
    PressureOutput,
    PressureSetting,
    Quantity,
    Ratio,
    RatioCarbonDioxide,
    RatioLiquidCarbonDioxide,
    RatioLiquidNitrogen,
    RatioLiquidNitrousOxide,
    RatioLiquidOxygen,
    RatioLiquidPollutant,
    RatioLiquidVolatiles,
    RatioNitrogen,
    RatioNitrousOxide,
    RatioOxygen,
    RatioPollutant,
    RatioSteam,
    RatioVolatiles,
    RatioWater,
    Reagents,
    RecipeHash,
    ReferenceId,
    RequestHash,
    RequiredPower,
    ReturnFuelCost,
    Setting,
    SignalId,
    SignalStrength,
    SolarAngle,
    SolarIrradiance,
    SoundAlert,
    Stress,
    Temperature,
    TemperatureExternal,
    TemperatureInput,
    TemperatureOutput,
    TemperatureSetting,
    TotalMoles,
    TotalMolesInput,
    TotalMolesOutput,
    VelocityMagnitude,
    VelocityRelativeX,
    VelocityRelativeY,
    VelocityRelativeZ,
    Vertical,
    VerticalRatio,
    Volume,
    WattsReachingContact,
}

impl std::str::FromStr for DeviceVariable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Activate" => Ok(DeviceVariable::Activate),
            "AirRelease" => Ok(DeviceVariable::AirRelease),
            "Charge" => Ok(DeviceVariable::Charge),
            "ClearMemory" => Ok(DeviceVariable::ClearMemory),
            "Color" => Ok(DeviceVariable::Color),
            "CompletionRatio" => Ok(DeviceVariable::CompletionRatio),
            "CurrentResearchPodType" => Ok(DeviceVariable::CurrentResearchPodType),
            "ElevatorLevel" => Ok(DeviceVariable::ElevatorLevel),
            "ElevatorSpeed" => Ok(DeviceVariable::ElevatorSpeed),
            "Error" => Ok(DeviceVariable::Error),
            "ExportCount" => Ok(DeviceVariable::ExportCount),
            "Filtration" => Ok(DeviceVariable::Filtration),
            "ForceWrite" => Ok(DeviceVariable::ForceWrite),
            "Harvest" => Ok(DeviceVariable::Harvest),
            "Horizontal" => Ok(DeviceVariable::Horizontal),
            "HorizontalRatio" => Ok(DeviceVariable::HorizontalRatio),
            "Idle" => Ok(DeviceVariable::Idle),
            "ImportCount" => Ok(DeviceVariable::ImportCount),
            "LineNumber" => Ok(DeviceVariable::LineNumber),
            "Lock" => Ok(DeviceVariable::Lock),
            "Maximum" => Ok(DeviceVariable::Maximum),
            "MinimumWattsToContact" => Ok(DeviceVariable::MinimumWattsToContact),
            "Mode" => Ok(DeviceVariable::Mode),
            "On" => Ok(DeviceVariable::On),
            "Open" => Ok(DeviceVariable::Open),
            "OperationalTemperatureEfficiency" => {
                Ok(DeviceVariable::OperationalTemperatureEfficiency)
            }
            "Output" => Ok(DeviceVariable::Output),
            "Plant" => Ok(DeviceVariable::Plant),
            "PositionX" => Ok(DeviceVariable::PositionX),
            "PositionY" => Ok(DeviceVariable::PositionY),
            "PositionZ" => Ok(DeviceVariable::PositionZ),
            "Power" => Ok(DeviceVariable::Power),
            "PowerActual" => Ok(DeviceVariable::PowerActual),
            "PowerGeneration" => Ok(DeviceVariable::PowerGeneration),
            "PowerPotential" => Ok(DeviceVariable::PowerPotential),
            "PowerRequired" => Ok(DeviceVariable::PowerRequired),
            "Pressure" => Ok(DeviceVariable::Pressure),
            "PressureExternal" => Ok(DeviceVariable::PressureExternal),
            "PressureInput" => Ok(DeviceVariable::PressureInput),
            "PressureInternal" => Ok(DeviceVariable::PressureInternal),
            "PressureOutput" => Ok(DeviceVariable::PressureOutput),
            "PressureSetting" => Ok(DeviceVariable::PressureSetting),
            "Quantity" => Ok(DeviceVariable::Quantity),
            "Ratio" => Ok(DeviceVariable::Ratio),
            "RatioCarbonDioxide" => Ok(DeviceVariable::RatioCarbonDioxide),
            "RatioLiquidCarbonDioxide" => Ok(DeviceVariable::RatioLiquidCarbonDioxide),
            "RatioLiquidNitrogen" => Ok(DeviceVariable::RatioLiquidNitrogen),
            "RatioLiquidNitrousOxide" => Ok(DeviceVariable::RatioLiquidNitrousOxide),
            "RatioLiquidOxygen" => Ok(DeviceVariable::RatioLiquidOxygen),
            "RatioLiquidPollutant" => Ok(DeviceVariable::RatioLiquidPollutant),
            "RatioLiquidVolatiles" => Ok(DeviceVariable::RatioLiquidVolatiles),
            "RatioNitrogen" => Ok(DeviceVariable::RatioNitrogen),
            "RatioNitrousOxide" => Ok(DeviceVariable::RatioNitrousOxide),
            "RatioOxygen" => Ok(DeviceVariable::RatioOxygen),
            "RatioPollutant" => Ok(DeviceVariable::RatioPollutant),
            "RatioSteam" => Ok(DeviceVariable::RatioSteam),
            "RatioVolatiles" => Ok(DeviceVariable::RatioVolatiles),
            "RatioWater" => Ok(DeviceVariable::RatioWater),
            "Reagents" => Ok(DeviceVariable::Reagents),
            "RecipeHash" => Ok(DeviceVariable::RecipeHash),
            "ReferenceId" => Ok(DeviceVariable::ReferenceId),
            "RequestHash" => Ok(DeviceVariable::RequestHash),
            "RequiredPower" => Ok(DeviceVariable::RequiredPower),
            "ReturnFuelCost" => Ok(DeviceVariable::ReturnFuelCost),
            "Setting" => Ok(DeviceVariable::Setting),
            "SignalId" => Ok(DeviceVariable::SignalId),
            "SignalStrength" => Ok(DeviceVariable::SignalStrength),
            "SolarAngle" => Ok(DeviceVariable::SolarAngle),
            "SolarIrradiance" => Ok(DeviceVariable::SolarIrradiance),
            "SoundAlert" => Ok(DeviceVariable::SoundAlert),
            "Stress" => Ok(DeviceVariable::Stress),
            "Temperature" => Ok(DeviceVariable::Temperature),
            "TemperatureExternal" => Ok(DeviceVariable::TemperatureExternal),
            "TemperatureInput" => Ok(DeviceVariable::TemperatureInput),
            "TemperatureOutput" => Ok(DeviceVariable::TemperatureOutput),
            "TemperatureSetting" => Ok(DeviceVariable::TemperatureSetting),
            "TotalMoles" => Ok(DeviceVariable::TotalMoles),
            "TotalMolesInput" => Ok(DeviceVariable::TotalMolesInput),
            "TotalMolesOutput" => Ok(DeviceVariable::TotalMolesOutput),
            "VelocityMagnitude" => Ok(DeviceVariable::VelocityMagnitude),
            "VelocityRelativeX" => Ok(DeviceVariable::VelocityRelativeX),
            "VelocityRelativeY" => Ok(DeviceVariable::VelocityRelativeY),
            "VelocityRelativeZ" => Ok(DeviceVariable::VelocityRelativeZ),
            "Vertical" => Ok(DeviceVariable::Vertical),
            "VerticalRatio" => Ok(DeviceVariable::VerticalRatio),
            "Volume" => Ok(DeviceVariable::Volume),
            "WattsReachingContact" => Ok(DeviceVariable::WattsReachingContact),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

impl std::fmt::Display for DeviceVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceVariable::Activate => write!(f, "Activate"),
            DeviceVariable::AirRelease => write!(f, "AirRelease"),
            DeviceVariable::Charge => write!(f, "Charge"),
            DeviceVariable::ClearMemory => write!(f, "ClearMemory"),
            DeviceVariable::Color => write!(f, "Color"),
            DeviceVariable::CompletionRatio => write!(f, "CompletionRatio"),
            DeviceVariable::CurrentResearchPodType => write!(f, "CurrentResearchPodType"),
            DeviceVariable::ElevatorLevel => write!(f, "ElevatorLevel"),
            DeviceVariable::ElevatorSpeed => write!(f, "ElevatorSpeed"),
            DeviceVariable::Error => write!(f, "Error"),
            DeviceVariable::ExportCount => write!(f, "ExportCount"),
            DeviceVariable::Filtration => write!(f, "Filtration"),
            DeviceVariable::ForceWrite => write!(f, "ForceWrite"),
            DeviceVariable::Harvest => write!(f, "Harvest"),
            DeviceVariable::Horizontal => write!(f, "Horizontal"),
            DeviceVariable::HorizontalRatio => write!(f, "HorizontalRatio"),
            DeviceVariable::Idle => write!(f, "Idle"),
            DeviceVariable::ImportCount => write!(f, "ImportCount"),
            DeviceVariable::LineNumber => write!(f, "LineNumber"),
            DeviceVariable::Lock => write!(f, "Lock"),
            DeviceVariable::Maximum => write!(f, "Maximum"),
            DeviceVariable::MinimumWattsToContact => write!(f, "MinimumWattsToContact"),
            DeviceVariable::Mode => write!(f, "Mode"),
            DeviceVariable::On => write!(f, "On"),
            DeviceVariable::Open => write!(f, "Open"),
            DeviceVariable::OperationalTemperatureEfficiency => {
                write!(f, "OperationalTemperatureEfficiency")
            }
            DeviceVariable::Output => write!(f, "Output"),
            DeviceVariable::Plant => write!(f, "Plant"),
            DeviceVariable::PositionX => write!(f, "PositionX"),
            DeviceVariable::PositionY => write!(f, "PositionY"),
            DeviceVariable::PositionZ => write!(f, "PositionZ"),
            DeviceVariable::Power => write!(f, "Power"),
            DeviceVariable::PowerActual => write!(f, "PowerActual"),
            DeviceVariable::PowerGeneration => write!(f, "PowerGeneration"),
            DeviceVariable::PowerPotential => write!(f, "PowerPotential"),
            DeviceVariable::PowerRequired => write!(f, "PowerRequired"),
            DeviceVariable::Pressure => write!(f, "Pressure"),
            DeviceVariable::PressureExternal => write!(f, "PressureExternal"),
            DeviceVariable::PressureInput => write!(f, "PressureInput"),
            DeviceVariable::PressureInternal => write!(f, "PressureInternal"),
            DeviceVariable::PressureOutput => write!(f, "PressureOutput"),
            DeviceVariable::PressureSetting => write!(f, "PressureSetting"),
            DeviceVariable::Quantity => write!(f, "Quantity"),
            DeviceVariable::Ratio => write!(f, "Ratio"),
            DeviceVariable::RatioCarbonDioxide => write!(f, "RatioCarbonDioxide"),
            DeviceVariable::RatioLiquidCarbonDioxide => write!(f, "RatioLiquidCarbonDioxide"),
            DeviceVariable::RatioLiquidNitrogen => write!(f, "RatioLiquidNitrogen"),
            DeviceVariable::RatioLiquidNitrousOxide => write!(f, "RatioLiquidNitrousOxide"),
            DeviceVariable::RatioLiquidOxygen => write!(f, "RatioLiquidOxygen"),
            DeviceVariable::RatioLiquidPollutant => write!(f, "RatioLiquidPollutant"),
            DeviceVariable::RatioLiquidVolatiles => write!(f, "RatioLiquidVolatiles"),
            DeviceVariable::RatioNitrogen => write!(f, "RatioNitrogen"),
            DeviceVariable::RatioNitrousOxide => write!(f, "RatioNitrousOxide"),
            DeviceVariable::RatioOxygen => write!(f, "RatioOxygen"),
            DeviceVariable::RatioPollutant => write!(f, "RatioPollutant"),
            DeviceVariable::RatioSteam => write!(f, "RatioSteam"),
            DeviceVariable::RatioVolatiles => write!(f, "RatioVolatiles"),
            DeviceVariable::RatioWater => write!(f, "RatioWater"),
            DeviceVariable::Reagents => write!(f, "Reagents"),
            DeviceVariable::RecipeHash => write!(f, "RecipeHash"),
            DeviceVariable::ReferenceId => write!(f, "ReferenceId"),
            DeviceVariable::RequestHash => write!(f, "RequestHash"),
            DeviceVariable::RequiredPower => write!(f, "RequiredPower"),
            DeviceVariable::ReturnFuelCost => write!(f, "ReturnFuelCost"),
            DeviceVariable::Setting => write!(f, "Setting"),
            DeviceVariable::SignalId => write!(f, "SignalId"),
            DeviceVariable::SignalStrength => write!(f, "SignalStrength"),
            DeviceVariable::SolarAngle => write!(f, "SolarAngle"),
            DeviceVariable::SolarIrradiance => write!(f, "SolarIrradiance"),
            DeviceVariable::SoundAlert => write!(f, "SoundAlert"),
            DeviceVariable::Stress => write!(f, "Stress"),
            DeviceVariable::Temperature => write!(f, "Temperature"),
            DeviceVariable::TemperatureExternal => write!(f, "TemperatureExternal"),
            DeviceVariable::TemperatureInput => write!(f, "TemperatureInput"),
            DeviceVariable::TemperatureOutput => write!(f, "TemperatureOutput"),
            DeviceVariable::TemperatureSetting => write!(f, "TemperatureSetting"),
            DeviceVariable::TotalMoles => write!(f, "TotalMoles"),
            DeviceVariable::TotalMolesInput => write!(f, "TotalMolesInput"),
            DeviceVariable::TotalMolesOutput => write!(f, "TotalMolesOutput"),
            DeviceVariable::VelocityMagnitude => write!(f, "VelocityMagnitude"),
            DeviceVariable::VelocityRelativeX => write!(f, "VelocityRelativeX"),
            DeviceVariable::VelocityRelativeY => write!(f, "VelocityRelativeY"),
            DeviceVariable::VelocityRelativeZ => write!(f, "VelocityRelativeZ"),
            DeviceVariable::Vertical => write!(f, "Vertical"),
            DeviceVariable::VerticalRatio => write!(f, "VerticalRatio"),
            DeviceVariable::Volume => write!(f, "Volume"),
            DeviceVariable::WattsReachingContact => write!(f, "WattsReachingContact"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeHash(String);

impl std::fmt::Display for TypeHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TypeHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TypeHash(s.to_string()))
    }
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum BatchMode {
    Average = 0,
    Sum = 1,
    Minimum = 2,
    Maximum = 3,
}

impl std::fmt::Display for BatchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BatchMode::Average => write!(f, "Average"),
            BatchMode::Sum => write!(f, "Sum"),
            BatchMode::Minimum => write!(f, "Minimum"),
            BatchMode::Maximum => write!(f, "Maximum"),
        }
    }
}

impl std::str::FromStr for BatchMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Average" | "0" => Ok(BatchMode::Average),
            "Sum" | "1" => Ok(BatchMode::Sum),
            "Minimum" | "2" => Ok(BatchMode::Minimum),
            "Maximum" | "3" => Ok(BatchMode::Maximum),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum ReagentMode {
    Contents = 0,
    Required = 1,
    Recipe = 2,
}

impl std::fmt::Display for ReagentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReagentMode::Contents => write!(f, "Contents"),
            ReagentMode::Required => write!(f, "Required"),
            ReagentMode::Recipe => write!(f, "Recipe"),
        }
    }
}

impl std::str::FromStr for ReagentMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Contents" | "0" => Ok(ReagentMode::Contents),
            "Required" | "1" => Ok(ReagentMode::Required),
            "Recipe" | "2" => Ok(ReagentMode::Recipe),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Reagent(String);

impl std::fmt::Display for Reagent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Reagent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Reagent(s.to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct Slot(u8);

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Slot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(v) => Ok(Slot(v)),
            Err(_) => Err(Error::ParseError(s.to_string())),
        }
    }
}

/// A name hash used for batch operations that filter by device name.
#[derive(Clone, Debug)]
pub struct NameHash(String);

impl std::fmt::Display for NameHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NameHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NameHash(s.to_string()))
    }
}

/// Slot logic types for reading/writing device slot properties.
#[derive(Clone, Debug)]
pub enum SlotLogicType {
    Occupied,
    OccupantHash,
    Quantity,
    Damage,
    Charge,
    ChargeMax,
    Class,
    MaxQuantity,
    PrefabHash,
    SortingClass,
    ReferenceId,
}

impl std::fmt::Display for SlotLogicType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SlotLogicType::Occupied => write!(f, "Occupied"),
            SlotLogicType::OccupantHash => write!(f, "OccupantHash"),
            SlotLogicType::Quantity => write!(f, "Quantity"),
            SlotLogicType::Damage => write!(f, "Damage"),
            SlotLogicType::Charge => write!(f, "Charge"),
            SlotLogicType::ChargeMax => write!(f, "ChargeMax"),
            SlotLogicType::Class => write!(f, "Class"),
            SlotLogicType::MaxQuantity => write!(f, "MaxQuantity"),
            SlotLogicType::PrefabHash => write!(f, "PrefabHash"),
            SlotLogicType::SortingClass => write!(f, "SortingClass"),
            SlotLogicType::ReferenceId => write!(f, "ReferenceId"),
        }
    }
}

impl std::str::FromStr for SlotLogicType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Occupied" => Ok(SlotLogicType::Occupied),
            "OccupantHash" => Ok(SlotLogicType::OccupantHash),
            "Quantity" => Ok(SlotLogicType::Quantity),
            "Damage" => Ok(SlotLogicType::Damage),
            "Charge" => Ok(SlotLogicType::Charge),
            "ChargeMax" => Ok(SlotLogicType::ChargeMax),
            "Class" => Ok(SlotLogicType::Class),
            "MaxQuantity" => Ok(SlotLogicType::MaxQuantity),
            "PrefabHash" => Ok(SlotLogicType::PrefabHash),
            "SortingClass" => Ok(SlotLogicType::SortingClass),
            "ReferenceId" => Ok(SlotLogicType::ReferenceId),
            _ => Err(Error::ParseError(s.to_string())),
        }
    }
}
