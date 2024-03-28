// Copyright (c) 2024
// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// CLI Specification for clap

use clap::{
    arg,
    Parser,
    ValueEnum,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ExcuseType {
    Classic,
    Modern,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Generates a random BOFH excuse. The excuse type can be specified with the \
                  -t/--type flag. The default is classic, which generates a 90s style BOFH \
                  excuse. You can also specify modern, which generates a more modern BOFH excuse."
)]
pub struct Cli {
    /// The type of excuse to generate: classic or modern
    ///
    /// The default is classic, which generates a 90s style BOFH excuse. You can
    /// also specify modern, which generates a more modern BOFH excuse.
    #[clap(
        short = 't',
        long = "type",
        default_value = "classic",
        env = "EXCUSE_TYPE",
        value_name = "TYPE"
    )]
    #[arg(value_enum, group = "type")]
    pub excuse_type: ExcuseType,

    /// Generate a classic BOFH excuse
    ///
    /// Generates a 90s style BOFH excuse.
    #[arg(short = 'c', long = "classic", group = "type")]
    pub classic: bool,

    /// Generate a modern BOFH excuse
    ///
    /// Generates a more modern BOFH excuse.
    #[arg(short = 'm', long = "modern", group = "type")]
    pub modern: bool,
}
