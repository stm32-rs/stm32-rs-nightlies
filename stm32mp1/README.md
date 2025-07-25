# stm32mp1
This crate provides an autogenerated API for access to STM32MP1 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[main repo]: https://github.com/stm32-rs/stm32-rs
[documentation]: https://docs.rs/stm32mp1/latest/stm32mp1/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.stm32mp1]
version = "0.16.0"
features = ["stm32mp153"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

In your code:

```rust
use stm32mp1::stm32mp153;

let mut peripherals = stm32mp153::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| stm32mp153 | STM32MP153 | [RM0442](https://www.st.com/resource/en/reference_manual/rm0442-stm32mp153-advanced-armbased-32bit-mpus-stmicroelectronics.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32mp153.html) |
| stm32mp157 | STM32MP157 | [RM0436](https://www.st.com/resource/en/reference_manual/dm00366355-stm32mp157-advanced-armbased-32bit-mpus-stmicroelectronics.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32mp157.html) |
