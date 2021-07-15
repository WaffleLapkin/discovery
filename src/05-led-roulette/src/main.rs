#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    let mut half_period_ = 300_u16;
    let half_period = volatile::Volatile::new(&mut half_period_);

    loop {
        leds[0].on().ok();
        delay.delay_ms(half_period.read());

        leds[0].off().ok();
        delay.delay_ms(half_period.read());
    }
}
