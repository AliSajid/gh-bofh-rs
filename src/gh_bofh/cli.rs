// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2024
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// * http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a
//   copy
// * of this software and associated documentation files (the "Software"), to
//   deal
// * in the Software without restriction, including without limitation the
//   rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in
//   all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//   FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//   THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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
