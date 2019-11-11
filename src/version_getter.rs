// cargo search base64 --limit 1

use std::iter::FromIterator;
use std::process::Command;

pub fn get_crate_version(name: &str) -> Option<String> {
    let result = Command::new("cargo")
        .args(vec!["search", name, "--limit", "1"])
        .output()
        .unwrap();

    assert!(result.status.success());

    let output = String::from_utf8(result.stdout).unwrap();
    let line = output.lines().nth(0).unwrap();
    let parts = Vec::from_iter(line.split_ascii_whitespace());

    let found_name = parts[0].to_string();
    let version = parts[2].replace("\"", "");

    if name == found_name {
        Some(version)
    } else {
        None
    }
}
