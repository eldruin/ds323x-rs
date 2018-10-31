//! Functions exclusive of DS3232

extern crate embedded_hal as hal;
use hal::blocking;
use super::{ Ds323x, BitFlags, Error, ic };
use interface::I2cInterface;

impl<I2C, E> Ds323x<I2cInterface<I2C>, ic::DS3232>
where
    I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E>
{
    /// Enable the 32kHz output when battery-powered.
    ///
    /// Additionally, the 32kHz output needs to be enabled. See
    /// [`enable_32khz_output`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn enable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Disable the 32kHz output when battery-powered.
    ///
    /// The 32kHz output will still generate a wave when not battery-powered if
    /// it enabled. See [`enable_32khz_output`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub fn disable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status & !BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status)
    }
}
