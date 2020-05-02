# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- [breaking-change] Renamed `is_busy()` and `is_running()` methods `busy()` and `running()`
  due to Rust naming conventions.
- Implement trait from [`rtcc`] crate.
- Changed `get_datetime()` and `set_datetime()` parameter from `DateTime`
  to `chrono::NaiveDateTime`.

### Added
- Methods to set and get date and time using `chrono::NaiveDate` and `chrono::NaiveTime`:
    - `get_time()`
    - `set_time()`
    - `get_date()`
    - `set_date()`
- [`chrono`] (through [`rtcc`]) dependency.

### Removed
- `DateTime` data structure was replaced by `chrono::NaiveDateTime`.

## [0.2.0] - 2018-11-16

### Added
- Support for configuration of alarms 1 and 2.

### Changed
- [breaking-change] `clear_has_been_stopped_flag()` always sets the value of the status register.

## 0.1.0 - 2018-10-31

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.

[`chrono`]: https://crates.io/crates/chrono
[`rtcc`]: https://crates.io/crates/rtcc

[Unreleased]: https://github.com/eldruin/ds323x-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/ds323x-rs/compare/v0.1.0...v0.2.0