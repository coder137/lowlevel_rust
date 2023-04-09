use std::{env, path::PathBuf};

fn parse_header(input_filename: &str, output_path: &PathBuf) {
    let target = env::var_os("TARGET").expect("Expects environment variable TARGET");
    println!("Target: {:?}", target);

    // Convert c header to rust using bindgen
    let bindings = bindgen::Builder::default()
        .header(input_filename)
        .clang_arg("-Idevice")
        .clang_arg("-Idevice/cmsis")
        .clang_arg("-Idevice/controller")
        .clang_arg(format!("--target={}", target.to_str().unwrap()))
        .layout_tests(false)
        .use_core()
        .wrap_unsafe_ops(true)
        .translate_enum_integer_types(true)
        .explicit_padding(false)
        .generate_block(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write to output
    bindings
        .write_to_file(output_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    // Parse headers only for valid on-target microcontrollers
    let target = env::var_os("TARGET").expect("Expects environment variable TARGET");
    let should_parse = match target.to_str().unwrap() {
        "thumbv7em-none-eabihf" => true,
        _ => false,
    };

    // TODO, Make this user configurable to support multiple microcontroller formats
    // NOTE, This controller.rs contains both
    // - Architecture considerations (ARM specific peripherals)
    // - Microcontroller considerations (STM32 specific peripherals)
    const PARSE_INPUT_FILE: &str = "device/controller/stm32l475xx.h";
    const OUTPUT_FILE: &str = "src/stm32l475xx/controller.rs";
    if should_parse {
        parse_header(PARSE_INPUT_FILE, &PathBuf::from(OUTPUT_FILE));
    }
}
