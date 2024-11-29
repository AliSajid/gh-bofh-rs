// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use gh_bofh_lib::{
    random_modern,
    MODERN,
};

#[test]
fn test_modern_excuses() {
    let mut excuses = HashSet::new();
    for _ in 0..200 {
        let excuse = random_modern();
        assert!(!excuse.is_empty());
        assert_ne!(
            excuse,
            "Excuse engine not initialized. Please try again later."
        );
        excuses.insert(excuse);
    }

    assert!(excuses.len() > 1);
    assert!(excuses.len() <= 105);

    assert!(excuses.iter().all(|excuse| MODERN.contains(excuse)));
}
