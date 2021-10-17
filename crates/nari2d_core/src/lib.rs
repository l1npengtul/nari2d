#![deny(clippy::pedantic)]
#![warn(clippy::all)]

#[cfg(feature = "serde_serialize")]
#[macro_use]
extern crate serde;

pub mod asset;
pub mod error;
pub mod geometry;
mod macros;
pub mod mesh;
pub mod traits;