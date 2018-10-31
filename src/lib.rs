//! This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
//! extremely accurate real-time clocks, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read and set date and time in 12-hour and 24-hour format. See: [`get_datetime`].
//! - Read and set date and time elements. For example, see: [`get_year`].
//! - Enable and disable the real-time clock. See: [`enable`].
//! - Read the temperature. See [`get_temperature`].
//! - Force a temperature conversion and time compensation. See [`convert_temperature`].
//! - Read the busy status. See [`is_busy`].
//! - Read whether the oscillator is or has been stopped. See [`has_been_stopped`].
//! - Clear the has-been-stopped flag. See [`clear_has_been_stopped_flag`].
//! - Enable/disable the 32kHz output. See [`enable_32khz_output`].
//! - Set the aging offset. See [`set_aging_offset`].
//!
//! [`get_datetime`]: struct.Ds323x.html#method.get_datetime
//! [`get_year`]: struct.Ds323x.html#method.get_year
//! [`enable`]: struct.Ds323x.html#method.enable
//! [`get_temperature`]: struct.Ds323x.html#method.get_temperature
//! [`convert_temperature`]: struct.Ds323x.html#method.convert_temperature
//! [`is_busy`]: struct.Ds323x.html#method.is_busy
//! [`has_been_stopped`]: struct.Ds323x.html#method.has_been_stopped
//! [`clear_has_been_stopped_flag`]: struct.Ds323x.html#method.clear_has_been_stopped_flag
//! [`enable_32khz_output`]: struct.Ds323x.html#method.enable_32khz_output
//! [`set_aging_offset`]: struct.Ds323x.html#method.set_aging_offset
//!
//! ## The devices
//!
//! This driver is compatible with the DS3231 and DS3232 I2C devices and the
//! DS3234 SPI device.
//!
//! ### DS3231
//! The DS3231 is a low-cost, extremely accurate I2C real-time clock (RTC) with
//! an integrated temperature-compensated crystal oscillator (TCXO) and crystal.
//!
//! The device incorporates a battery input, and maintains accurate timekeeping
//! when main power to the device is interrupted. The integration of the
//! crystal resonator enhances the long-term accuracy of the device as well as
//! reduces the piece-part count in a manufacturing line.
//! The DS3231 is available in commercial and industrial temperature ranges,
//! and is offered in a 16-pin, 300-mil SO package.
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
//! ### DS3232
//! The DS3232 is a low-cost temperature-compensated crystal oscillator (TCXO)
//! with a very accurate, temperature-compensated, integrated real-time clock
//! (RTC) and 236 bytes of battery-backed SRAM.
//!
//! Additionally, the DS3232 incorporates a battery input and maintains
//! accurate timekeeping when main power to the device is interrupted. The
//! integration of the crystal resonator enhances the long-term accuracy of the
//! device as well as reduces the piece-part count in a manufacturing line.
//! The DS3232 is available in commercial and industrial temperature ranges,
//! and is offered in an industry-standard 20-pin, 300-mil SO package.
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
//! necessary. Additionally, the RST pin is monitored as a pushbutton input for
//! generating a μP reset.
//!
//! ### DS3234
//! The DS3234 is a low-cost, extremely accurate SPI bus real-time clock (RTC)
//! with an integrated temperature-compensated crystal oscillator (TCXO) and
//! crystal.
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
//! ### Create a driver instance for the DS3231
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let rtc = Ds323x::new_ds3231(dev);
//! // do something...
//!
//! // get the I2C device back
//! let dev = rtc.destroy_ds3231();
//! # }
//! ```
//!
//! ### Create a driver instance for the DS3232
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let rtc = Ds323x::new_ds3232(dev);
//! // do something...
//!
//! // get the I2C device back
//! let dev = rtc.destroy_ds3232();
//! # }
//! ```
//!
//! ### Create a driver instance for the DS3234
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = hal::Pin::new(24);
//! let rtc = Ds323x::new_ds3234(dev, chip_select);
//! // do something...
//!
//! // get the SPI device and chip select pin back
//! let (dev, chip_select) = rtc.destroy_ds3234();
//! # }
//! ```
//!
//! ### Set the current date and time at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::{ Ds323x, DateTime, Hours };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let datetime = DateTime {
//!                           year: 2018,
//!                           month: 08,
//!                           day: 15,
//!                           weekday: 4,
//!                           hour: Hours::H24(19),
//!                           minute: 59,
//!                           second: 58
//!                };
//! rtc.set_datetime(&datetime).unwrap();
//! # }
//! ```
//!
//! ### Get the current date and time at once
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::{ Ds323x, Hours };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//!
//! let datetime = rtc.get_datetime().unwrap();
//!
//! // The hours depend on the RTC running mode
//! match datetime.hour {
//!     Hours::H24(h) => println!("{}-{}-{}, {} {}:{}:{}", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//!     Hours::AM(h) => println!("{}-{}-{}, {} {}:{}:{} AM", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//!     Hours::PM(h) => println!("{}-{}-{}, {} {}:{}:{} PM", datetime.year,
//!                               datetime.month, datetime.day, datetime.weekday,
//!                               h, datetime.minute, datetime.second),
//! }
//! // This will print something like: 2018-08-15, 4 19:59:58
//! # }
//! ```
//!
//! ### Get the year
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::{ Ds323x, Hours };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let year = rtc.get_year().unwrap();
//! println!("Year: {}", year);
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Set the year
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::{ Ds323x, Hours };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_year(2018).unwrap();
//! # }
//! ```
//! Similar methods exist for month, day, weekday, hours, minutes and seconds.
//!
//! ### Enable/disable the device
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.disable().unwrap(); // stops the clock
//! let is_running = rtc.is_running().unwrap();
//! println!("Is running: {}", is_running); // will print false
//! rtc.enable().unwrap(); // set clock to run
//! println!("Is running: {}", is_running); // will print true
//! # }
//! ```
//!
//! ### Read the temperature
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let temperature = rtc.get_temperature().unwrap();
//! # }
//! ```
//!
//! ### Read busy status
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! let is_busy = rtc.is_busy().unwrap();
//! # }
//! ```
//!
//! ### Enable the square-wave output with a frequency of 4.096Hz
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::{ Ds323x, SqWFreq };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_square_wave_frequency(SqWFreq::_4_096Hz).unwrap();
//! // The same output pin can be used for interrupts or as square-wave output
//! rtc.use_int_sqw_output_as_square_wave().unwrap();
//! rtc.enable_square_wave().unwrap();
//! # }
//! ```
//!
//! ### Set the aging offset
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ds323x;
//! use ds323x::Ds323x;
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let mut rtc = Ds323x::new_ds3231(dev);
//! rtc.set_aging_offset(-15).unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking;
use core::marker::PhantomData;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C/SPI bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData
}

