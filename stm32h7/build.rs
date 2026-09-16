use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let devices = ["stm32h723", "stm32h725", "stm32h730", "stm32h733", "stm32h735", "stm32h742", "stm32h743", "stm32h743v", "stm32h745cm4", "stm32h745cm7", "stm32h747cm4", "stm32h747cm7", "stm32h750", "stm32h753", "stm32h753v", "stm32h755cm4", "stm32h755cm7", "stm32h757cm4", "stm32h757cm7", "stm32h7a3", "stm32h7b0", "stm32h7b3", "stm32h7r", "stm32h7s"];
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
