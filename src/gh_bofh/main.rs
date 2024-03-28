// Copyright (c) 2024
// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod cli;

use clap::Parser;
use cli::{
    Cli,
    ExcuseType,
};
use gh_bofh_lib::{
    random_classic,
    random_modern,
};

fn main() {
    let arguments = Cli::parse();
    match arguments.excuse_type {
        ExcuseType::Classic => println!("{}", random_classic()),
        ExcuseType::Modern => println!("{}", random_modern()),
    }
}
