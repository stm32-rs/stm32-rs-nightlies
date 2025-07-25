# stm32wb0
This crate provides an autogenerated API for access to STM32WB0 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[main repo]: https://github.com/stm32-rs/stm32-rs
[documentation]: https://docs.rs/stm32wb0/latest/stm32wb0/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.stm32wb0]
version = "0.16.0"
features = ["stm32wb05"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

In your code:

```rust
use stm32wb0::stm32wb05;

let mut peripherals = stm32wb05::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| stm32wb05 | STM32WB05 | [RM0529](https://www.st.com/resource/en/reference_manual/rm0529-stm32wb05xz-ultralow-power-wireless-32bit-mcu-armbased-cortexm0-with-bluetooth-low-energy-and-24-ghz-radio-solution-stmicroelectronics.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32wb0-series.html) |
| stm32wb06-07 | STM32WB06, STM32WB07 | [RM0530](https://www.st.com/resource/en/reference_manual/rm0530--stm32wb07xc-and-stm32wb06xc-ultralow-power-wireless-32bit-mcus-armbased-cortexm0-with-bluetooth-le-and-24-ghz-radio-solution-stmicroelectronics.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32wb0-series.html) |
| stm32wb09 | STM32WB09 | [RM0505](https://www.st.com/resource/en/reference_manual/rm0505-ultralow-power-wireless-32bit-mcu-armbased-cortexm0-with-bluetooth-le-and-24-ghz-radio-solution-stmicroelectronics.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32wb0-series.html) |
