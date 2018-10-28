//! Device configuration

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Enable the oscillator (set the clock running).
    ///
    /// (Does not alter the device register if already running).
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        if (control & BitFlags::EOSC) != 0 {
            self.iface.write_register(Register::CONTROL, control & !BitFlags::EOSC)?;
        }
        Ok(())
    }

    /// Disable the oscillator (stops the clock).
    ///
    /// (Does not alter the device register if already stopped).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        if (control & BitFlags::EOSC) == 0 {
            self.iface.write_register(Register::CONTROL, control | BitFlags::EOSC)?;
        }
        Ok(())
    }
}
