use core;
use interface::DisplayInterface;

const MAX_GATES: u16 = 296;
const MAX_DUMMY_LINE_PERIOD: u8 = 127;

trait Contains<C>
where
    C: Copy + PartialOrd,
{
    fn contains(&self, item: C) -> bool;
}

/// The address increment orientation when writing image data. This configures how the controller
/// will auto-increment the row and column addresses when image data is written using the
/// `WriteImageData` command.
#[derive(Clone, Copy)]
pub enum IncrementAxis {
    /// X direction
    Horizontal,
    /// Y direction
    Vertical,
}

#[derive(Clone, Copy)]
pub enum DataEntryMode {
    DecrementXDecrementY,
    IncrementXDecrementY,
    DecrementXIncrementY,
    IncrementYIncrementX, // POR
}

#[derive(Clone, Copy)]
pub enum TemperatureSensor {
    Internal,
    External,
}

#[derive(Clone, Copy)]
pub enum RamOption {
    Normal,
    Bypass,
    Invert,
}

#[derive(Clone, Copy)]
pub enum DeepSleepMode {
    /// Not sleeping
    Normal,
    /// Deep sleep with RAM preserved
    PreserveRAM,
    /// Deep sleep RAM not preserved
    DiscardRAM,
}

#[derive(Clone, Copy)]
pub enum Command {
    /// Set the MUX of gate lines, scanning sequence and direction
    /// 0: MAX gate lines
    /// 1: Gate scanning sequence and direction
    DriverOutputControl(u16, u8),
    /// Set the gate driving voltage.
    GateDrivingVoltage(u8),
    /// Set the source driving voltage.
    /// 0: VSH1
    /// 1: VSH2
    /// 2: VSL
    SourceDrivingVoltage(u8, u8, u8),
    /// Booster enable with phases 1 to 3 for soft start current and duration setting
    /// 0: Soft start setting for phase 1
    /// 1: Soft start setting for phase 2
    /// 2: Soft start setting for phase 3
    /// 3: Duration setting
    BoosterEnable(u8, u8, u8, u8),
    /// Set the scanning start position of the gate driver
    GateScanStartPostion(u16),
    /// Set deep sleep mode
    DeepSleepMode(DeepSleepMode),
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
    /// Activate display update sequence. BUSY will be high when in progress.
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
    StartEndYPosition(u16, u16),
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

fn u16_as_u8(val: u16) -> [u8; 2] {
    [(val & 0xFF00 >> 8) as u8, (val & 0xFF) as u8]
}

/// Populates data buffer (array) and returns a pair (tuple) with command and
/// appropriately sized slice into populated buffer.
/// E.g.
///
/// let mut buf = [0u8; 4];
/// let (command, data) = pack!(buf, 0x3C, [0x12, 0x34]);
macro_rules! pack {
    ($buf:ident, $cmd:expr,[]) => {
        ($cmd, &$buf[..0])
    };
    ($buf:ident, $cmd:expr,[$arg0:expr]) => {{
        $buf[0] = $arg0;
        ($cmd, &$buf[..1])
    }};
    ($buf:ident, $cmd:expr,[$arg0:expr, $arg1:expr]) => {{
        $buf[0] = $arg0;
        $buf[1] = $arg1;
        ($cmd, &$buf[..2])
    }};
    ($buf:ident, $cmd:expr,[$arg0:expr, $arg1:expr, $arg2:expr]) => {{
        $buf[0] = $arg0;
        $buf[1] = $arg1;
        $buf[2] = $arg2;
        ($cmd, &$buf[..3])
    }};
    ($buf:ident, $cmd:expr,[$arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr]) => {{
        $buf[0] = $arg0;
        $buf[1] = $arg1;
        $buf[2] = $arg2;
        $buf[3] = $arg3;
        ($cmd, &$buf[..4])
    }};
}

impl Command {
    pub(crate) fn execute<I: DisplayInterface>(&self, interface: &mut I) -> Result<(), I::Error> {
        use self::Command::*;

        let mut buf = [0u8; 4];
        let (command, data) = match *self {
            DriverOutputControl(gate_lines, scanning_seq_and_dir) => {
                let [upper, lower] = u16_as_u8(gate_lines);
                pack!(buf, 0x01, [lower, upper, scanning_seq_and_dir])
            }
            GateDrivingVoltage(voltages) => pack!(buf, 0x03, [voltages]),
            SourceDrivingVoltage(vsh1, vsh2, vsl) => pack!(buf, 0x04, [vsh1, vsh2, vsl]),
            BoosterEnable(phase1, phase2, phase3, duration) => {
                pack!(buf, 0x0C, [phase1, phase2, phase3, duration])
            }
            GateScanStartPostion(position) => {
                debug_assert!(Contains::contains(&(0..MAX_GATES), position));
                let [upper, lower] = u16_as_u8(position);
                pack!(buf, 0x0F, [lower, upper])
            }
            DeepSleepMode(mode) => {
                let mode = match mode {
                    self::DeepSleepMode::Normal => 0b00,
                    self::DeepSleepMode::PreserveRAM => 0b01,
                    self::DeepSleepMode::DiscardRAM => 0b11,
                };

                pack!(buf, 0x10, [mode])
            }
            DataEntryMode(data_entry_mode, increment_axis) => {
                let mode = match data_entry_mode {
                    self::DataEntryMode::DecrementXDecrementY => 0b00,
                    self::DataEntryMode::IncrementXDecrementY => 0b01,
                    self::DataEntryMode::DecrementXIncrementY => 0b10,
                    self::DataEntryMode::IncrementYIncrementX => 0b11,
                };
                let axis = match increment_axis {
                    IncrementAxis::Horizontal => 0b000,
                    IncrementAxis::Vertical => 0b100,
                };

                pack!(buf, 0x11, [axis | mode])
            }
            SoftReset => pack!(buf, 0x12, []),
            // TemperatatSensorSelection(TemperatureSensor) => {
            // }
            // WriteTemperatureSensor(u16) => {
            // }
            // ReadTemperatureSensor(u16) => {
            // }
            // WriteExternalTemperatureSensor(u8, u8, u8) => {
            // }
            UpdateDisplay => pack!(buf, 0x20, []),
            // UpdateDisplayOption1(RamOption, RamOption) => {
            // }
            UpdateDisplayOption2(value) => pack!(buf, 0x22, [value]),
            // EnterVCOMSensing => {
            // }
            // VCOMSenseDuration(u8) => {
            // }
            WriteVCOM(value) => pack!(buf, 0x2C, [value]),
            DummyLinePeriod(period) => {
                debug_assert!(Contains::contains(&(0..=MAX_DUMMY_LINE_PERIOD), period));
                pack!(buf, 0x3A, [period])
            }
            GateLineWidth(tgate) => pack!(buf, 0x3B, [tgate]),
            BorderWaveform(border_waveform) => pack!(buf, 0x3C, [border_waveform]),
            StartEndXPosition(start, end) => pack!(buf, 0x44, [start, end]),
            StartEndYPosition(start, end) => {
                let [start_upper, start_lower] = u16_as_u8(start);
                let [end_upper, end_lower] = u16_as_u8(end);
                pack!(buf, 0x45, [start_lower, start_upper, end_lower, end_upper])
            }
            // AutoWriteRedPattern(u8) => {
            // }
            // AutoWriteBlackPattern(u8) => {
            // }
            XAddress(address) => pack!(buf, 0x4E, [address]),
            YAddress(address) => pack!(buf, 0x4F, [address]),
            AnalogBlockControl(value) => pack!(buf, 0x74, [value]),
            DigitalBlockControl(value) => pack!(buf, 0x7E, [value]),
            _ => unimplemented!(),
        };

        interface.send_command(command)?;
        if data.len() == 0 {
            Ok(())
        } else {
            interface.send_data(data)
        }
    }
}

impl<'buf> BufCommand<'buf> {
    pub(crate) fn execute<I: DisplayInterface>(&self, interface: &mut I) -> Result<(), I::Error> {
        use self::BufCommand::*;

        let (command, data) = match self {
            WriteBlackData(buffer) => (0x24, buffer),
            WriteRedData(buffer) => (0x26, buffer),
            WriteLUT(buffer) => (0x32, buffer),
        };

        interface.send_command(command)?;
        if data.len() == 0 {
            Ok(())
        } else {
            interface.send_data(data)
        }
    }
}

impl<C> Contains<C> for core::ops::Range<C>
where
    C: Copy + PartialOrd,
{
    fn contains(&self, item: C) -> bool {
        item >= self.start && item < self.end
    }
}

impl<C> Contains<C> for core::ops::RangeInclusive<C>
where
    C: Copy + PartialOrd,
{
    fn contains(&self, item: C) -> bool {
        item >= *self.start() && item <= *self.end()
    }
}
