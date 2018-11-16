#![no_std]

extern crate embedded_hal as hal;

#[cfg(test)]
#[macro_use]
extern crate std;

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
