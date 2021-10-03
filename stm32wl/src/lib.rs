//! Peripheral access API for STM32WL microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.19.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/stm32-rs/stm32-rs)
//!
//! This crate supports all STM32WL devices; for the complete list please
//! see:
//! [stm32wl](https://crates.io/crates/stm32wl)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [stm32-rs Device Coverage](https://stm32-rs.github.io/stm32-rs/)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "stm32wl5x_cm0p")]
pub mod stm32wl5x_cm0p;

#[cfg(feature = "stm32wl5x_cm4")]
pub mod stm32wl5x_cm4;

#[cfg(feature = "stm32wle5")]
pub mod stm32wle5;

