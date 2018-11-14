use hal;

use command::{BufCommand, Command, DataEntryMode, DeepSleepMode, IncrementAxis};
use interface::DisplayInterface;

// Max display resolution is 160x296
const MAX_SOURCE_OUTPUTS: usize = 160;
const MAX_GATE_OUTPUTS: usize = 296;

// Magic numbers from the data sheet
const ANALOG_BLOCK_CONTROL_MAGIC: u8 = 0x54;
const DIGITAL_BLOCK_CONTROL_MAGIC: u8 = 0x3B;

struct Config {}

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

pub struct Display<I> where I: DisplayInterface {
    interface: I,
    dimensions: Dimensions,
    rotation: Rotation,
}

impl<I> Display<I> where I: DisplayInterface {
    pub fn new(interface: I, dimensions: Dimensions, rotation: Rotation) -> Self {
        Self { interface, dimensions, rotation }
    }

    /// Perform a hardware reset followed by software reset
    pub fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        self.interface.reset(delay);
        Command::SoftReset.execute(&mut self.interface)?;
        self.interface.busy_wait();

        self.init(Config {})
    }

    /// Initialise the controller according to Section 9: Typical Operating Sequence
    /// from the data sheet
    fn init(&mut self, config: Config) -> Result<(), I::Error> {
        Command::AnalogBlockControl(ANALOG_BLOCK_CONTROL_MAGIC).execute(&mut self.interface)?;
        Command::DigitalBlockControl(DIGITAL_BLOCK_CONTROL_MAGIC).execute(&mut self.interface)?;

        Command::DriverOutputControl(self.dimensions.rows, 0x00).execute(&mut self.interface)?;

        Command::DummyLinePeriod(0x07).execute(&mut self.interface)?;
        Command::GateLineWidth(0x04).execute(&mut self.interface)?;

        Command::SourceDrivingVoltage(0x2D, 0xB2, 0x22).execute(&mut self.interface)?;
        Command::WriteVCOM(0x3C).execute(&mut self.interface)?;

        // POR is HiZ. Need pull from config
        // Command::BorderWaveform(u8).execute(&mut self.interface)?;

        // BufCommand::WriteLUT().execute(&mut self.interface)?;

        Command::DataEntryMode(DataEntryMode::IncrementYIncrementX, IncrementAxis::Horizontal).execute(&mut self.interface)?;

        let end = self.dimensions.cols / 8 - 1;
        Command::StartEndXPosition(0, end).execute(&mut self.interface)?;
        Command::StartEndYPosition(0, self.dimensions.rows).execute(&mut self.interface)?;

        Ok(())
    }

    pub fn update<D: hal::blocking::delay::DelayMs<u8>>(&mut self, black: &[u8], red: &[u8], delay: &mut D) -> Result<(), I::Error> {
        // Write the B/W RAM
        Command::XAddress(0).execute(&mut self.interface)?;
        Command::YAddress(0).execute(&mut self.interface)?;
        BufCommand::WriteBlackData(&black).execute(&mut self.interface)?;

        // Write the Red RAM
        Command::XAddress(0).execute(&mut self.interface)?;
        Command::YAddress(0).execute(&mut self.interface)?;
        BufCommand::WriteRedData(&red).execute(&mut self.interface)?;

        // Kick off the display update
        Command::UpdateDisplayOption2(0xC7).execute(&mut self.interface)?;
        Command::UpdateDisplay.execute(&mut self.interface)?;
        delay.delay_ms(5);
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
        self.dimensions.rows
    }

    pub fn cols(&self) -> u8 {
        self.dimensions.cols
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}
