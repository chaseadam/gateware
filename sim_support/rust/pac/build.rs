use std::fs;
use std::io::Write;

extern crate svd2rust;
fn main() {
    println!("cargo:rerun-if-changed=../../../target/soc.svd");
    let svd = String::from_utf8(include_bytes!("../../../target/soc.svd").to_vec())
        .expect("svd file wasn't valid utf8");
    let pac_file = svd2rust::generate(&svd, svd2rust::Target::RISCV, false)
        .expect("couldn't generate file with svd2rust");

    // This appears to be what they do inside svd2rust:main.rs
    let lib_rs = pac_file.lib_rs.replace("] ", "]\n");

    // These strings are generated by svd2rust.  Remove them to silence warnings.
    let bad_strings = [
        "# ! [ deny ( legacy_directory_ownership ) ]",
        "# ! [ deny ( plugin_as_library ) ]",
        "# ! [ deny ( safe_extern_statics ) ]",
        "# ! [ deny ( unions_with_drop_fields ) ]",
        "#![no_main]",
        "# ! [ no_std ]",
    ];

    let mut out_file = fs::File::create("src/pac.rs").expect("couldn't open output file");
    for line in lib_rs.lines() {
        if bad_strings.contains(&line) {
            println!("Found bad string, skipping");
            continue;
        }
        out_file
            .write(line.as_bytes())
            .expect("couldn't write line to pac.rs");
        out_file
            .write(b"\n")
            .expect("couldn't write line ending to pac.rs");
    }
}
