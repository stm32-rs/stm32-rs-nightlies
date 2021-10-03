//! Peripheral access API for STM32F7 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.19.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32F7 devices; for the complete list please
//! see:
//! [stm32f7](https://crates.io/crates/stm32f7)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32f730")]
pub mod stm32f730;

#[cfg(feature = "stm32f745")]
pub mod stm32f745;

#[cfg(feature = "stm32f750")]
pub mod stm32f750;

#[cfg(feature = "stm32f765")]
pub mod stm32f765;

#[cfg(feature = "stm32f7x2")]
pub mod stm32f7x2;

#[cfg(feature = "stm32f7x3")]
pub mod stm32f7x3;

#[cfg(feature = "stm32f7x6")]
pub mod stm32f7x6;

#[cfg(feature = "stm32f7x7")]
pub mod stm32f7x7;

#[cfg(feature = "stm32f7x9")]
pub mod stm32f7x9;