/// Square-wave output frequency
#[derive(Debug, Clone, PartialEq)]
pub enum SqWFreq {
    /// 1 Hz
    _1Hz,
    /// 1.024 Hz
    _1_024Hz,
    /// 4.096 Hz
    _4_096Hz,
    /// 8.192 Hz
    _8_192Hz,
}

struct Register;

impl Register {
    const SECONDS      : u8 = 0x00;
    const MINUTES      : u8 = 0x01;
    const HOURS        : u8 = 0x02;
    const DOW          : u8 = 0x03;
    const DOM          : u8 = 0x04;
    const MONTH        : u8 = 0x05;
    const YEAR         : u8 = 0x06;
    const CONTROL      : u8 = 0x0E;
    const STATUS       : u8 = 0x0F;
    const AGING_OFFSET : u8 = 0x10;
    const TEMP_MSB     : u8 = 0x11;
}

struct BitFlags;

impl BitFlags {
    const H24_H12    : u8 = 0b0100_0000;
    const AM_PM      : u8 = 0b0010_0000;
    const CENTURY    : u8 = 0b1000_0000;
    const EOSC       : u8 = 0b1000_0000;
    const BBSQW      : u8 = 0b0100_0000;
    const TEMP_CONV  : u8 = 0b0010_0000;
    const RS2        : u8 = 0b0001_0000;
    const RS1        : u8 = 0b0000_1000;
    const INTCN      : u8 = 0b0000_0100;
    const BUSY       : u8 = 0b0000_0100;
    const EN32KHZ    : u8 = 0b0000_1000;
    const OSC_STOP   : u8 = 0b1000_0000;
}

const DEVICE_ADDRESS   : u8 = 0b110_1000;
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
pub mod interface;
use interface::{ I2cInterface, SpiInterface };

/// DS3231, DS3232 and DS3234 RTC driver
#[derive(Debug, Default)]
pub struct Ds323x<DI, IC> {
    iface: DI,
    control: u8,
    _ic: PhantomData<IC>
}

impl<I2C, E> Ds323x<I2cInterface<I2C>, ic::DS3231>
where
    I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E>
{
    /// Create a new instance of the DS3231 device.
    pub fn new_ds3231(i2c: I2C) -> Self {
        Ds323x {
            iface: I2cInterface {
                i2c,
            },
            control: CONTROL_POR_VALUE,
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_ds3231(self) -> I2C {
        self.iface.i2c
    }
}

impl<I2C, E> Ds323x<I2cInterface<I2C>, ic::DS3232>
where
    I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E>
{
    /// Create a new instance of the DS3232 device.
    pub fn new_ds3232(i2c: I2C) -> Self {
        Ds323x {
            iface: I2cInterface {
                i2c,
            },
            control: CONTROL_POR_VALUE,
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_ds3232(self) -> I2C {
        self.iface.i2c
    }
}

impl<SPI, CS, E> Ds323x<SpiInterface<SPI, CS>, ic::DS3234>
where
    SPI: blocking::spi::Transfer<u8, Error = E> + blocking::spi::Write<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    /// Create a new instance.
    pub fn new_ds3234(spi: SPI, chip_select: CS) -> Self {
        Ds323x {
            iface: SpiInterface {
                spi,
                cs: chip_select
            },
            control: CONTROL_POR_VALUE,
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_ds3234(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

mod ds323x;
pub use ds323x::{ Hours, DateTime };
