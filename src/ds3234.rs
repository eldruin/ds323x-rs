//! Functions exclusive of DS3234

extern crate embedded_hal as hal;
use hal::blocking;
use core::marker::PhantomData;
use super::{ Ds323x, TempConvRate, Register, BitFlags, Error, ic, CONTROL_POR_VALUE };
use interface::{ SpiInterface, WriteData };

impl<SPI, CS, E> Ds323x<SpiInterface<SPI, CS>, ic::DS3234>
where
    SPI: blocking::spi::Transfer<u8, Error = E> + blocking::spi::Write<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    /// Create a new instance.
    pub fn new_ds3234(spi: SPI, chip_select: CS) -> Self {
        const STATUS_POR_VALUE : u8 = BitFlags::OSC_STOP | BitFlags::BB32KHZ | BitFlags::EN32KHZ;
        Ds323x {
            iface: SpiInterface {
                spi,
                cs: chip_select
            },
            control: CONTROL_POR_VALUE,
            status: STATUS_POR_VALUE,
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_ds3234(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }

    /// Enable the 32kHz output when battery-powered. (enabled per default)
    ///
    /// Additionally, the 32kHz output needs to be enabled. See
    /// [`enable_32khz_output()`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn enable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Disable the 32kHz output when battery-powered.
    ///
    /// The 32kHz output will still generate a wave when not battery-powered if
    /// it enabled. See [`enable_32khz_output()`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn disable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status & !BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Set the temperature conversion rate.
    ///
    /// Set how often the temperature is measured and applies compensation to
    /// the oscillator. This can be used to reduce power consumption but sudden
    /// temperature changes will not be compensated for.
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn set_temperature_conversion_rate(&mut self, rate: TempConvRate) -> Result<(), Error<E>> {
        let status;
        match rate {
            TempConvRate::_64s  => status = self.status & !BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_128s => status = self.status & !BitFlags::CRATE1 |  BitFlags::CRATE0,
            TempConvRate::_256s => status = self.status |  BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_512s => status = self.status |  BitFlags::CRATE1 |  BitFlags::CRATE0,
        }
        self.write_status_without_clearing_alarm(status)
    }

    /// Enable the temperature conversions when battery-powered. (enabled per default)
    ///
    /// Note: This is only available for DS3234 devices.
    pub fn enable_temperature_conversions_on_battery(&mut self) -> Result<(), Error<E>> {
        self.iface.write_register(Register::TEMP_CONV, 0)
    }

    /// Disable the temperature conversions when battery-powered.
    ///
    /// Note: This is only available for DS3234 devices.
    pub fn disable_temperature_conversions_on_battery(&mut self) -> Result<(), Error<E>> {
        self.iface.write_register(Register::TEMP_CONV, BitFlags::TEMP_CONV_BAT)
    }
}
