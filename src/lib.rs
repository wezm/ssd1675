#![no_std]

//! SSD1675 ePaper Display Driver
//!
//! For a complete example see
//! [the Raspberry Pi Inky pHAT example](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs).
//!
//! ### Usage
//!
//! To control a display you will need:
//!
//! * An [Interface] to the controller
//! * A [display configuration][Config]
//! * A [Display]
//!
//! The `Interface` captures the details of the hardware connection to the SSD1675 controller. This
//! includes an SPI device and some GPIO pins. The SSD1675 can control many different displays that
//! vary in dimensions, rotation, and driving characteristics. The [Config] captures these details.
//! To aid in constructing the `Config` there is a [Builder] interface. Finally when you have an
//! interface and a Config a Display instance can be created. Optionally the Display can be
//! promoted to a [GraphicDisplay], which allows it to use the functionality from the
//! [embedded-graphics crate]. The plain display only provides the ability to update the display by
//! passing black/white and red buffers.
//!
//! To update the display you will typically follow this flow:
//!
//! * [reset]
//! * [clear]
//! * [update]
//! * [sleep]

extern crate embedded_hal as hal;

#[cfg(test)]
#[macro_use]
extern crate std;

mod color;
mod command;
mod config;
mod display;
mod graphics;
mod interface;

pub use color::Color;
pub use config::Builder;
pub use display::{Dimensions, Display, Rotation};
pub use graphics::GraphicDisplay;
pub use interface::DisplayInterface;
pub use interface::Interface;
