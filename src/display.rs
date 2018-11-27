use hal;

use config::Config;
use command::{BufCommand, Command, DataEntryMode, DeepSleepMode, IncrementAxis};
use interface::DisplayInterface;

// Max display resolution is 160x296
const MAX_SOURCE_OUTPUTS: usize = 160;
const MAX_GATE_OUTPUTS: usize = 296;

// Magic numbers from the data sheet
const ANALOG_BLOCK_CONTROL_MAGIC: u8 = 0x54;
const DIGITAL_BLOCK_CONTROL_MAGIC: u8 = 0x3B;

pub struct Dimensions {
    pub rows: u16,
    pub cols: u8,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::Rotate0
    }
}

pub struct Display<'a, I> where I: DisplayInterface {
    interface: I,
    config: Config<'a>,
}

impl<'a, I> Display<'a, I> where I: DisplayInterface {
    pub fn new(interface: I, config: Config<'a>) -> Self {
        // TODO: Assert dimensions are evenly divisible by 8
        Self { interface, config }
    }

    /// Perform a hardware reset followed by software reset
    pub fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        self.interface.reset(delay);
        Command::SoftReset.execute(&mut self.interface)?;
        self.interface.busy_wait();

        self.init()
    }

    /// Initialise the controller according to Section 9: Typical Operating Sequence
    /// from the data sheet
    fn init(&mut self) -> Result<(), I::Error> {
        Command::AnalogBlockControl(ANALOG_BLOCK_CONTROL_MAGIC).execute(&mut self.interface)?;
        Command::DigitalBlockControl(DIGITAL_BLOCK_CONTROL_MAGIC).execute(&mut self.interface)?;

        Command::DriverOutputControl(self.config.dimensions.rows, 0x00).execute(&mut self.interface)?;

        self.config.dummy_line_period.execute(&mut self.interface)?;
        self.config.gate_line_width.execute(&mut self.interface)?;

        // Command::GateDrivingVoltage(0b10000 | 0b0001);
        // Command::SourceDrivingVoltage(0x2D, 0xB2, 0x22).execute(&mut self.interface)?;
        self.config.write_vcom.execute(&mut self.interface)?;

        // POR is HiZ. Need pull from config
        // Command::BorderWaveform(u8).execute(&mut self.interface)?;

        // BufCommand::WriteLUT(&LUT_RED).execute(&mut self.interface)?;
        if let Some(ref write_lut) = self.config.write_lut {
            write_lut.execute(&mut self.interface)?;
        }

        self.config.data_entry_mode.execute(&mut self.interface)?;

        let end = self.config.dimensions.cols / 8 - 1;
        Command::StartEndXPosition(0, end).execute(&mut self.interface)?;
        Command::StartEndYPosition(0, self.config.dimensions.rows).execute(&mut self.interface)?;

        Ok(())
    }

    pub fn update<D: hal::blocking::delay::DelayMs<u8>>(&mut self, black: &[u8], red: &[u8], delay: &mut D) -> Result<(), I::Error> {
        // Write the B/W RAM
        let buf_limit = ((self.rows() * self.cols() as u16) as f32 / 8.).ceil() as usize;
        Command::XAddress(0).execute(&mut self.interface)?;
        Command::YAddress(0).execute(&mut self.interface)?;
        BufCommand::WriteBlackData(&black[..buf_limit]).execute(&mut self.interface)?;

        // Write the Red RAM
        Command::XAddress(0).execute(&mut self.interface)?;
        Command::YAddress(0).execute(&mut self.interface)?;
        BufCommand::WriteRedData(&red[..buf_limit]).execute(&mut self.interface)?;

        // Kick off the display update
        Command::UpdateDisplayOption2(0xC7).execute(&mut self.interface)?;
        Command::UpdateDisplay.execute(&mut self.interface)?;
        delay.delay_ms(50);
        // TODO: We don't really need to wait here... the program can go off and do other things
        // and only busy wait if it wants to talk to the display again. Could possibly treat
        // the interface like a smart pointer in which "acquiring" it would wait until it's not
        // busy.
        self.interface.busy_wait();

        Ok(())
    }

    pub fn deep_sleep(&mut self) -> Result<(), I::Error> {
        Command::DeepSleepMode(DeepSleepMode::PreserveRAM).execute(&mut self.interface)
    }

    pub fn rows(&self) -> u16 {
        self.config.dimensions.rows
    }

    pub fn cols(&self) -> u8 {
        self.config.dimensions.cols
    }

    pub fn rotation(&self) -> Rotation {
        self.config.rotation
    }
}

const LUT_RED: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUTT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    64,   12,   32,   12,    6,   // 0 Flash
    16,   8,    4,    4,     6,   // 1 clear
    4,    8,    8,    16,    16,  // 2 bring in the black
    2,    2,    2,    64,    32,  // 3 time for red
    2,    2,    2,    2,     2,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];
