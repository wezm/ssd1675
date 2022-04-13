extern crate libm;

use hal;

use command::{BufCommand, Command, DeepSleepMode};
use config::Config;
use interface::DisplayInterface;

// Max display resolution is 160x296
/// The maximum number of rows supported by the controller
pub const MAX_GATE_OUTPUTS: u16 = 300;
/// The maximum number of columns supported by the controller
pub const MAX_SOURCE_OUTPUTS: u16 = 400;

// Magic numbers from the data sheet
const ANALOG_BLOCK_CONTROL_MAGIC: u8 = 0x54;
const DIGITAL_BLOCK_CONTROL_MAGIC: u8 = 0x3B;

/// Represents the dimensions of the display.
pub struct Dimensions {
    /// The number of rows the display has.
    ///
    /// Must be less than or equal to MAX_GATE_OUTPUTS.
    pub rows: u16,
    /// The number of columns the display has.
    ///
    /// Must be less than or equal to MAX_SOURCE_OUTPUTS.
    pub cols: u16,
}

/// Represents the physical rotation of the display relative to the native orientation.
///
/// For example the native orientation of the Inky pHAT display is a tall (portrait) 104x212
/// display. `Rotate270` can be used to make it the right way up when attached to a Raspberry Pi
/// Zero with the ports on the top.
#[derive(Clone, Copy)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Default for Rotation {
    /// Default is no rotation (`Rotate0`).
    fn default() -> Self {
        Rotation::Rotate0
    }
}

/// A configured display with a hardware interface.
pub struct Display<'a, I>
where
    I: DisplayInterface,
{
    interface: I,
    config: Config<'a>,
}

impl<'a, I> Display<'a, I>
where
    I: DisplayInterface,
{
    /// Create a new display instance from a DisplayInterface and Config.
    ///
    /// The `Config` is typically created with `config::Builder`.
    pub fn new(interface: I, config: Config<'a>) -> Self {
        Self { interface, config }
    }

    /// Perform a hardware reset followed by software reset.
    ///
    /// This will wake a controller that has previously entered deep sleep.
    pub fn reset<D: hal::blocking::delay::DelayMs<u8>>(
        &mut self,
        delay: &mut D,
    ) -> Result<(), I::Error> {
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

        Command::DriverOutputControl(self.config.dimensions.rows, 0x00)
            .execute(&mut self.interface)?;

        Command::GateDrivingVoltage(0x17).execute(&mut self.interface)?;
        Command::SourceDrivingVoltage(0x41, 0xAC, 0x32).execute(&mut self.interface)?;
        self.config.dummy_line_period.execute(&mut self.interface)?;
        self.config.gate_line_width.execute(&mut self.interface)?;
        self.config.data_entry_mode.execute(&mut self.interface)?;

        self.config.write_vcom.execute(&mut self.interface)?;

        // POR is HiZ. Need pull from config
        Command::BorderWaveform(0b00110001).execute(&mut self.interface)?;

        if let Some(ref write_lut) = self.config.write_lut {
            write_lut.execute(&mut self.interface)?;
        }


        let end = (self.config.dimensions.cols / 8 - 1) as u8;
        Command::StartEndXPosition(0, end).execute(&mut self.interface)?;
        Command::StartEndYPosition(0, self.config.dimensions.rows).execute(&mut self.interface)?;

        Ok(())
    }

    /// Update the display by writing the supplied B/W and Red buffers to the controller.
    ///
    /// This method will write the two buffers to the controller then initiate the update
    /// display command. Currently it will busy wait until the update has completed.
    pub fn update<D: hal::blocking::delay::DelayMs<u8>>(
        &mut self,
        black: &[u8],
        red: &[u8],
        delay: &mut D,
    ) -> Result<(), I::Error> {
        // Write the B/W RAM
        let buf_limit = libm::ceilf((self.rows() as usize * self.cols() as usize) as f32 / 8.) as usize;
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
        // the interface like a smart pointer in which deref would wait until it's not
        // busy.
        self.interface.busy_wait();

        Ok(())
    }

    /// Enter deep sleep mode.
    ///
    /// This puts the display controller into a low power mode. `reset` must be called to wake it
    /// from sleep.
    pub fn deep_sleep(&mut self) -> Result<(), I::Error> {
        Command::DeepSleepMode(DeepSleepMode::PreserveRAM).execute(&mut self.interface)
    }

    /// Returns the number of rows the display has.
    pub fn rows(&self) -> u16 {
        self.config.dimensions.rows
    }

    /// Returns the number of columns the display has.
    pub fn cols(&self) -> u16 {
        self.config.dimensions.cols
    }

    /// Returns the rotation the display was configured with.
    pub fn rotation(&self) -> Rotation {
        self.config.rotation
    }
}
