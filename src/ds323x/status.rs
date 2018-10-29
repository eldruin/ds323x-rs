//! Device configuration

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Read busy status.
    pub fn is_busy(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::BUSY) != 0)
    }

    /// Read whether the oscillator is stopped or has been stopped at
    /// some point.
    pub fn has_been_stopped(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::OSC_STOP) != 0)
    }

    /// Clear the has been stopped flag.
    ///
    /// (Does not alter the device register if already cleared).
    pub fn clear_has_been_stopped_flag(&mut self) -> Result<(), Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        if (status & BitFlags::OSC_STOP) != 0 {
            self.iface.write_register(Register::STATUS, status & !BitFlags::OSC_STOP)?;
        }
        Ok(())
    }
}
