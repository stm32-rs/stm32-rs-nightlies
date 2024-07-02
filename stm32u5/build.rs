use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32U535").is_some() {
            "src/stm32u535/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U545").is_some() {
            "src/stm32u545/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U575").is_some() {
            "src/stm32u575/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U585").is_some() {
            "src/stm32u585/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U595").is_some() {
            "src/stm32u595/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U599").is_some() {
            "src/stm32u599/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U5A5").is_some() {
            "src/stm32u5a5/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32U5A9").is_some() {
            "src/stm32u5a9/device.x"
        } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
