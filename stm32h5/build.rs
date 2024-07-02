use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32H503").is_some() {
            "src/stm32h503/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H562").is_some() {
            "src/stm32h562/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H563").is_some() {
            "src/stm32h563/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32H573").is_some() {
            "src/stm32h573/device.x"
        } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
