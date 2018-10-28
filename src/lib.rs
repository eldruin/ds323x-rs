//! This is a platform agnostic Rust driver for the DS3231, DS3232 and DS3234
//! extremely accurate real-time clocks, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read/write the seconds.
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

#![deny(unsafe_code)]
#![deny(missing_docs)]
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

struct Register;

impl Register {
    const SECONDS   : u8 = 0x00;
 }

const DEVICE_ADDRESS: u8 = 0b110_1000;

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
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_ds3234(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

mod ds323x;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate embedded_hal_mock as hal;
    extern crate std;
    use self::std::vec;

    struct DummyOutputPin;
    impl embedded_hal::digital::OutputPin for DummyOutputPin {
        fn set_low(&mut self) {}
        fn set_high(&mut self) {}
    }

    mod ds3231 {
        use super::*;

        #[test]
        fn can_create() {
            Ds323x::new_ds3231(hal::i2c::Mock::new(&[]));
        }

        #[test]
        fn can_get_seconds() {
            let transactions = [
                hal::i2c::Transaction::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1])
            ];
            let mut dev = Ds323x::new_ds3231(hal::i2c::Mock::new(&transactions));
            assert_eq!(1, dev.get_seconds().unwrap());
        }

        #[test]
        fn can_set_seconds() {
            let transactions = [
                hal::i2c::Transaction::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1])
            ];
            let mut dev = Ds323x::new_ds3231(hal::i2c::Mock::new(&transactions));
            dev.set_seconds(1).unwrap();
        }
    }
    mod ds3232 {
        use super::*;

        #[test]
        fn can_create() {
            Ds323x::new_ds3232(hal::i2c::Mock::new(&[]));
        }

        #[test]
        fn can_get_seconds() {
            let transactions = [
                hal::i2c::Transaction::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1])
            ];
            let mut dev = Ds323x::new_ds3232(hal::i2c::Mock::new(&transactions));
            assert_eq!(1, dev.get_seconds().unwrap());
        }

        #[test]
        fn can_set_seconds() {
            let transactions = [
                hal::i2c::Transaction::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1])
            ];
            let mut dev = Ds323x::new_ds3232(hal::i2c::Mock::new(&transactions));
            dev.set_seconds(1).unwrap();
        }
    }

    mod ds3234 {
        use super::*;

        #[test]
        fn can_create() {
            Ds323x::new_ds3234(hal::spi::Mock::new(&[]), DummyOutputPin);
        }

        #[test]
        fn can_get_seconds() {
            let transactions = [
                hal::spi::Transaction::transfer(vec![Register::SECONDS, 0], vec![Register::SECONDS, 1])
            ];
            let mut dev = Ds323x::new_ds3234(hal::spi::Mock::new(&transactions), DummyOutputPin);
            assert_eq!(1, dev.get_seconds().unwrap());
        }

        #[test]
        fn can_set_seconds() {
            let transactions = [
                hal::spi::Transaction::write(vec![Register::SECONDS + 0x80, 1])
            ];
            let mut dev = Ds323x::new_ds3234(hal::spi::Mock::new(&transactions), DummyOutputPin);
            dev.set_seconds(1).unwrap();
        }
    }
}