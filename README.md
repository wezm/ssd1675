# SSD1675 EPD display driver

Rust driver for the [Solomon Systech SSD1675][SSD1675] e-Paper display (EPD)
controller, for use with [embedded-hal].

[![crates.io](https://img.shields.io/crates/v/ssd1675.svg)](https://crates.io/crates/ssd1675)
[![Documentation](https://docs.rs/ssd1675/badge.svg)][crate-docs]

<img src="https://raw.githubusercontent.com/wezm/ssd1675/master/IMG_2198.jpg" width="459" alt="Photo of Inky pHAT ePaper display on Raspberry Pi Zero W" />


## Description

This driver is intended to work on embedded platforms using the `embedded-hal`
trait library. It is `no_std` compatible, only uses safe Rust, and does not
require an allocator. It supports the 4-wire SPI interface.

## Tested Devices

The library has been tested and confirmed working on these devices:

* Red/Black/White [Inky pHAT] version 2 on Raspberry Pi Zero (pictured above)

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

[SSD1675]: http://www.solomon-systech.com/en/product/advanced-display/bistable-display-driver-ic/SSD1675/
[embedded-hal]: https://crates.io/crates/embedded-hal
[Inky pHat]: https://shop.pimoroni.com/products/inky-phat
[crate-docs]: https://docs.rs/ssd1675
[LICENSE-MIT]: https://github.com/wezm/ssd1675/blob/master/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/wezm/ssd1675/blob/master/LICENSE-APACHE
