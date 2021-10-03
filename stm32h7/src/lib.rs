//! Peripheral access API for STM32H7 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.19.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api)
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

#[cfg(feature = "stm32h735")]
pub mod stm32h735;

#[cfg(feature = "stm32h743")]
pub mod stm32h743;

#[cfg(feature = "stm32h743v")]
pub mod stm32h743v;

#[cfg(feature = "stm32h747cm4")]
pub mod stm32h747cm4;

#[cfg(feature = "stm32h747cm7")]
pub mod stm32h747cm7;

#[cfg(feature = "stm32h753")]
pub mod stm32h753;

#[cfg(feature = "stm32h753v")]
pub mod stm32h753v;

#[cfg(feature = "stm32h7b3")]
pub mod stm32h7b3;

