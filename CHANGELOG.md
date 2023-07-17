# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2023-07-17

### Fixed
- Fixed `set_day` method. See: [PR #9](https://github.com/eldruin/ds323x-rs/pull/9)

### Changed
- Raised MSRV to version 1.60.0

## [0.5.0] - 2022-02-21

### Changed

- [breaking-change] Update `rtcc` to version 0.3.
- [breaking-change] Remove `get_` from all public method names to comply with the Rust API guidelines.
- Raise MSRV to version 1.35.0

## [0.4.0] - 2021-05-22

### Changed
- [breaking-change] Return `Error::InvalidDeviceState` if it was not possible to read the
  date and/or time from the device because the state of the device corresponds to
  an invalid date and/or time.

## [0.3.2] - 2021-02-22

### Fixed
- Day bounds on the `set_day()` method. Thanks to @jamesmunns. See:
  [PR #5](https://github.com/eldruin/ds323x-rs/pull/5)

## [0.3.1] - 2020-07-10

### Added
- Added methods to set alarms 1 and 2 with a `chrono::NaiveTime`: `set_alarm1_hms()`
  and `set_alarm2_hm()`.

### Changed
- Changed alarm setting methods to automatically correct invalid values to irrelevant
  input parameters due to the selected matching strategy.

## [0.3.0] - 2020-05-02

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

[Unreleased]: https://github.com/eldruin/ds323x-rs/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/eldruin/ds323x-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/eldruin/ds323x-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/eldruin/ds323x-rs/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/eldruin/ds323x-rs/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/eldruin/ds323x-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/eldruin/ds323x-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/eldruin/ds323x-rs/compare/v0.1.0...v0.2.0