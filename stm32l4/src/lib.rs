//! Peripheral access API for STM32L4 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.32.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32L4 devices; for the complete list please
//! see:
//! [stm32l4](https://crates.io/crates/stm32l4)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32l412")]
pub mod stm32l412;

#[cfg(feature = "stm32l4p5")]
pub mod stm32l4p5;

#[cfg(feature = "stm32l4r5")]
pub mod stm32l4r5;

#[cfg(feature = "stm32l4r9")]
pub mod stm32l4r9;

#[cfg(feature = "stm32l4x1")]
pub mod stm32l4x1;

#[cfg(feature = "stm32l4x2")]
pub mod stm32l4x2;

#[cfg(feature = "stm32l4x3")]
pub mod stm32l4x3;

#[cfg(feature = "stm32l4x5")]
pub mod stm32l4x5;

#[cfg(feature = "stm32l4x6")]
pub mod stm32l4x6;

