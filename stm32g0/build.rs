use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32G030").is_some() {
            "src/stm32g030/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G031").is_some() {
            "src/stm32g031/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G041").is_some() {
            "src/stm32g041/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G050").is_some() {
            "src/stm32g050/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G051").is_some() {
            "src/stm32g051/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G061").is_some() {
            "src/stm32g061/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G070").is_some() {
            "src/stm32g070/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G071").is_some() {
            "src/stm32g071/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G081").is_some() {
            "src/stm32g081/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G0B0").is_some() {
            "src/stm32g0b0/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G0B1").is_some() {
            "src/stm32g0b1/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32G0C1").is_some() {
            "src/stm32g0c1/device.x"
        } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
