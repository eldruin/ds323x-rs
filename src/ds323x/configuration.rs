//! Device configuration

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Enable the oscillator (set the clock running).
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::EOSC)
    }

    /// Disable the oscillator (stops the clock).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::EOSC)
    }

    /// Force a temperature conversion and time compensation with TXCO algorithm.
    ///
    /// The *busy* status should be checked before doing this. See [`is_busy()`](#method.is_busy)
    pub fn convert_temperature(&mut self) -> Result<(), Error<E>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        // do not overwrite if a conversion is in progress
        if (control & BitFlags::TEMP_CONV) == 0 {
            self.iface.write_register(Register::CONTROL, control | BitFlags::TEMP_CONV)?;
        }
        Ok(())
    }

    /// Enable the 32kHz output.
    ///
    /// (Does not alter the device register if already enabled).
    pub fn enable_32khz_output(&mut self) -> Result<(), Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        if (status & BitFlags::EN32KHZ) == 0 {
            self.iface.write_register(Register::STATUS, status | BitFlags::EN32KHZ)?;
        }
        Ok(())
    }

    /// Disable the 32kHz output.
    ///
    /// (Does not alter the device register if already disabled).
    pub fn disable_32khz_output(&mut self) -> Result<(), Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        if (status & BitFlags::EN32KHZ) != 0 {
            self.iface.write_register(Register::STATUS, status & !BitFlags::EN32KHZ)?;
        }
        Ok(())
    }

    /// Set the aging offset.
    pub fn set_aging_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.iface.write_register(Register::AGING_OFFSET, offset as u8)
    }

    fn write_control(&mut self, control: u8) -> Result<(), Error<E>> {
        self.iface.write_register(Register::CONTROL, control)?;
        self.control = control;
        Ok(())
    }
}
