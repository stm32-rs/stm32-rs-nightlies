//! Peripheral access API for STM32F1 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.19.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32F1 devices; for the complete list please
//! see:
//! [stm32f1](https://crates.io/crates/stm32f1)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32f100")]
pub mod stm32f100;

#[cfg(feature = "stm32f101")]
pub mod stm32f101;

#[cfg(feature = "stm32f102")]
pub mod stm32f102;

#[cfg(feature = "stm32f103")]
pub mod stm32f103;

#[cfg(feature = "stm32f107")]
pub mod stm32f107;

