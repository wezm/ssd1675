use interface::DisplayInterface;


/// The address increment orientation when writing image data. This configures how the controller will
/// auto-increment the row and column addresses when image data is written using the
/// `WriteImageData` command.
#[derive(Clone, Copy)]
pub enum IncrementAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
enum DataEntryMode {
    DecrementXDecrementY,
    IncrementXDecrementY,
    DecrementXIncrementY,
    IncrementYIncrementX, // POR
}

#[derive(Clone, Copy)]
enum TemperatureSensor {
    Internal,
    External,
}

#[derive(Clone, Copy)]
enum RamOption {
    Normal,
    Bypass,
    Invert,
}

pub enum Command {
    /// Set the MUX of gate lines, scanning sequence and direction
    /// 0: MAX gate lines
    /// 1: Gate scanning sequence and direction
    DriverOutputControl(u16, u8),
    /// Set the gate driving voltage.
    GateDrivingVoltage(u8),
    /// Set the source driving voltage.
    /// 0: VHS1/VSH2
    /// 1: VSL
    SourceDrivingVoltage(u16, u8),
    /// Booster enable with phases 1 to 3 for soft start current and duration setting
    /// 0: Soft start setting for phase 1
    /// 1: Soft start setting for phase 2
    /// 2: Soft start setting for phase 3
    /// 3: Duration setting
    BoosterEnable(u8, u8, u8, u8),
    /// Set the scanning start position of the gate driver
    GateScanStartPostion(u16),
    /// Set deep sleep mode
    DeepSleepMode(u8),
    /// Set the data entry mode and increament axis
    DataEntryMode(DataEntryMode, IncrementAxis),
    /// Perform a soft reset, and reset all parameters to their default values
    /// BUSY will be high when in progress.
    SoftReset,
    // /// Start HV ready detection. Read result with `ReadStatusBit` command
    // StartHVReadyDetection,
    // /// Start VCI level detection
    // /// 0: threshold
    // /// Read result with `ReadStatusBit` command
    // StartVCILevelDetection(u8),
    /// Specify internal or external temperature sensor
    TemperatatSensorSelection(TemperatureSensor),
    /// Write to the temperature sensor register
    WriteTemperatureSensor(u16),
    /// Read from the temperature sensor register
    ReadTemperatureSensor(u16),
    /// Write a command to the external temperature sensor
    WriteExternalTemperatureSensor(u8, u8, u8),
    /// Activate dispay update sequence. BUSY will be high when in progress.
    UpdateDisplay,
    /// Set RAM content options for update display command.
    /// 0: Black/White RAM option
    /// 1: Red RAM option
    UpdateDisplayOption1(RamOption, RamOption),
    /// Set display update sequence options
    UpdateDisplayOption2(u8),
    // Read from RAM (not implemented)
    // ReadData,
    /// Enter VCOM sensing and hold for duration defined by VCOMSenseDuration
    /// BUSY will be high when in progress.
    EnterVCOMSensing,
    /// Set VCOM sensing duration
    VCOMSenseDuration(u8),
    // /// Program VCOM register into OTP
    // ProgramVCOMIntoOTP,
    /// Write VCOM register from MCU interface
    WriteVCOM(u8),
    // ReadDisplayOption,
    // ReadUserId,
    // StatusBitRead,
    // ProgramWaveformSetting,
    // LoadWaveformSetting,
    // CalculateCRC,
    // ReadCRC,
    // ProgramOTP,
    // WriteDisplayOption,
    // WriteUserId,
    // OTPProgramMode,
    /// Set the number dummy line period in terms of gate line width (TGate)
    DummyLinePeriod(u8),
    /// Set the gate line width (TGate)
    GateLineWidth(u8),
    /// Select border waveform for VBD
    BorderWaveform(u8),
    // ReadRamOption,
    /// Set the start/end positions of the window address in the X direction
    /// 0: Start
    /// 1: End
    StartEndXPosition(u8, u8),
    /// Set the start/end positions of the window address in the Y direction
    /// 0: Start
    /// 1: End
    StartEndYPosition(u8, u8),
    /// Auto write red RAM for regular pattern
    AutoWriteRedPattern(u8),
    /// Auto write red RAM for regular pattern
    AutoWriteBlackPattern(u8),
    /// Set RAM X address
    XAddress(u8),
    /// Set RAM Y address
    YAddress(u8),
    /// Set analog block control
    AnalogBlockControl(u8),
    /// Set digital block control
    DigitalBlockControl(u8),
    // Used to terminate frame memory reads
    // Nop,
}

/// Enumerates commands that can be sent to the controller that accept a slice argument buffer. This
/// is separated from `Command` so that the lifetime parameter of the argument buffer slice does
/// not pervade code which never invokes these two commands.
pub enum BufCommand<'buf> {
    /// Write to black/white RAM
    /// 1 = White
    /// 0 = Black
    WriteBlackData(&'buf [u8]),
    /// Write to red RAM
    /// 1 = Red
    /// 0 = Use contents of black/white RAM
    WriteRedData(&'buf [u8]),
    /// Write LUT register (70 bytes)
    WriteLUT(&'buf [u8]),
}
