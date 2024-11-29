// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use gh_bofh_lib::CLASSIC;

#[test]
fn test_binary_plain_default() {
    let mut cmd = Command::cargo_bin("gh-bofh").unwrap();
    cmd.assert().success();
}

#[test]
fn test_binary_output_classic() {
    let cmd = Command::cargo_bin("gh-bofh").unwrap().output().unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_flag_classic() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .args(["--type", "classic"])
        .output()
        .unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_flag_short_classic() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .arg("-c")
        .output()
        .unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}

#[test]
fn test_binary_output_env_var_classic() {
    let cmd = Command::cargo_bin("gh-bofh")
        .unwrap()
        .env("EXCUSE_TYPE", "classic")
        .output()
        .unwrap();
    assert!(cmd.status.success());
    assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
    assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
    assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
}
