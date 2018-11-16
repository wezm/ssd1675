use hal;
use color::Color;
use display::{Display, Rotation};
use interface::DisplayInterface;
use core::ops::{Deref, DerefMut};

pub struct GraphicDisplay<'a, I> where I: DisplayInterface {
    display: Display<I>,
    black_buffer: &'a mut [u8],
    red_buffer: &'a mut [u8],
}

impl<'a, I> GraphicDisplay<'a, I> where I: DisplayInterface {
    pub fn new(display: Display<I>, black_buffer: &'a mut [u8], red_buffer: &'a mut [u8]) -> Self {
        GraphicDisplay { display, black_buffer, red_buffer }
    }

    pub fn update<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        self.display.update(self.black_buffer, self.red_buffer, delay)
    }

    pub fn clear(&mut self, _color: Color) {
        // TODO: Support color
        for byte in &mut self.black_buffer.iter_mut() {
            *byte = 1; // background_color.get_byte_value();
        }

        // TODO: Combine loops
        for byte in &mut self.red_buffer.iter_mut() {
            *byte = 0; // background_color.get_byte_value();
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Give us index inside the buffer and the bit-position in that u8 which needs to be changed
        let (index, bit) = rotation(x, y, self.cols() as u32, self.rows() as u32, self.rotation());
        let index = index as usize;

        match color {
            Color::Black => {
                self.black_buffer[index] &= !bit;
                self.red_buffer[index] &= !bit;
            }
            Color::White => {
                self.black_buffer[index] |= bit;
                self.red_buffer[index] &= !bit;
            }
            Color::Red => {
                self.black_buffer[index] &= !bit;
                self.red_buffer[index] |= bit;
            }
        }
    }
}

impl<'a, I> Deref for GraphicDisplay<'a, I> where I: DisplayInterface {
    type Target = Display<I>;

    fn deref(&self) -> &Display<I> {
        &self.display
    }
}

impl<'a, I> DerefMut for GraphicDisplay<'a, I> where I: DisplayInterface {
    fn deref_mut(&mut self) -> &mut Display<I> {
        &mut self.display
    }
}

fn rotation(x: u32, y: u32, width: u32, height: u32, rotation: Rotation) -> (u32, u8) {
    match rotation {
        Rotation::Rotate0 => (
            x / 8 + (width / 8) * y,
            0x80 >> (x % 8),
        ),
        Rotation::Rotate90 => (
            (width - 1 - y) / 8 + (width / 8) * x,
            0x01 << (y % 8),
        ),
        Rotation::Rotate180 => (
            ((width / 8) * height - 1) - (x / 8 + (width / 8) * y),
            0x01 << (x % 8),
        ),
        Rotation::Rotate270 => (
            y / 8 + (height - 1 - x) * (width / 8),
            0x80 >> (y % 8),
        ),
    }
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::{drawable::Pixel, Drawing, prelude::UnsignedCoord};

#[cfg(feature = "graphics")]
impl<'a, I> Drawing<Color> for GraphicDisplay<'a, I>
where
    I: DisplayInterface,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = Pixel<Color>>,
    {
        for Pixel(UnsignedCoord(x, y), colour) in item_pixels {
            self.set_pixel(x, y, colour);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::{Display, DisplayInterface, Dimensions, GraphicDisplay, Color, Rotation};

    const ROWS: u16 = 212;
    const COLS: u8 = 104;
    const BUFFER_SIZE: usize = ROWS as usize * COLS as usize;

    struct MockInterface {}
    struct MockError {}

    impl MockInterface {
        fn new() -> Self {
            MockInterface {}
        }
    }

    impl DisplayInterface for MockInterface {
        type Error = MockError;

        fn reset<D: hal::blocking::delay::DelayMs<u8>>(&mut self, delay: &mut D) {
            // self.reset.set_low();
            // delay.delay_ms(RESET_DELAY_MS);
            // self.reset.set_high();
            // delay.delay_ms(RESET_DELAY_MS);
        }

        fn send_command(&mut self, command: u8) -> Result<(), Self::Error> {
            // self.dc.set_low();
            // self.write(&[command])?;
            // self.dc.set_high();

            Ok(())
        }

        fn send_data(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            // self.dc.set_high();
            // self.write(data)
            Ok(())
        }

        fn busy_wait(&self) {
            // while self.busy.is_high() {}
        }
    }

    // fn setup_display<'a>() -> GraphicDisplay<'a, MockInterface> {
    // }

    #[test]
    fn set_corner_pixels() {
        let interface = MockInterface::new();
        let dimensions = Dimensions { rows: ROWS, cols: COLS };
        let mut black_buffer = [0u8; BUFFER_SIZE]; // FIXME: This is using 1 byte per pixel when it only needs to be one bit
        let mut red_buffer = [0u8; BUFFER_SIZE];
        let display = Display::new(interface, dimensions, Rotation::Rotate270);
        GraphicDisplay::new(display, &mut black_buffer, &mut red_buffer);
    }
}
