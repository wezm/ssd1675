/// Represents the state of a pixel in the display
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
    RedOrYellow,
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::prelude::*;
#[cfg(feature = "graphics")]
impl PixelColor for Color {}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::RedOrYellow,
            _ => panic!("invalid color value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(Color::Black, Color::from(0u8));
        assert_eq!(Color::White, Color::from(1u8));
    }

    #[test]
    fn from_u8_panic() {
        for val in 3..=u8::max_value() {
            extern crate std;
            let result = std::panic::catch_unwind(|| Color::from(val));
            assert!(result.is_err());
        }
    }
}
