// the library for the embedded linux device
extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};

// the eink library
extern crate ssd1675;
use ssd1675::{Display, DisplayInterface, Dimensions, GraphicDisplay, Color, Rotation};

// Graphics
extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::{Font12x16, Font6x8};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use embedded_graphics::Drawing;

// HAL (Traits)
extern crate embedded_hal;
use embedded_hal::prelude::*;

// activate spi, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

const ROWS: u16 = 212;
const COLS: u8 = 104;

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat#
    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs = Pin::new(8); // BCM8
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(17); // BCM17
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(22); // BCM22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let reset = Pin::new(27); // BCM27
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset.set_direction(Direction::Out).expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");
    println!("Pins configured");

    let mut delay = Delay {};

    let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

    // println!("Test all the rotations");
    let dimensions = Dimensions { rows: ROWS, cols: COLS };
    let mut black_buffer = [0u8; ROWS as usize * COLS as usize]; // FIXME: This is using 1 byte per pixel when it only needs to be one bit
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize];
    let display = Display::new(controller, dimensions, Rotation::Rotate270);
    let mut display = GraphicDisplay::new(display, &mut black_buffer, &mut red_buffer);
    display.reset(&mut delay).expect("error resetting display");
    // display.set_rotation(DisplayRotation::Rotate0);
    println!("reset and initialised");

    display.clear(Color::White);
    println!("clear");
    display.draw(
        Font12x16::render_str("Hello Rust")
            .with_stroke(Some(Color::Red))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 88))
            .into_iter(),
    );
    println!("draw");

    // display.set_rotation(DisplayRotation::Rotate90);
    // display.draw(
    //     Font::render_str("Rotate 90!")
    //         .with_stroke(Some(Color::Black))
    //         .with_fill(Some(Color::White))
    //         .translate(Coord::new(5, 50))
    //         .into_iter(),
    // );

    // display.set_rotation(DisplayRotation::Rotate180);
    // display.draw(
    //     Font6x8::render_str("Rotate 180!")
    //         .with_stroke(Some(Color::Black))
    //         .with_fill(Some(Color::White))
    //         .translate(Coord::new(5, 50))
    //         .into_iter(),
    // );

    // display.set_rotation(DisplayRotation::Rotate270);
    // display.draw(
    //     Font6x8::render_str("Rotate 270!")
    //         .with_stroke(Some(Color::Black))
    //         .with_fill(Some(Color::White))
    //         .translate(Coord::new(5, 50))
    //         .into_iter(),
    // );

    // epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
    // epd4in2
    //     .display_frame(&mut spi)
    //     .expect("display frame new graphics");
    // delay.delay_ms(5000u16);

    // println!("Now test new graphics with default rotation and some special stuff:");
    // display.clear_buffer(Color::White);

    // draw a analog clock
    display.draw(
        Circle::new(Coord::new(32, 32), 30)
            .with_stroke(Some(Color::Black))
            .with_stroke_width(2)
            .into_iter(),
    );
    // display.draw(
    //     Line::new(Coord::new(32, 32), Coord::new(0, 32))
    //         .with_stroke(Some(Color::Black))
    //         .with_stroke_width(2)
    //         .into_iter(),
    // );
    // display.draw(
    //     Line::new(Coord::new(32, 32), Coord::new(40, 40))
    //         .with_stroke(Some(Color::Black))
    //         .with_stroke_width(2)
    //         .into_iter(),
    // );
    display.draw(
        Rect::new(Coord::new(32, 32), Coord::new(64, 64))
            .with_fill(Some(Color::Black))
            .into_iter(),
    );

    // // draw white on black background
    // display.draw(
    //     Font6x8::render_str("It's working-WoB!")
    //         // Using Style here
    //         .with_style(Style {
    //             fill_color: Some(Color::Black),
    //             stroke_color: Some(Color::White),
    //             stroke_width: 0u8, // Has no effect on fonts
    //         })
    //         .translate(Coord::new(175, 250))
    //         .into_iter(),
    // );

    // // use bigger/different font
    // display.draw(
    //     Font12x16::render_str("It's working-BoW!")
    //         // Using Style here
    //         .with_style(Style {
    //             fill_color: Some(Color::White),
    //             stroke_color: Some(Color::Black),
    //             stroke_width: 0u8, // Has no effect on fonts
    //         })
    //         .translate(Coord::new(50, 200))
    //         .into_iter(),
    // );

    // // a moving `Hello World!`
    // let limit = 10;
    // for i in 0..limit {
    //     println!("Moving Hello World. Loop {} from {}", (i + 1), limit);

    //     display.draw(
    //         Font6x8::render_str("  Hello World! ")
    //             .with_style(Style {
    //                 fill_color: Some(Color::White),
    //                 stroke_color: Some(Color::Black),
    //                 stroke_width: 0u8, // Has no effect on fonts
    //             })
    //             .translate(Coord::new(5 + i * 12, 50))
    //             .into_iter(),
    //     );

    //     epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
    //     epd4in2
    //         .display_frame(&mut spi)
    //         .expect("display frame new graphics");

    //     delay.delay_ms(1_000u16);
    // }
    display.update(&mut delay).expect("error updating display");
    println!("update...");

    println!("Finished - going to sleep");
    display.deep_sleep()
}

