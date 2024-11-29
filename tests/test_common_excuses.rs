// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use gh_bofh_lib::{
    random_classic,
    CLASSIC,
};

#[test]
fn test_common_excuses() {
    let mut excuses = HashSet::new();
    for _ in 0..500 {
        let excuse = random_classic();
        assert!(!excuse.is_empty());
        assert_ne!(excuse, "No excuse found, try again later");
        excuses.insert(excuse);
    }

    assert!(excuses.len() > 1);
    assert!(excuses.len() <= 467);

    assert!(excuses.iter().all(|excuse| CLASSIC.contains(excuse)));
}
