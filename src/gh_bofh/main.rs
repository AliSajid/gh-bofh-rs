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
    let chosen_type = process_choice(&arguments);
    match chosen_type {
        ExcuseType::Classic => println!("{}", random_classic()),
        ExcuseType::Modern => println!("{}", random_modern()),
    }
}

fn process_choice(arguments: &Cli) -> ExcuseType {
    if arguments.classic {
        ExcuseType::Classic
    } else if arguments.modern {
        ExcuseType::Modern
    } else {
        arguments.excuse_type
    }
}
