use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cicvverify() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["cicvverify"]) 
        .current_dir("exercises")
        .assert()
        .success();
}
