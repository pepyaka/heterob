#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod common;
pub use common::*;

pub mod bit_numbering;
pub mod endianness;
