//! Peripheral access API for STM32H7 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.37.1)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.37.1/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32H7 devices; for the complete list please
//! see:
//! [stm32h7](https://crates.io/crates/stm32h7)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32h723")]
pub mod stm32h723;

#[cfg(feature = "stm32h725")]
pub mod stm32h725;

#[cfg(feature = "stm32h730")]
pub mod stm32h730;

#[cfg(feature = "stm32h733")]
pub mod stm32h733;

#[cfg(feature = "stm32h735")]
pub mod stm32h735;

#[cfg(feature = "stm32h742")]
pub mod stm32h742;

#[cfg(feature = "stm32h743")]
pub mod stm32h743;

#[cfg(feature = "stm32h743v")]
pub mod stm32h743v;

#[cfg(feature = "stm32h745cm4")]
pub mod stm32h745cm4;

#[cfg(feature = "stm32h745cm7")]
pub mod stm32h745cm7;

#[cfg(feature = "stm32h747cm4")]
pub mod stm32h747cm4;

#[cfg(feature = "stm32h747cm7")]
pub mod stm32h747cm7;

#[cfg(feature = "stm32h750")]
pub mod stm32h750;

#[cfg(feature = "stm32h753")]
pub mod stm32h753;

#[cfg(feature = "stm32h753v")]
pub mod stm32h753v;

#[cfg(feature = "stm32h755cm4")]
pub mod stm32h755cm4;

#[cfg(feature = "stm32h755cm7")]
pub mod stm32h755cm7;

#[cfg(feature = "stm32h757cm4")]
pub mod stm32h757cm4;

#[cfg(feature = "stm32h757cm7")]
pub mod stm32h757cm7;

#[cfg(feature = "stm32h7a3")]
pub mod stm32h7a3;

#[cfg(feature = "stm32h7b0")]
pub mod stm32h7b0;

#[cfg(feature = "stm32h7b3")]
pub mod stm32h7b3;

#[cfg(feature = "stm32h7r")]
pub mod stm32h7r;

#[cfg(feature = "stm32h7s")]
pub mod stm32h7s;

