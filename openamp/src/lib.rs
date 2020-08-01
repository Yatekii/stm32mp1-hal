#![no_std]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
use std as core;

pub mod vring;