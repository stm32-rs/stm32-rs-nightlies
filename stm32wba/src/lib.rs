//! Peripheral access API for STM32WBA microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.37.1)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.37.1/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32WBA devices; for the complete list please
//! see:
//! [stm32wba](https://crates.io/crates/stm32wba)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32wba50")]
pub mod stm32wba50;

#[cfg(feature = "stm32wba52")]
pub mod stm32wba52;

#[cfg(feature = "stm32wba54")]
pub mod stm32wba54;

#[cfg(feature = "stm32wba55")]
pub mod stm32wba55;

