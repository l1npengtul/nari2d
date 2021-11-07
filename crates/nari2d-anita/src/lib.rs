#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(stdsimd)]

#[cfg(feature = "simd-aarch")]
mod aarch64;
#[cfg(feature = "simd-arm")]
mod arm;
#[cfg(feature = "simd-wasm")]
mod wasm;
#[cfg(feature = "simd-wgpu")]
mod wgpu;
#[cfg(feature = "simd-x86_64")]
mod x86_64;
