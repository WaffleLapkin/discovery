#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, OutputSwitch};

#[entry]
fn main() -> ! {
    // Test shows that leds go in order
    // N, NE, E, SE, S, SW, W, NW
    // (clockwise, staring from N)
    let (mut delay, leds) = aux5::init();

    let mut half_period_ = 300_u16;
    let half_period = volatile::Volatile::new(&mut half_period_);

    for mut led in leds {
        led.on().ok();
        delay.delay_ms(half_period.read());

        led.off().ok();
        delay.delay_ms(half_period.read());
    }

    loop {}
}
