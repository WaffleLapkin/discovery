#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, OutputSwitch};

#[entry]
fn main() -> ! {
    // Test shows that leds go in order
    // N, NE, E, SE, S, SW, W, NW
    // (clockwise, staring from N)
    let (mut delay, mut leds) = aux5::init();

    // The [diagram] suggests that there are either 1 or 2 leds turned on at a time.
    // Every 50 ms either the "older" (the one that is on for longer, `left`) gets turned
    // off, or a new one is turned on (and turning off and on is changing every time).
    //
    // This can be acheived with a ~100 ms cycle:
    // - turn on the `right` (new) led
    // - delay for ~50 ms
    // - turn off the `left` (old) led
    // - delay for ~50 ms
    // - progress leds (`left = old`, `right = <a new one>`)
    //
    // [diagram]: https://docs.rust-embedded.org/discovery/assets/timing-diagram.png

    let mut hp_v = 50_u16;
    let half_period = volatile::Volatile::new(&mut hp_v);

    for idx in (0..8).cycle() {
        let [left, right] = wrapping_get2_mut(&mut leds, idx);

        // turn on the `right` (new) led
        right.on().ok();
        // delay for ~50 ms
        delay.delay_ms(half_period.read());

        // turn off the `left` (old) led
        left.off().ok();
        // delay for ~50 ms
        delay.delay_ms(half_period.read());
    }

    // Cycle<..> iterator is infinite
    unreachable!()
}

fn wrapping_get2_mut<T>(slice: &mut [T], idx: usize) -> [&mut T; 2] {
    if idx == slice.len() - 1 {
        // Wrapping case

        let (l, r) = slice.split_at_mut(idx);
        [&mut r[0], &mut l[0]]
    } else {
        let (l, r) = slice.split_at_mut(idx + 1);
        [&mut l[l.len() - 1], &mut r[0]]
    }
}
