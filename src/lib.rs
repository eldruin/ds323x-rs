//! This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
//! extremely accurate real-time clocks, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read and set date and time in 12-hour and 24-hour format. See: [`datetime`].
//! - Read and set date and time individual elements. For example, see: [`year`].
//! - Enable and disable the real-time clock. See: [`enable`].
//! - Read the busy status. See [`busy`].
//! - Read whether the oscillator is or has been stopped. See [`has_been_stopped`].
//! - Clear the has-been-stopped flag. See [`clear_has_been_stopped_flag`].
//! - Set and read the aging offset. See [`set_aging_offset`].
//! - Select the function of the INT/SQW output pin. See [`use_int_sqw_output_as_interrupt`].
//! - Alarms:
//!     - Set alarms 1 and 2 with several matching policies. See [`set_alarm1_day`].
//!     - Set alarms 1 and 2 for a time. See [`set_alarm1_hms`].
//!     - Read whether alarms 1 or 2 have matched. See [`has_alarm1_matched`].
//!     - Clear flag indicating that alarms 1 or 2 have matched. See [`clear_alarm1_matched_flag`].
//!     - Enable and disable alarms 1 and 2 interrupt generation. See [`enable_alarm1_interrupts`].
//! - Wave generation:
//!     - Enable and disable the square-wave generation. See [`enable_square_wave`].
//!     - Select the square-wave frequency. See [`set_square_wave_frequency`].
//!     - Enable and disable the 32kHz output. See [`enable_32khz_output`].
//!     - Enable and disable the 32kHz output when battery powered. See [`enable_32khz_output_on_battery`].
//! - Temperature conversion:
//!     - Read the temperature. See [`temperature`].
//!     - Force a temperature conversion and time compensation. See [`convert_temperature`].
//!     - Set the temperature conversion rate. See [`set_temperature_conversion_rate`].
//!     - Enable and disable the temperature conversions when battery-powered. See [`enable_temperature_conversions_on_battery`].
//!
//! [`datetime`]: Ds323x::datetime
//! [`year`]: Ds323x::year
//! [`enable`]: Ds323x::enable
//! [`temperature`]: Ds323x::temperature
//! [`convert_temperature`]: Ds323x::convert_temperature
//! [`busy`]: Ds323x::busy
//! [`has_been_stopped`]: Ds323x::has_been_stopped
//! [`clear_has_been_stopped_flag`]: Ds323x::clear_has_been_stopped_flag
//! [`set_aging_offset`]: Ds323x::set_aging_offset
//! [`enable_32khz_output`]: Ds323x::enable_32khz_output
//! [`use_int_sqw_output_as_interrupt`]: Ds323x::use_int_sqw_output_as_interrupt
//! [`enable_square_wave`]: Ds323x::enable_square_wave
//! [`set_square_wave_frequency`]: Ds323x::set_square_wave_frequency
//! [`set_alarm1_day`]: Ds323x::set_alarm1_day
//! [`set_alarm1_hms`]: Ds323x::set_alarm1_hms
//! [`has_alarm1_matched`]: Ds323x::has_alarm1_matched
//! [`clear_alarm1_matched_flag`]: Ds323x::clear_alarm1_matched_flag
//! [`enable_alarm1_interrupts`]: Ds323x::enable_alarm1_interrupts
//! [`enable_32khz_output_on_battery`]: Ds323x::enable_32khz_output_on_battery
//! [`set_temperature_conversion_rate`]: Ds323x::set_temperature_conversion_rate
//! [`enable_temperature_conversions_on_battery`]: Ds323x::enable_temperature_conversions_on_battery
//!
//! ## The devices
//!
//! This driver is compatible with the DS3231 and DS3232 I2C devices and the
//! DS3234 SPI device.
//!
//! These devices are low-cost temperature-compensated crystal oscillator (TCXO)
//! with a very accurate, temperature-compensated, integrated real-time clock
//! (RTC) including 236/256 bytes of battery-backed SRAM, depending on the model.
//!
//! ### DS3231 and DS3232 details
//!
//! The devices incorporate a battery input, and maintain accurate timekeeping
//! when main power to the devices is interrupted. The integration of the
//! crystal resonator enhances the long-term accuracy of the devices as well as
//! reduces the piece-part count in a manufacturing line.
//! The devices are available in commercial and industrial temperature ranges,
//! and are offered in a 16-pin, 300-mil SO package.
//!
//! The RTC maintains seconds, minutes, hours, day, date, month, and year
//! information. The date at the end of the month is automatically adjusted for
//! months with fewer than 31 days, including corrections for leap year. The
//! clock operates in either the 24-hour or 12-hour format with an AM/PM
//! indicator. Two programmable time-of-day alarms and a programmable
//! square-wave output are provided. Address and data are transferred serially
//! through an I2C bidirectional bus.
//!
//! A precision temperature-compensated voltage reference and comparator
//! circuit monitors the status of VCC to detect power failures, to provide a
//! reset output, and to automatically switch to the backup supply when
//! necessary. Additionally, the RST pin is monitored as a pushbutton
//! input for generating a μP reset.
//!
//! ### DS3234 details
//!
//! The DS3234 incorporates a precision, temperature-compensated voltage
//! reference and comparator circuit to monitor VCC. When VCC drops below the
//! power-fail voltage (VPF), the device asserts the RST output and also
//! disables read and write access to the part when VCC drops below both VPF
//! and VBAT. The RST pin is monitored as a pushbutton input for generating a
//! μP reset. The device switches to the backup supply input and maintains
//! accurate timekeeping when main power to the device is interrupted.
//! The integration of the crystal resonator enhances the long-term accuracy of
//! the device as well as reduces the piece-part count in a manufacturing line.
//! The DS3234 is available in commercial and industrial temperature ranges,
//! and is offered in an industry-standard 300-mil, 20-pin SO package.
//!
//! The DS3234 also integrates 256 bytes of battery-backed SRAM. In the event
//! of main power loss, the contents of the memory are maintained by the power
//! source connected to the V BAT pin. The RTC maintains seconds, minutes,
//! hours, day, date, month, and year information. The date at the end of the
//! month is automatically adjusted for months with fewer than 31 days,
//! including corrections for leap year. The clock operates in either the
//! 24-hour or 12-hour format with AM/PM indicator. Two programmable
//! time-of-day alarms and a programmable square-wave output are provided.
//! Address and data are transferred serially by an SPI bidirectional bus.
//!
//! Datasheets:
//! - [DS3231](https://datasheets.maximintegrated.com/en/ds/DS3231.pdf)
//! - [DS3232](https://datasheets.maximintegrated.com/en/ds/DS3232.pdf)
//! - [DS3234](https://datasheets.maximintegrated.com/en/ds/DS3234.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! In the following 3 examples an instance of the devices DS3231, DS3232 and
//! DS3234 will be created as an example. The rest of examples will use the
//! DS3231 as an example, except when using features specific to another IC,
//! for example, RAM access which is not available in the DS3231 device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create a driver instance for the DS3231
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let rtc = Ds323x::new_ds3231(dev);
//! // do something...
//!
//! // get the I2C device back
//! let dev = rtc.destroy_ds3231();
//! ```
//!
//! ### Create a driver instance for the DS3232
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let rtc = Ds323x::new_ds3232(dev);
//! // do something...
//!
//! // get the I2C device back
//! let dev = rtc.destroy_ds3232();
//! ```
//!
//! ### Create a driver instance for the DS3234
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use embedded_hal_bus::spi::ExclusiveDevice;
//! use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
//!
//! let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
//! let chip_select = SysfsPin::new(25);
//! let dev = ExclusiveDevice::new(spi, chip_select, Delay).unwrap();
//! let rtc = Ds323x::new_ds3234(dev);
//! // do something...
//!
//! // get the SPI device back
//! let dev = rtc.destroy_ds3234();
//! ```
//!
//! ### Set the current date and time at once
//!
//! ```no_run
//! use ds323x::{Ds323x, NaiveDate, DateTimeAccess};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
//! rtc.set_datetime(&datetime).unwrap();
//! ```
//!
//! ### Get the current date and time at once
//!
//! ```no_run
//! use ds323x::{Ds323x, DateTimeAccess};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let dt = rtc.datetime().unwrap();
//! println!("{}", dt);
//! // This will print something like: 2020-05-01 19:59:58
//! ```
//!
//! ### Get the year
//!
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ```no_run
//! use ds323x::{Ds323x, Rtcc};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let year = rtc.year().unwrap();
//! println!("Year: {}", year);
//! ```
//!
//! ### Set the year
//!
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ```no_run
//! use ds323x::{Ds323x, Rtcc};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_year(2018).unwrap();
//! ```
//!
//! ### Enable/disable the device
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.disable().unwrap(); // stops the clock
//! let running = rtc.running().unwrap();
//! println!("Is running: {}", running); // will print false
//! rtc.enable().unwrap(); // set clock to run
//! println!("Is running: {}", running); // will print true
//! ```
//!
//! ### Read the temperature
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let temperature = rtc.temperature().unwrap();
//! ```
//!
//! ### Read busy status
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let busy = rtc.busy().unwrap();
//! ```
//!
//! ### Enable the square-wave output with a frequency of 4.096Hz
//!
//! ```no_run
//! use ds323x::{Ds323x, SqWFreq};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_square_wave_frequency(SqWFreq::_4_096Hz).unwrap();
//! // The same output pin can be used for interrupts or as square-wave output
//! rtc.use_int_sqw_output_as_square_wave().unwrap();
//! rtc.enable_square_wave().unwrap();
//! ```
//!
//! ### Enable the 32kHz output except when on battery power
//!
//! Additionally enabling the output depending on the power source is only
//! available for the devices DS3232 and DS3234.
//!
//! ```no_run
//! use ds323x::{Ds323x, SqWFreq};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3232(dev);
//! rtc.disable_32khz_output_on_battery().unwrap(); // only available for DS3232 and DS3234
//! rtc.enable_32khz_output().unwrap();
//! ```
//!
//! ### Set the aging offset
//!
//! ```no_run
//! use ds323x::Ds323x;
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_aging_offset(-15).unwrap();
//! ```
//!
//! ### Set the temperature conversion rate to once every 128 seconds
//!
//! This is only available for the devices DS3232 and DS3234.
//!
//! ```no_run
//! use ds323x::{Ds323x, TempConvRate};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3232(dev);
//! rtc.set_temperature_conversion_rate(TempConvRate::_128s).unwrap();
//! ```
//!
//! ### Set the Alarm1 to each week on a week day at a specific time
//!
//! ```no_run
//! use ds323x::{Ds323x, Hours, WeekdayAlarm1, Alarm1Matching};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let alarm1 = WeekdayAlarm1 {
//!     weekday: 1,
//!     hour: Hours::H24(7),
//!     minute: 2,
//!     second: 15
//! };
//! rtc.set_alarm1_weekday(alarm1, Alarm1Matching::AllMatch).unwrap();
//! ```
//!
//! ### Set the Alarm2 to each day at the same time and enable interrupts on output
//!
//! The INT/SQW output pin will be set to 1 when it the alarm matches.
//!
//! ```no_run
//! use ds323x::{Ds323x, Hours, DayAlarm2, Alarm2Matching};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let alarm2 = DayAlarm2 {
//!     day: 1, // does not matter given the chosen matching
//!     hour: Hours::AM(11),
//!     minute: 2
//! };
//! rtc.set_alarm2_day(alarm2, Alarm2Matching::HoursAndMinutesMatch).unwrap();
//! rtc.use_int_sqw_output_as_interrupt().unwrap();
//! rtc.enable_alarm2_interrupts().unwrap();
//! ```
//!
//! ### Set the Alarm1 to a specific time
//!
//! ```no_run
//! use ds323x::{Ds323x, Hours, NaiveTime};
//! use linux_embedded_hal::I2cdev;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let time = NaiveTime::from_hms(19, 59, 58);
//! rtc.set_alarm1_hms(time).unwrap();
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
use embedded_hal::spi::{Mode, MODE_1, MODE_3};
pub use rtcc::{
    DateTimeAccess, Datelike, Hours, NaiveDate, NaiveDateTime, NaiveTime, Rtcc, Timelike,
};

