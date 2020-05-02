//! Functions exclusive of DS3234
use super::{ic, BitFlags, Ds323x, Error, Register, TempConvRate, CONTROL_POR_VALUE};
use core::marker::PhantomData;
use hal::blocking;
use interface::{SpiInterface, WriteData};

impl<SPI, CS, CommE, PinE> Ds323x<SpiInterface<SPI, CS>, ic::DS3234>
where
    SPI: blocking::spi::Transfer<u8, Error = CommE> + blocking::spi::Write<u8, Error = CommE>,
    CS: hal::digital::v2::OutputPin<Error = PinE>,
{
    /// Create a new instance.
    pub fn new_ds3234(spi: SPI, chip_select: CS) -> Self {
        const STATUS_POR_VALUE: u8 = BitFlags::OSC_STOP | BitFlags::BB32KHZ | BitFlags::EN32KHZ;
        Ds323x {
            iface: SpiInterface {
                spi,
                cs: chip_select,
            },
            control: CONTROL_POR_VALUE,
            status: STATUS_POR_VALUE,
            _ic: PhantomData,
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
    pub fn enable_32khz_output_on_battery(&mut self) -> Result<(), Error<CommE, PinE>> {
        let status = self.status | BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Disable the 32kHz output when battery-powered.
    ///
    /// The 32kHz output will still generate a wave when not battery-powered if
    /// it enabled. See [`enable_32khz_output()`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn disable_32khz_output_on_battery(&mut self) -> Result<(), Error<CommE, PinE>> {
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
    pub fn set_temperature_conversion_rate(
        &mut self,
        rate: TempConvRate,
    ) -> Result<(), Error<CommE, PinE>> {
        let status = match rate {
            TempConvRate::_64s => self.status & !BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_128s => self.status & !BitFlags::CRATE1 | BitFlags::CRATE0,
            TempConvRate::_256s => self.status | BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_512s => self.status | BitFlags::CRATE1 | BitFlags::CRATE0,
        };
        self.write_status_without_clearing_alarm(status)
    }

    /// Enable the temperature conversions when battery-powered. (enabled per default)
    ///
    /// Note: This is only available for DS3234 devices.
    pub fn enable_temperature_conversions_on_battery(&mut self) -> Result<(), Error<CommE, PinE>> {
        self.iface.write_register(Register::TEMP_CONV, 0)
    }

    /// Disable the temperature conversions when battery-powered.
    ///
    /// Note: This is only available for DS3234 devices.
    pub fn disable_temperature_conversions_on_battery(&mut self) -> Result<(), Error<CommE, PinE>> {
        self.iface
            .write_register(Register::TEMP_CONV, BitFlags::TEMP_CONV_BAT)
    }
}
