#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Power on the TIM6 timer
    rcc.apb1enr.write(|w| w.tim6en().set_bit());

    tim6.cr1.write(|w| {
        // Select one pulse mode
        w.opm().set_bit();
        // Keep the counter disabled for now
        w.cen().clear_bit()
    });

    // Configure the prescaler to have the counter operate at 1 KHz
    //
    // The frequency of the counter is `apb1 / (psc + 1)`, `apb1 = 8 MHz` =>
    // $$
    // 10^3 = 8 * 10^6 / (psc + 1)
    // psc + 1 = 8 * 10^3
    // psc = (8 * 10^3) + 1
    // psc = 7999
    // $$
    let psc = 7999;
    tim6.psc.write(|w| w.psc().bits(psc));

    #[inline(never)]
    fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
        // Set the timer to go off in `ms` ticks.
        // Since we've configured the prescaler to have the counter operate at 1 KHz 1 tick = 1 ms.
        tim6.arr.write(|w| w.arr().bits(ms));

        // Enable the counter
        tim6.cr1.write(|w| w.cen().set_bit());

        // Wait until the alarm goes off (until the update event occurs)
        while !tim6.sr.read().uif().bit_is_set() {}

        // Clear the update event flag
        tim6.sr.write(|w| w.uif().clear_bit());
    }

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
