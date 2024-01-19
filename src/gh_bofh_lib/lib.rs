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
mod excuses;

use excuses::{
    CLASSIC,
    MODERN,
};
use rand::seq::SliceRandom;

type ClassicExcuse = &'static str;
type ModernExcuse = &'static str;

/// Returns a random classic excuse
///
/// This function returns a random BOFH excuse from the classic list.
///
/// # Examples
///
/// ```
/// use gh_bofh_lib::random_classic;
///
/// let excuse = random_classic();
/// println!("{}", excuse);
/// ```
#[must_use]
pub fn random_classic() -> ClassicExcuse {
    CLASSIC
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"No excuse found, try again later")
}

/// Returns a random modern excuse
///
/// This function returns a random BOFH excuse from the modern list.
///
/// # Examples
///
/// ```
/// use gh_bofh_lib::random_modern;
///
/// let excuse = random_modern();
///
/// println!("{}", excuse);
/// ```
#[must_use]
pub fn random_modern() -> ModernExcuse {
    MODERN
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Excuse engine not initialized. Please try again later.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_classic() {
        let excuse: ClassicExcuse = random_classic();
        assert_ne!(excuse, "No excuse found, try again later");
    }

    #[test]
    fn test_random_modern() {
        let excuse: ModernExcuse = random_modern();
        assert_ne!(excuse, "Excuse engine not initialized");
    }
}