/// SPI mode 1 (CPOL = 0, CPHA = 1)
pub const SPI_MODE_1: Mode = MODE_1;
/// SPI mode 3 (CPOL = 1, CPHA = 1)
pub const SPI_MODE_3: Mode = MODE_3;

/// All possible errors in this crate
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// I²C/SPI bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData,
    /// Internal device state is invalid.
    ///
    /// It was not possible to read a valid date and/or time.
    /// The device is probably missing initialization.
    InvalidDeviceState,
}

/// Square-wave output frequency
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SqWFreq {
    /// 1 Hz (default)
    _1Hz,
    /// 1.024 Hz
    _1_024Hz,
    /// 4.096 Hz
    _4_096Hz,
    /// 8.192 Hz
    _8_192Hz,
}

/// Temperature conversion rate
///
/// This is only available on the DS3232 and DS3234 devices.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TempConvRate {
    /// Once every 64 seconds (default)
    _64s,
    /// Once every 128 seconds
    _128s,
    /// Once every 256 seconds
    _256s,
    /// Once every 512 seconds
    _512s,
}

struct Register;

impl Register {
    const SECONDS: u8 = 0x00;
    const MINUTES: u8 = 0x01;
    const HOURS: u8 = 0x02;
    const DOW: u8 = 0x03;
    const DOM: u8 = 0x04;
    const MONTH: u8 = 0x05;
    const YEAR: u8 = 0x06;
    const ALARM1_SECONDS: u8 = 0x07;
    const ALARM2_MINUTES: u8 = 0x0B;
    const CONTROL: u8 = 0x0E;
    const STATUS: u8 = 0x0F;
    const AGING_OFFSET: u8 = 0x10;
    const TEMP_MSB: u8 = 0x11;
    const TEMP_CONV: u8 = 0x13;
}

