use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

use libc::{c_void, sighandler_t, SIGINT, signal};
use rppal::gpio::Gpio;

const GPIO_LED: u8 = 16;
static RUNNING: AtomicBool = AtomicBool::new(true);

fn term_handler(sig: i32) {
    match sig {
        SIGINT => RUNNING.store(false, Relaxed),
        _ => (),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        signal(SIGINT, term_handler as *mut c_void as sighandler_t);
    };

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    while RUNNING.load(Relaxed) {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    pin.set_low();

    Ok(())
}
