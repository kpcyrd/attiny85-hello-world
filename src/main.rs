#![no_std]
#![no_main]

use attiny_hal::clock;
use attiny_hal::delay::Delay;
use attiny_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use avr_device::attiny85::Peripherals;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    let pac = Peripherals::take().unwrap();
    let pins = attiny_hal::Pins::new(pac.PORTB);

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
