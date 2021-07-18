#![deny(unsafe_code)]
#![allow(clippy::redundant_pattern_matching, clippy::empty_loop)]
#![no_main]
#![no_std]

use core::iter;

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

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

        //iprintln!(&mut _itm.stim[0], "Sending {}", byte as char);

        usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)))
    };

    let rev_send = |bytes: &[u8]| bytes.iter().rev().copied().for_each(&send);
    let msg_send = |bytes: &[u8]| bytes.iter().copied().for_each(&send);

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    for byte in iter::repeat_with(recv) {
        // Enter!
        if byte == 0xd {
            rev_send(&buffer);
            buffer.clear();

            continue;
        }

        if let Err(_) = buffer.push(byte) {
            msg_send(b"Error: Couldn't append input: Buffer overflow");
            buffer.clear();
        }
    }

    loop {}
}
