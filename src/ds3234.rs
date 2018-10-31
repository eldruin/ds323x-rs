//! Functions exclusive of DS3234

extern crate embedded_hal as hal;
use hal::blocking;
use super::{ Ds323x, BitFlags, Error, ic };
use interface::SpiInterface;

impl<SPI, CS, E> Ds323x<SpiInterface<SPI, CS>, ic::DS3234>
where
    SPI: blocking::spi::Transfer<u8, Error = E> + blocking::spi::Write<u8, Error = E>,
    CS:  hal::digital::OutputPin
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
