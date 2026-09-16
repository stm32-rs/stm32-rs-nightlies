use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let devices = ["stm32g030", "stm32g031", "stm32g041", "stm32g050", "stm32g051", "stm32g061", "stm32g070", "stm32g071", "stm32g081", "stm32g0b0", "stm32g0b1", "stm32g0c1"];
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
