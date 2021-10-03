use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32F100").is_some() {
            "src/stm32f100/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F101").is_some() {
            "src/stm32f101/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F102").is_some() {
            "src/stm32f102/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F103").is_some() {
            "src/stm32f103/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32F107").is_some() {
            "src/stm32f107/device.x"
        } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
