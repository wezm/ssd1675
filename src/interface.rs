use hal;

// Section 15.2 of the HINK-E0213A07 data sheet says to hold for 10ms
const RESET_DELAY_MS: u8 = 10;

/// Trait implemented by displays to provide implemenation of core functionality.
pub trait DisplayInterface {
    type Error;

    /// Send a command to the controller.
    ///
    /// Prefer calling `execute` on a [Commmand](../command/enum.Command.html) over calling this
    /// directly.
    fn send_command(&mut self, command: u8) -> Result<(), Self::Error>;

    /// Send data for a command.
    fn send_data(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Reset the controller.
    fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D);

    /// Wait for the controller to indicate it is not busy.
    fn busy_wait(&self);
}

/// The hardware interface to a display.
///
/// ### Example
///
/// This example uses the Linux implementation of the embedded HAL traits to build a display
/// interface. For a complete example see [the Raspberry Pi Inky pHAT example](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs).
///
/// ```ignore
/// extern crate linux_embedded_hal;
/// use linux_embedded_hal::spidev::{self, SpidevOptions};
/// use linux_embedded_hal::sysfs_gpio::Direction;
/// use linux_embedded_hal::Delay;
/// use linux_embedded_hal::{Pin, Spidev};
///
/// extern crate ssd1675;
/// use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};
///
/// // Configure SPI
/// let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
/// let options = SpidevOptions::new()
///     .bits_per_word(8)
///     .max_speed_hz(4_000_000)
///     .mode(spidev::SPI_MODE_0)
///     .build();
/// spi.configure(&options).expect("SPI configuration");
///
/// // https://pinout.xyz/pinout/inky_phat
/// // Configure Digital I/O Pins
/// let cs = Pin::new(8); // BCM8
/// cs.export().expect("cs export");
/// while !cs.is_exported() {}
/// cs.set_direction(Direction::Out).expect("CS Direction");
/// cs.set_value(1).expect("CS Value set to 1");
///
/// let busy = Pin::new(17); // BCM17
/// busy.export().expect("busy export");
/// while !busy.is_exported() {}
/// busy.set_direction(Direction::In).expect("busy Direction");
///
/// let dc = Pin::new(22); // BCM22
/// dc.export().expect("dc export");
/// while !dc.is_exported() {}
/// dc.set_direction(Direction::Out).expect("dc Direction");
/// dc.set_value(1).expect("dc Value set to 1");
///
/// let reset = Pin::new(27); // BCM27
/// reset.export().expect("reset export");
/// while !reset.is_exported() {}
/// reset
///     .set_direction(Direction::Out)
///     .expect("reset Direction");
/// reset.set_value(1).expect("reset Value set to 1");
///
/// // Build the interface from the pins and SPI device
/// let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

#[allow(dead_code)] // Prevent warning about CS being unused
pub struct Interface<SPI, CS, BUSY, DC, RESET> {
    /// SPI interface
    spi: SPI,
    /// CS (chip select) for SPI (output)
    cs: CS,
    /// Active low busy pin (input)
    busy: BUSY,
    /// Data/Command Control Pin (High for data, Low for command) (output)
    dc: DC,
    /// Pin for reseting the controller (output)
    reset: RESET,
}

impl<SPI, CS, BUSY, DC, RESET> Interface<SPI, CS, BUSY, DC, RESET>
where
    SPI: hal::blocking::spi::Write<u8>,
    CS: hal::digital::v2::OutputPin,
    BUSY: hal::digital::v2::InputPin,
    DC: hal::digital::v2::OutputPin,
    RESET: hal::digital::v2::OutputPin,
{
    /// Create a new Interface from embedded hal traits.
    pub fn new(spi: SPI, cs: CS, busy: BUSY, dc: DC, reset: RESET) -> Self {
        Self {
            spi,
            cs,
            busy,
            dc,
            reset,
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), SPI::Error> {
        // Select the controller with chip select (CS)
        // self.cs.set_low();

        // Linux has a default limit of 4096 bytes per SPI transfer
        // https://github.com/torvalds/linux/blob/ccda4af0f4b92f7b4c308d3acc262f4a7e3affad/drivers/spi/spidev.c#L93
        if cfg!(target_os = "linux") {
            for data_chunk in data.chunks(4096) {
                self.spi.write(data_chunk)?;
            }
        } else {
            self.spi.write(data)?;
        }

        // Release the controller
        // self.cs.set_high();

        Ok(())
    }
}

impl<SPI, CS, BUSY, DC, RESET> DisplayInterface for Interface<SPI, CS, BUSY, DC, RESET>
where
    SPI: hal::blocking::spi::Write<u8>,
    CS: hal::digital::v2::OutputPin,
    BUSY: hal::digital::v2::InputPin,
    DC: hal::digital::v2::OutputPin,
    RESET: hal::digital::v2::OutputPin,
{
    type Error = SPI::Error;

    fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) {
        self.reset.set_low();
        delay.delay_ms(RESET_DELAY_MS);
        self.reset.set_high();
        delay.delay_ms(RESET_DELAY_MS);
    }

    fn send_command(&mut self, command: u8) -> Result<(), Self::Error> {
        self.dc.set_low();
        self.write(&[command])?;
        self.dc.set_high();

        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.dc.set_high();
        self.write(data)
    }

    fn busy_wait(&self) {
        while match self.busy.is_high() {
            Ok(x) => x,
            _ => false,
        } {}
    }
}
