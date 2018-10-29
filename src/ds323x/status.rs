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
}
