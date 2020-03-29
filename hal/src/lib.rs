#![cfg_attr(not(test), no_std)]

pub mod gpio;
pub mod rcc;
pub mod time;

pub use stm32mp1_pac as stm32;
