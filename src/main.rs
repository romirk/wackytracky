use std::{error::Error, thread, time::Duration};

use rppal::{gpio::Gpio, system::DeviceInfo};

const GPIO_LED: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    while (true) {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
