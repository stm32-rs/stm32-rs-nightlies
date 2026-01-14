use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let devices = ["stm32f301", "stm32f302", "stm32f303", "stm32f373", "stm32f3x4"];
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
