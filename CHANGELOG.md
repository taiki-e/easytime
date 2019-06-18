# Unreleased

* Implement `TryFrom` for `easytime::Instant` and `easytime::Duration`. With this change, the minimum required version of `easytime` with `--no-default-features` goes up to Rust 1.34 (the minimum required version of the default feature has not changed).

* Changed the `Debug` implementation of `easytime::Duration` to display the same as the result of `std::time::Duration::checked_*`.

# 0.1.2 - 2019-03-01

* Remove "This example is not tested" warning in documentation example.

# 0.1.1 - 2019-02-23

* Add the `unwrap_or_else` method to `easytime::Instant` and `easytime::Duration`.

# 0.1.0 - 2019-02-19

Initial release
