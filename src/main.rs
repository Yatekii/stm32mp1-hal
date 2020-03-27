#![no_std]
#![no_main]
use panic_halt as _;

use cortex_m_rt::entry;

pub use stm32mp1_pac as target;

#[entry]
fn main() -> ! {
    if let Some(p) = target::Peripherals::take() {
        let i = 8;
        let offset = 2 * i;
        unsafe {
            &(*target::GPIOD::ptr())
                .gpiox_pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
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
    };

    loop {
        continue;
    }
}
