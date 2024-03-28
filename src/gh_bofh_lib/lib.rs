// Copyright (c) 2024
// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

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
