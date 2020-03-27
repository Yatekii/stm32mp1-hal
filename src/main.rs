#![no_std]
#![no_main]
use panic_halt as _;

use cortex_m_rt::entry;

pub use stm32mp1_pac as target;

#[entry]
fn main() -> ! {
    let i = 8;
    let offset = 2 * i;
    unsafe {
        &(*target::RCC::ptr())
            .rcc_mc_ahb3ensetr
            .write(|w| w.hsemen().set_bit());
        &(*target::RCC::ptr())
            .rcc_mc_ahb4ensetr
            .write(|w| w.gpioden().set_bit());

        &(*target::GPIOD::ptr())
            .gpiox_pupdr
            .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)));
        &(*target::GPIOD::ptr())
            .gpiox_otyper
            .modify(|r, w| w.bits(r.bits() & !(0b1 << i)));
        &(*target::GPIOD::ptr())
            .gpiox_moder
            .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)));
    }

    unsafe { (*target::GPIOD::ptr()).gpiox_bsrr.write(|w| w.bits(1 << i)) }

    let mut count: u8 = 0;
    loop {
        count += 1;

        if count & 1 == 1 {
            unsafe { (*target::GPIOD::ptr()).gpiox_bsrr.write(|w| w.bits(1 << i)) }
        } else {
            unsafe {
                (*target::GPIOD::ptr())
                    .gpiox_bsrr
                    .write(|w| w.bits(1 << (i + 16)))
            };
        }

        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}
