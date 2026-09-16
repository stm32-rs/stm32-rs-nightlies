use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let devices = ["stm32f401", "stm32f405", "stm32f407", "stm32f410", "stm32f411", "stm32f412", "stm32f413", "stm32f415", "stm32f417", "stm32f427", "stm32f429", "stm32f437", "stm32f439", "stm32f446", "stm32f469"];
        let mut device_file = None;
        for &d in &devices {
            if env::var_os(&format!("CARGO_FEATURE_{}", d.to_uppercase())).is_some() {
                device_file = Some(format!("src/{d}/device.x"));
                break;
            }
        }
        if let Some(device_file) = device_file {
            fs::copy(&device_file, out.join("device.x")).unwrap();
            println!("cargo:rerun-if-changed={device_file}");
        } else {
            panic!("No device features selected. Avaliable device features are: {devices:?}");
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
