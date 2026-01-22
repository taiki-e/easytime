# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Add `Duration::{from_mins,from_hours}`.
  They are based on the same methods in the standard library that stabilized in Rust 1.91.

- Add `Duration::from_nanos_u128`.
  They are based on the same methods in the standard library that stabilized in Rust 1.93.

- Optimize `Duration::from_secs_{f32,f64}`.

- Enable [release immutability](https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/immutable-releases).

## [0.2.7] - 2024-03-05

- Make `Instant::{duration_since, elapsed, sub}` saturating to follow the [upstream change](https://github.com/rust-lang/rust/pull/89926).

- Add `#[must_use]` to follow upstream changes.

## [0.2.6] - 2023-08-26

- Increase the minimum supported Rust version from Rust 1.34 to Rust 1.58.

## [0.2.5] - 2023-07-27

- Remove dependency on `const_fn` crate.

## [0.2.4] - 2022-09-05

- Make `Duration::new` const function on Rust 1.58+.

- Make `Duration::is_zero` const function on Rust 1.46+. Previously, this was const function on Rust 1.53+.

- Documentation improvements.

## [0.2.3] - 2022-06-30

- Add `{Duration, Instant}::NONE` constants. ([#27](https://github.com/taiki-e/easytime/pull/27), thanks @dhardy)

## [0.2.2] - 2021-07-24

- Add `Duration::{ZERO, MAX}` constants and `Duration::is_zero` method. ([#25](https://github.com/taiki-e/easytime/pull/25))
  They are based on the same methods in the standard library that stabilized in Rust 1.53.

## [0.2.1] - 2021-04-06

- Apply `doc(cfg(...))` on feature gated APIs. ([#23](https://github.com/taiki-e/easytime/pull/23))

## [0.2.0] - 2021-01-03

- Add `Duration::{as_secs_f64, as_secs_f32, from_secs_f64, from_secs_f32, mul_f64, mul_f32, div_f64, div_f32}` methods. ([#21](https://github.com/taiki-e/easytime/pull/21))
  They are based on [`duration_float`](https://github.com/rust-lang/rust/issues/54361) feature of the standard library that stabilized in Rust 1.38.

- Make `Duration::{as_secs, subsec_millis, subsec_micros, subsec_nanos, as_millis, as_micros, as_nanos, is_some, is_none, unwrap_or}` const function on rustc 1.46+. ([#19](https://github.com/taiki-e/easytime/pull/19))

- Make `Instant::{is_some, is_none, unwrap_or}` const function on Rust 1.46+. ([#19](https://github.com/taiki-e/easytime/pull/19))

- Implement `TryFrom` for `Instant` and `Duration`. ([#10](https://github.com/taiki-e/easytime/pull/10))
  With this change, the minimum required version of `easytime` without default features goes up to Rust 1.34.
  (The minimum required version of the default feature has not changed.)

- Implement `PartialEq<std::time::Duration>` and `PartialOrd<std::time::Duration>` for `Duration`. ([#22](https://github.com/taiki-e/easytime/pull/22))

- Implement `PartialEq<std::time::Instant>` and `PartialOrd<std::time::Instant>` for `Instant`. ([#22](https://github.com/taiki-e/easytime/pull/22))

- Implement `From<Option<std::time::Duration>>` for `Duration`. ([#22](https://github.com/taiki-e/easytime/pull/22))

- Implement `From<Option<std::time::Instant>>` for `Instant`. ([#22](https://github.com/taiki-e/easytime/pull/22))

- Changed the `Debug` implementation of `Duration` to display the same as the result of `std::time::Duration::checked_*`. ([#9](https://github.com/taiki-e/easytime/pull/9))

- Documentation improvements.

## [0.1.2] - 2019-03-01

- Remove "This example is not tested" warning in documentation example.

## [0.1.1] - 2019-02-23

- Add the `unwrap_or_else` method to `Instant` and `Duration`.

## [0.1.0] - 2019-02-19

Initial release

[Unreleased]: https://github.com/taiki-e/easytime/compare/v0.2.7...HEAD
[0.2.7]: https://github.com/taiki-e/easytime/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/taiki-e/easytime/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/taiki-e/easytime/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/taiki-e/easytime/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/easytime/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/easytime/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/easytime/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/easytime/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/taiki-e/easytime/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/easytime/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/easytime/releases/tag/v0.1.0
