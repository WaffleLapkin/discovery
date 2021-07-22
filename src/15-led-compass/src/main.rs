#![deny(unsafe_code)]
#![allow(uncommon_codepoints)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{
    entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, Measurement,
};

use m::Float as _;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let Measurement { x, y, .. } = lsm303dlhc.mag_data().unwrap();

        //iprintln!(&mut itm.stim[0], "{:?}",);

        let 𝜃 = (y as f32).atan2(x as f32); // in radians

        //                ^ y
        //       5𝜏/16    |    3𝜏/16
        //           _.-""|""-._
        //         .'  \  |  /  `.                     E
        //  7𝜏/16 /     \ | /     \ 𝜏/16           NE     SE
        //       |¯--..__\|/__..--¯|             N     o     S
        //    ---|--------|--------|---> x         NW     SW
        //       |_..--¯¯/|\¯¯--.._|                   W
        // -7𝜏/16 \     / | \     / -𝜏/16
        //         `._ /  |  \ _.'
        //            `-..|..-'
        //      -5𝜏/16    |    -3𝜏/16
        //                |
        //

        let 𝜏 = core::f32::consts::TAU;

        let dir = match () {
            () if 𝜃.abs() <= (𝜏 / 16.) => Direction::South,

            () if (𝜏 / 16.) < 𝜃 && 𝜃 <= (3. * 𝜏 / 16.) => Direction::Southeast,
            () if (3. * 𝜏 / 16.) < 𝜃 && 𝜃 <= (5. * 𝜏 / 16.) => Direction::East,
            () if (5. * 𝜏 / 16.) < 𝜃 && 𝜃 <= (7. * 𝜏 / 16.) => Direction::Northeast,

            () if 𝜃.abs() > (7. * 𝜏 / 16.) => Direction::North,

            () if -(7. * 𝜏 / 16.) <= 𝜃 && 𝜃 < -(5. * 𝜏 / 16.) => Direction::Northwest,
            () if -(5. * 𝜏 / 16.) <= 𝜃 && 𝜃 < -(3. * 𝜏 / 16.) => Direction::West,
            () if -(3. * 𝜏 / 16.) <= 𝜃 && 𝜃 < -(𝜏 / 16.) => Direction::Southwest,

            _ => unreachable!(),
        };

        iprintln!(
            &mut itm.stim[0],
            "𝜃 = {:+02.2}, 𝜃/𝜏 = {:+02.2}, dir = {:?}",
            𝜃,
            𝜃 / 𝜏,
            dir
        );

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(1_000_u16);
    }
}
