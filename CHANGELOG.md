# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

- Add `Duration::{as_secs_f64, as_secs_f32, from_secs_f64, from_secs_f32,
  mul_f64, mul_f32, div_f64, div_f32}` methods.
  They are based on [`duration_float`](https://github.com/rust-lang/rust/issues/54361)
  feature of the standard library that stabilized on Rust 1.38.

- Make `Duration::{as_secs, subsec_millis, subsec_micros, subsec_nanos,
  as_millis, as_micros, as_nanos, is_some, is_none, unwrap_or}` const function
  on rustc 1.46+.

- Make `Instant::{is_some, is_none, unwrap_or}` const function on Rust 1.46+.

- Implement `TryFrom` for `easytime::Instant` and `easytime::Duration`. With
  this change, the minimum required version of `easytime` with `--no-default-features` goes up to Rust 1.34 (the minimum required version of the default feature has not changed).

- Changed the `Debug` implementation of `easytime::Duration` to display the
  same as the result of `std::time::Duration::checked_*`.

- Documentation improvements.

## [0.1.2] - 2019-03-01

- Remove "This example is not tested" warning in documentation example.

## [0.1.1] - 2019-02-23

- Add the `unwrap_or_else` method to `easytime::Instant` and
  `easytime::Duration`.

## [0.1.0] - 2019-02-19

Initial release

[Unreleased]: https://github.com/taiki-e/easytime/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/taiki-e/easytime/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/easytime/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/easytime/releases/tag/v0.1.0
