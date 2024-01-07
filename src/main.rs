#![no_std]
#![no_main]

use attiny_hal::{clock, delay::Delay, Peripherals, Pins};
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;

#[no_mangle]
fn main() -> ! {
    let pac = Peripherals::take().unwrap();
    let pins = Pins::new(pac.PORTB);

    let mut delay = Delay::<clock::MHz8>::new();

    // Digital pin 1 is also connected to an onboard LED marked "L"
    let mut led = pins.pb1.into_output();
    led.set_high();

    loop {
        led.set_high();
        delay.delay_ms(500_u16);
        led.set_low();
        delay.delay_ms(1000_u16);

        led.set_high();
        delay.delay_ms(250_u16);
        led.set_low();
        delay.delay_ms(100_u16);
        led.set_high();
        delay.delay_ms(250_u16);
        led.set_low();
        delay.delay_ms(1000_u16);
    }
}
