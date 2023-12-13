#![no_std]
#![no_main]

use avr_device::attiny85::Peripherals;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    let pac = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(pac);

    // Digital pin 1 is also connected to an onboard LED marked "L"
    let mut led = pins.d1.into_output();
    led.set_high();

    loop {
        led.set_high();
        arduino_hal::delay_ms(500);
        led.set_low();
        arduino_hal::delay_ms(1000);

        led.set_high();
        arduino_hal::delay_ms(250);
        led.set_low();
        arduino_hal::delay_ms(100);
        led.set_high();
        arduino_hal::delay_ms(250);
        led.set_low();
        arduino_hal::delay_ms(1000);
    }
}
