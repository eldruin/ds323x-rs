# Rust DS3231, DS3232 and DS3234 Extremely Accurate Real-Time Clock Driver

[![crates.io](https://img.shields.io/crates/v/ds323x.svg)](https://crates.io/crates/ds323x)
[![Docs](https://docs.rs/ds323x/badge.svg)](https://docs.rs/ds323x)
[![Build Status](https://travis-ci.org/eldruin/ds323x-rs.svg?branch=master)](https://travis-ci.org/eldruin/ds323x-rs)
[![Coverage Status](https://coveralls.io/repos/eldruin/ds323x-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/ds323x-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
extremely accurate real-time clocks, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read and set date and time in 12-hour and 24-hour format. See: `get_datetime`.
- Read and set date and time individual elements. For example, see: `get_year`.
- Enable and disable the real-time clock. See: `enable`.
- Read the busy status. See `is_busy`.
- Read whether the oscillator is or has been stopped. See `has_been_stopped`.
- Clear the has-been-stopped flag. See `clear_has_been_stopped_flag`.
- Set and read the aging offset. See `set_aging_offset`.
- Select the function of the INT/SQW output pin. See `use_int_sqw_output_as_interrupt`.
- Alarms:
    - Set alarms 1 and 2 with several matching policies. See `set_alarm1_day`.
    - Read whether alarms 1 or 2 have matched. See `has_alarm1_matched`.
    - Clear flag indicating that alarms 1 or 2 have matched. See `clear_alarm1_matched_flag`.
    - Enable and disable alarms 1 and 2 interrupt generation. See `enable_alarm1_interrupts`.
- Wave generation:
    - Enable and disable the square-wave generation. See `enable_square_wave`.
    - Select the square-wave frequency. See `set_square_wave_frequency`.
    - Enable and disable the 32kHz output. See `enable_32khz_output`.
    - Enable and disable the 32kHz output when battery powered. See `enable_32khz_output_on_battery`.
- Temperature conversion:
    - Read the temperature. See `get_temperature`.
    - Force a temperature conversion and time compensation. See `convert_temperature`.
    - Set the temperature conversion rate. See `set_temperature_conversion_rate`.
    - Enable and disable the temperature conversions when battery-powered. See `enable_temperature_conversions_on_battery`.

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

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.
In the following example an instance of the device DS3231 will be created.
Other devices can be created with similar methods like:
`Ds323x::new_ds3234(...)`.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate ds323x;
extern crate linux_embedded_hal as hal;
use ds323x::{Ds323x, NaiveDate, Rtcc};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3231(dev);
    let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    // do something else...
    let time = rtc.get_time().unwrap();
    println!("Time: {}", time);

    let _dev = rtc.destroy_ds3231();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/ds323x-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

