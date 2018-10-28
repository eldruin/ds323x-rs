# Rust DS3231, DS3232 and DS3234 Extremely Accurate Real-Time Clock Driver [![crates.io](https://img.shields.io/crates/v/ds323x.svg)](https://crates.io/crates/ds323x) [![Docs](https://docs.rs/ds323x/badge.svg)](https://docs.rs/ds323x) [![Build Status](https://travis-ci.org/eldruin/ds323x-rs.svg?branch=master)](https://travis-ci.org/eldruin/ds323x-rs)

This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
extremely accurate real-time clocks, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read/write the seconds.
- Read/write the minutes.
- Read/write the hours in 24h or AM/PM format.
- Read/write the weekday.
- Read/write the day.
- Read/write the month.

## The devices

This driver is compatible with the DS3231 and DS3232 I2C devices and the
DS3234 SPI device.

### DS3231
The DS3231 is a low-cost, extremely accurate I2C real-time clock (RTC) with
an integrated temperature-compensated crystal oscillator (TCXO) and crystal.

The device incorporates a battery input, and maintains accurate timekeeping
when main power to the device is interrupted. The integration of the
crystal resonator enhances the long-term accuracy of the device as well as
reduces the piece-part count in a manufacturing line.
The DS3231 is available in commercial and industrial temperature ranges,
and is offered in a 16-pin, 300-mil SO package.

The RTC maintains seconds, minutes, hours, day, date, month, and year
information. The date at the end of the month is automatically adjusted for
months with fewer than 31 days, including corrections for leap year. The
clock operates in either the 24-hour or 12-hour format with an AM/PM
indicator. Two programmable time-of-day alarms and a programmable
square-wave output are provided. Address and data are transferred serially
through an I2C bidirectional bus.

A precision temperature-compensated voltage reference and comparator
circuit monitors the status of VCC to detect power failures, to provide a
reset output, and to automatically switch to the backup supply when
necessary. Additionally, the RST pin is monitored as a pushbutton
input for generating a μP reset.

### DS3232
The DS3232 is a low-cost temperature-compensated crystal oscillator (TCXO)
with a very accurate, temperature-compensated, integrated real-time clock
(RTC) and 236 bytes of battery-backed SRAM.

Additionally, the DS3232 incorporates a battery input and maintains
accurate timekeeping when main power to the device is interrupted. The
integration of the crystal resonator enhances the long-term accuracy of the
device as well as reduces the piece-part count in a manufacturing line.
The DS3232 is available in commercial and industrial temperature ranges,
and is offered in an industry-standard 20-pin, 300-mil SO package.

The RTC maintains seconds, minutes, hours, day, date, month, and year
information. The date at the end of the month is automatically adjusted for
months with fewer than 31 days, including corrections for leap year. The
clock operates in either the 24-hour or 12-hour format with an AM/PM
indicator. Two programmable time-of-day alarms and a programmable
square-wave output are provided. Address and data are transferred serially
through an I2C bidirectional bus.

A precision temperature-compensated voltage reference and comparator
circuit monitors the status of VCC to detect power failures, to provide a
reset output, and to automatically switch to the backup supply when
necessary. Additionally, the RST pin is monitored as a pushbutton input for
generating a μP reset.

### DS3234
The DS3234 is a low-cost, extremely accurate SPI bus real-time clock (RTC)
with an integrated temperature-compensated crystal oscillator (TCXO) and
crystal.

The DS3234 incorporates a precision, temperature-compensated voltage
reference and comparator circuit to monitor VCC. When VCC drops below the
power-fail voltage (VPF), the device asserts the RST output and also
disables read and write access to the part when VCC drops below both VPF
and VBAT. The RST pin is monitored as a pushbutton input for generating a
μP reset. The device switches to the backup supply input and maintains
accurate timekeeping when main power to the device is interrupted.
The integration of the crystal resonator enhances the long-term accuracy of
the device as well as reduces the piece-part count in a manufacturing line.
The DS3234 is available in commercial and industrial temperature ranges,
and is offered in an industry-standard 300-mil, 20-pin SO package.

The DS3234 also integrates 256 bytes of battery-backed SRAM. In the event
of main power loss, the contents of the memory are maintained by the power
source connected to the V BAT pin. The RTC maintains seconds, minutes,
hours, day, date, month, and year information. The date at the end of the
month is automatically adjusted for months with fewer than 31 days,
including corrections for leap year. The clock operates in either the
24-hour or 12-hour format with AM/PM indicator. Two programmable
time-of-day alarms and a programmable square-wave output are provided.
Address and data are transferred serially by an SPI bidirectional bus.

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

