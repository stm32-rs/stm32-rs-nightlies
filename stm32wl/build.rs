use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_STM32WL5X_CM0P").is_some() {
            "src/stm32wl5x_cm0p/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32WL5X_CM4").is_some() {
            "src/stm32wl5x_cm4/device.x"
        } else if env::var_os("CARGO_FEATURE_STM32WLE5").is_some() {
            "src/stm32wle5/device.x"
        } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
