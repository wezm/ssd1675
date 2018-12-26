# SSD1675 ePaper Display Driver

Rust driver for the [Solomon Systech SSD1675][SSD1675] e-Paper display (EPD)
controller, for use with [embedded-hal].

[![Build Status](https://travis-ci.org/wezm/ssd1675.svg?branch=master)](https://travis-ci.org/wezm/ssd1675)
[![crates.io](https://img.shields.io/crates/v/ssd1675.svg)](https://crates.io/crates/ssd1675)
[![Documentation](https://docs.rs/ssd1675/badge.svg)][crate-docs]

<img src="https://raw.githubusercontent.com/wezm/ssd1675/master/IMG_2198.jpg" width="459" alt="Photo of Inky pHAT ePaper display on Raspberry Pi Zero W" />

## Description

This driver is intended to work on embedded platforms using the `embedded-hal`
trait library. It is `no_std` compatible, builds on stable Rust, and only uses
safe Rust. It supports the 4-wire SPI interface.

## Tested Devices

The library has been tested and confirmed working on these devices:

* Red/Black/White [Inky pHAT] version 2 on Raspberry Pi Zero (pictured above)

## Examples

**Note:** To build the examples the `examples` feature needs to be enabled. E.g.

    cargo build --release --examples --features examples

### Raspberry Pi with Inky pHAT

The [Raspberry Pi Inky pHAT
example](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs),
shows how to display information on an [Inky pHAT] using this crate. The photo
at the top of the page shows this example in action. To avoid the need to
compile on the Raspberry Pi itself I recommend cross-compiling with the [cross]
tool. With `cross` installed build the example as follows:

    cross build --target=arm-unknown-linux-gnueabi --release --example raspberry_pi_inky_phat --features examples

After it is built copy
`target/arm-unknown-linux-gnueabi/release/examples/raspberry_pi_inky_phat` to
the Raspberry Pi.

## Credits

* [Waveshare EPD driver](https://github.com/caemor/epd-waveshare)
* [SSD1306 OLED display driver](https://github.com/jamwaffles/ssd1306)
* [SSD1322 OLED display driver](https://github.com/edarc/ssd1322)
* [Pimoroni Python library for the Inky pHAT and Inky wHAT e-paper displays](https://github.com/pimoroni/inky)

## License

`ssd1675` is dual licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) **or**
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crate-docs]: https://docs.rs/ssd1675
[cross]: https://github.com/rust-embedded/cross
[embedded-hal]: https://crates.io/crates/embedded-hal
[Inky pHAT]: https://shop.pimoroni.com/products/inky-phat
[LICENSE-APACHE]: https://github.com/wezm/ssd1675/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/wezm/ssd1675/blob/master/LICENSE-MIT
[SSD1675]: http://www.solomon-systech.com/en/product/advanced-display/bistable-display-driver-ic/SSD1675/
