// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use gh_bofh_lib::MODERN;

#[test]
fn test_binary_output_modern() {
    let cmd = Command::cargo_bin("gh-bofh").unwrap().output().unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(!MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_flag_modern() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .args(["--type", "modern"])
        .output()
        .unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_flag_short_modern() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .arg("-m")
        .output()
        .unwrap();
    println!("{:?}", cmd);
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_env_var_modern() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .env("EXCUSE_TYPE", "modern")
        .output()
        .unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}
