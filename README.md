# Rust DS3231, DS3232 and DS3234 Extremely Accurate Real-Time Clock Driver [![crates.io](https://img.shields.io/crates/v/ds323x.svg)](https://crates.io/crates/ds323x) [![Docs](https://docs.rs/ds323x/badge.svg)](https://docs.rs/ds323x) [![Build Status](https://travis-ci.org/eldruin/ds323x-rs.svg?branch=master)](https://travis-ci.org/eldruin/ds323x-rs)

This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
extremely accurate real-time clocks, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read/write the seconds.

## The devices

This driver is compatible with the DS3231 and DS3232 I2C devices and the
DS3234 SPI device.

### DS3231
TODO

### DS3232
TODO

### DS3234
TODO

Datasheets:
- [DS3231](https://datasheets.maximintegrated.com/en/ds/DS3231.pdf)
- [DS3232](https://datasheets.maximintegrated.com/en/ds/DS3232.pdf)
- [DS3234](https://datasheets.maximintegrated.com/en/ds/DS3234.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

