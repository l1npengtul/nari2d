#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(iter_collect_into)]

// chapter 8 - the core
// painful/10, would play again

#[cfg(feature = "serde_impl")]
#[macro_use]
extern crate serde;

#[macro_use]
extern crate nari2d_macros;

pub mod accel;
pub mod asset;
pub mod error;
pub mod geometry;
mod macros;
