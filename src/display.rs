use hal;

use interface::DisplayInterface;

struct Config {}

struct Dimensions {
    rows: usize,
    cols: usize,
}

pub struct Display<I> where I: DisplayInterface {
    interface: I,
    dimensions: Dimensions,
    rotation: u8,
}

impl<I> Display<I> where I: DisplayInterface {
    fn new(interface: I, dimensions: Dimensions, rotation: u8) -> Self {
        Self { interface, dimensions, rotation }
    }

    fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) {
        self.interface.reset(delay)
    }

    fn init(&mut self, config: Config) -> Result<(), I::Error> {


        Ok(())
    }

    fn deep_sleep(&mut self) -> Result<(), I::Error> {
        // TODO: Send DeepSleep command
        unimplemented!()
    }
}


