#![no_std]

extern crate embedded_hal as hal;

mod command;
mod interface;
mod display;
mod graphics;
mod color;

pub use interface::DisplayInterface;
pub use interface::Interface;
pub use display::{Display, Dimensions, Rotation};
pub use graphics::GraphicDisplay;
pub use color::Color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
