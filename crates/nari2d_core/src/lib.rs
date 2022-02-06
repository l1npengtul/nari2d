#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(portable_simd)]
#![feature(map_first_last)]
#![feature(total_cmp)]

// chapter 8 - the core
// painful/10, would play again

#[cfg(feature = "serde_impl")]
#[macro_use]
extern crate serde;

pub mod accel;
pub mod asset;
pub mod collections;
pub mod error;
pub mod geometry;
mod macros;
pub mod traits;
