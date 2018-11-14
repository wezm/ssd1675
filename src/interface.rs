use hal;

// Section 15.2 of the HINK-E0213A07 data sheet says to hold for 10ms
const RESET_DELAY_MS: u8 = 10;

const MAX_SPI_SPEED_HZ: u32 = 20_000_000;

pub trait DisplayInterface {
    type Error;

    fn send_command(&mut self, command: u8) -> Result<(), Self::Error>;
    fn send_data(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D);
    fn busy_wait(&self);
}

pub struct Interface<SPI, CS, BUSY, DC, RESET> {
    /// SPI
    spi: SPI,
    /// CS for SPI
    cs: CS,
    /// Low for busy, Wait until display is ready!
    busy: BUSY,
    /// Data/Command Control Pin (High for data, Low for command)
    dc: DC,
    /// Pin for Reseting
    reset: RESET,
}

impl<SPI, CS, BUSY, DC, RESET> Interface<SPI, CS, BUSY, DC, RESET>
where
    SPI: hal::blocking::spi::Write<u8>,
    CS: hal::digital::OutputPin,
    BUSY: hal::digital::InputPin,
    DC: hal::digital::OutputPin,
    RESET: hal::digital::OutputPin,
{
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
    CS: hal::digital::OutputPin,
    BUSY: hal::digital::InputPin,
    DC: hal::digital::OutputPin,
    RESET: hal::digital::OutputPin,
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
        while self.busy.is_high() {}
    }
}
