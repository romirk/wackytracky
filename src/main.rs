#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use astro::true_coordinates;
use panic_halt as _;

use embedded_hal::serial::Read;

pub mod astro;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let sun = true_coordinates(0.0, 0.0, 0.0);
    let alt = sun.0 as i64;

    ufmt::uwriteln!(&mut serial, "The answer is {}\r", alt).void_unwrap();

    // calculate true coordinates of the sun
    // let sun = true_coordinates(0.0, 0.0, 0.0);

    // get altitude and azimuth
    // let alt = sun.0 as str;
    // let az = sun.1 as str;

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}   