// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Performance benchmarks for the `gh_bofh_lib` crate.
//!
//! These benchmarks measure the performance of the `random_classic` and
//! `random_modern` functions from the `gh_bofh_lib` crate. The benchmarks are
//! designed to help identify performance bottlenecks and ensure that the
//! functions perform efficiently under various conditions.

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use gh_bofh_lib::{
    random_classic,
    random_modern,
};

/// Benchmark the performance of the `random_classic` function.
///
/// This function measures the time it takes to generate classic excuses using
/// the `random_classic` function from the `gh_bofh_lib` crate.
///
/// # Arguments
///
/// * `c` - A mutable reference to a `Criterion` struct used for benchmarking.
fn benchmark_random_classic(c: &mut Criterion) {
    c.bench_function("random_classic", |b| b.iter(random_classic));
}

/// Benchmark the performance of the `random_modern` function.
///
/// This function measures the time it takes to generate modern excuses using
/// the `random_modern` function from the `gh_bofh_lib` crate.
///
/// # Arguments
///
/// * `c` - A mutable reference to a `Criterion` struct used for benchmarking.
fn benchmark_random_modern(c: &mut Criterion) {
    c.bench_function("random_modern", |b| b.iter(random_modern));
}

criterion_group!(benches, benchmark_random_classic, benchmark_random_modern);
criterion_main!(benches);
