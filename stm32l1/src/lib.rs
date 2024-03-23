//! Peripheral access API for STM32L1 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.32.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32L1 devices; for the complete list please
//! see:
//! [stm32l1](https://crates.io/crates/stm32l1)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32l100")]
pub mod stm32l100;

#[cfg(feature = "stm32l151")]
pub mod stm32l151;

#[cfg(feature = "stm32l162")]
pub mod stm32l162;

