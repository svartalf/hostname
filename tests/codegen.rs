use std::fs;
use std::process::Command;

use windows_bindgen::Bindgen;

#[test]
fn gen_bindings() {
    let output = "src/windows/bindings.rs";
    let existing = fs::read_to_string(output).unwrap();

    Bindgen::new()
        .filter("ComputerNamePhysicalDnsHostname")
        .filter("GetComputerNameExW")
        .filter("SetComputerNameExW")
        .output("src/windows/bindings.rs")
        .flat()
        .sys()
        .dead_code()
        .write();

    let out = Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(output)
        .output()
        .unwrap();
    assert!(out.status.success());

    // Check the output is the same as before.
    // Depending on the git configuration the file may have been checked out with `\r\n` newlines or
    // with `\n`. Compare line-by-line to ignore this difference.
    let mut new = fs::read_to_string(output).unwrap();
    if existing.contains("\r\n") && !new.contains("\r\n") {
        new = new.replace("\n", "\r\n");
    } else if !existing.contains("\r\n") && new.contains("\r\n") {
        new = new.replace("\r\n", "\n");
    }

    similar_asserts::assert_eq!(existing, new);
}
