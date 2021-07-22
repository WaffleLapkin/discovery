#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::ops::Not;

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;

// Old (LSM303DLHC), 0b01001000
const IRA_REG_M: u8 = 0x0A;
// New (LSM303AGR), 0b01000000
const WHO_AM_I_M: u8 = 0x4F;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            // Start
            w.start().set_bit();
            // Magnometer addr
            w.nbytes().bits(1);
            w.sadd().bits(MAGNETOMETER);
            // w
            w.rd_wrn().clear_bit();
            // Unset automatic end mode for restart
            w.autoend().clear_bit()
        });

        // Set number of bytes to send: 1
        i2c1.cr2.write(|w| w.nbytes().bits(1));

        // Busy wait for tx to be writable
        while i2c1.isr.read().txis().bit().not() {}

        i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I_M));

        // Busy wait for write to complete
        while i2c1.isr.read().tc().bit().not() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.write(|w| {
            // (Re?)Start
            w.start().set_bit();
            w.nbytes().bits(1);
            w.sadd().bits(MAGNETOMETER);
            // r
            w.rd_wrn().set_bit();
            // Set automatic end mode for restart
            w.autoend().clear_bit()
        });

        // TODO Receive the contents of the register
        // Busy wait for rx to be readable
        while i2c1.isr.read().rxne().bit().not() {}

        // Broadcast STOP (automatic)

        // Read the byte
        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x4F - 0b01000000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", WHO_AM_I_M, byte);

    loop {}
}
