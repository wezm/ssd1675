# SSD1675 EPD display driver

<img src="https://raw.githubusercontent.com/wezm/ssd1675/master/IMG_2198.jpg" width="459" alt="Photo of Inky pHAT ePaper display on Raspberry Pi Zero W" />

Rust driver for the [Solomon Systech SSD1675][SSD1675] e-Paper display (EPD)
controller, for use with [embedded-hal].

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

[SSD1675]: http://www.solomon-systech.com/en/product/advanced-display/bistable-display-driver-ic/SSD1675/
[embedded-hal]: https://crates.io/crates/embedded-hal
[Inky pHat]: https://shop.pimoroni.com/products/inky-phat
