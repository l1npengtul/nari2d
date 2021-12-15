#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(portable_simd)]

#[cfg(feature = "serde_impl")]
#[macro_use]
extern crate serde;

pub mod accel;
pub mod asset;
pub mod error;
pub mod geometry;
mod macros;
pub mod traits;