struct BitFlags;

impl BitFlags {
    const H24_H12: u8 = 0b0100_0000;
    const AM_PM: u8 = 0b0010_0000;
    const CENTURY: u8 = 0b1000_0000;
    const EOSC: u8 = 0b1000_0000;
    const BBSQW: u8 = 0b0100_0000;
    const TEMP_CONV: u8 = 0b0010_0000;
    const RS2: u8 = 0b0001_0000;
    const RS1: u8 = 0b0000_1000;
    const INTCN: u8 = 0b0000_0100;
    const ALARM2_INT_EN: u8 = 0b0000_0010;
    const ALARM1_INT_EN: u8 = 0b0000_0001;
    const OSC_STOP: u8 = 0b1000_0000;
    const BB32KHZ: u8 = 0b0100_0000;
    const CRATE1: u8 = 0b0010_0000;
    const CRATE0: u8 = 0b0001_0000;
    const EN32KHZ: u8 = 0b0000_1000;
    const BUSY: u8 = 0b0000_0100;
    const ALARM2F: u8 = 0b0000_0010;
    const ALARM1F: u8 = 0b0000_0001;
    const TEMP_CONV_BAT: u8 = 0b0000_0001;
    const ALARM_MATCH: u8 = 0b1000_0000;
    const WEEKDAY: u8 = 0b0100_0000;
}

const DEVICE_ADDRESS: u8 = 0b110_1000;
const CONTROL_POR_VALUE: u8 = 0b0001_1100;

/// IC markers
pub mod ic {
    /// DS3231 IC marker
    pub struct DS3231;
    /// DS3232 IC marker
    pub struct DS3232;
    /// DS3234 IC marker
    pub struct DS3234;
}

/// DS3231, DS3232 and DS3234 RTC driver
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ds323x<DI, IC> {
    iface: DI,
    control: u8,
    status: u8,
    _ic: PhantomData<IC>,
}

mod ds323x;
pub mod interface;
pub use crate::ds323x::{
    Alarm1Matching, Alarm2Matching, DayAlarm1, DayAlarm2, WeekdayAlarm1, WeekdayAlarm2,
};
mod ds3231;
mod ds3232;
mod ds3234;

mod private {
    use super::{ic, interface};
    pub trait Sealed {}

    impl<SPI> Sealed for interface::SpiInterface<SPI> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}

    impl Sealed for ic::DS3231 {}
    impl Sealed for ic::DS3232 {}
    impl Sealed for ic::DS3234 {}
}
