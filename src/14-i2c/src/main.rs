#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::ops::Not;

use aux14::i2c1;
#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// New (LSM303AGR)
const WHO_AM_I_M: u8 = 0x4F;
const CFG_REG_A_M: u8 = 0x60;
const OUTX_L_REG_M: u8 = 0x68;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    let who_am_i_check = i2c1_read_reg_single(i2c1, MAGNETOMETER, WHO_AM_I_M);
    assert_eq!(
        who_am_i_check, 0b01000000,
        "who am i check failed, LSM303AGR is not properly connected"
    );

    // Initialize magnetometer (see https://www.st.com/resource/en/datasheet/lsm303agr.pdf, 5.3 Startup sequence)
    i2c1_write_reg(i2c1, MAGNETOMETER, CFG_REG_A_M, &[0x00]);

    iprintln!(
        &mut itm.stim[0],
        "CFG_REG_A_M(0x{:02x}) = 0b{:08b}",
        CFG_REG_A_M,
        i2c1_read_reg_single(i2c1, MAGNETOMETER, CFG_REG_A_M)
    );

    loop {
        let mut buf = [0; 6];
        i2c1_read_reg(i2c1, MAGNETOMETER, OUTX_L_REG_M, &mut buf);

        // OUTX_L_REG_M
        // OUTX_H_REG_M
        // OUTY_L_REG_M
        // OUTY_H_REG_M
        // OUTZ_L_REG_M
        // OUTZ_H_REG_M
        let [x_l, x_h, y_l, y_h, z_l, z_h] = buf;

        let x = (((x_h as u16) << 8) | x_l as u16) as i16;
        let y = (((y_h as u16) << 8) | y_l as u16) as i16;
        let z = (((z_h as u16) << 8) | z_l as u16) as i16;

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));

        delay.delay_ms(1_000_u16);
    }
}

fn i2c1_set_reg_addr(i2c1: &i2c1::RegisterBlock, addr: u16, reg: u8) {
    // Broadcast START
    // Broadcast the MAGNETOMETER address with the R/W bit set to Write
    i2c1.cr2.write(|w| {
        // Start
        w.start().set_bit();
        w.nbytes().bits(1);
        w.sadd().bits(addr);
        // w
        w.rd_wrn().clear_bit();
        // Unset automatic end mode for restart
        w.autoend().clear_bit()
    });

    // Set number of bytes to send: 1
    i2c1.cr2.write(|w| w.nbytes().bits(1));

    // Busy wait for tx to be writable
    while i2c1.isr.read().txis().bit().not() {}

    i2c1.txdr.write(|w| w.txdata().bits(reg));

    // Busy wait for write to complete
    while i2c1.isr.read().tc().bit().not() {}
}

fn i2c1_read_reg(i2c1: &i2c1::RegisterBlock, addr: u16, at: u8, buf: &mut [u8]) {
    if buf.len() > u8::MAX as _ {
        unimplemented!()
    }

    i2c1_set_reg_addr(i2c1, addr, at);

    // Broadcast RESTART
    // Broadcast the MAGNETOMETER address with the R/W bit set to Read
    i2c1.cr2.write(|w| {
        // (Re?)Start
        w.start().set_bit();
        w.nbytes().bits(buf.len() as _);
        w.sadd().bits(addr);
        // r
        w.rd_wrn().set_bit();
        // Set automatic end mode for restart
        w.autoend().clear_bit()
    });

    for byte in buf {
        // Busy wait for rx to be readable
        while i2c1.isr.read().rxne().bit().not() {}

        // Read the byte
        *byte = i2c1.rxdr.read().rxdata().bits();
    }

    // Broadcast STOP (automatic)
}

fn i2c1_read_reg_single(i2c1: &i2c1::RegisterBlock, addr: u16, at: u8) -> u8 {
    let mut buf = [0];
    i2c1_read_reg(i2c1, addr, at, &mut buf);
    let [ret] = buf;
    ret
}

fn i2c1_write_reg(i2c1: &i2c1::RegisterBlock, addr: u16, at: u8, data: &[u8]) {
    if data.len() >= u8::MAX as _ || data.len() == 0 {
        unimplemented!()
    }

    //i2c1_set_reg_addr(i2c1, addr, at);

    // Broadcast RESTART
    // Broadcast the MAGNETOMETER address with the R/W bit set to Read
    i2c1.cr2.write(|w| {
        // (Re?)Start
        w.start().set_bit();
        w.nbytes().bits(1 + data.len() as u8);
        w.sadd().bits(addr);
        // w
        w.rd_wrn().clear_bit();
        // Set automatic end mode
        w.autoend().clear_bit()
    });

    let sub = match data.len() {
        1 => at | (1 << 7),  // Enable autoincrement (MSB = 1)
        _ => at & !(1 << 7), // Disable autoincrement (MSB = 0)
    };

    // Busy wait for tx to be writable
    while i2c1.isr.read().txis().bit().not() {}

    i2c1.txdr.write(|w| w.txdata().bits(sub));

    for &byte in data {
        i2c1.txdr.write(|w| w.txdata().bits(byte));
    }
    // Busy wait for write to complete
    while i2c1.isr.read().tc().bit().not() {}

    // Broadcast STOP (automatic)
}
