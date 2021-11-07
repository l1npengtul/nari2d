#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(stdsimd)]

#[cfg(feature = "serde_serialize")]
#[macro_use]
extern crate serde;

pub mod accel;
pub mod asset;
pub mod error;
pub mod geometry;
mod macros;
pub mod traits;
