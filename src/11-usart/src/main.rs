#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::iter;

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut _itm) = aux11::init();

    let recv = || {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        usart1.rdr.read().rdr().bits() as u8
    };

    let send = |byte| {
        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {}

        usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)))
    };

    // Echo
    iter::repeat_with(recv).for_each(send);

    loop {}
}
