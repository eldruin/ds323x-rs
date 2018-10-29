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

    /// Read the temperature.
    pub fn get_temperature(&mut self) -> Result<f32, Error<E>> {
        let mut data = [Register::TEMP_MSB, 0, 0];
        self.iface.read_data(&mut data)?;
        let is_negative = (data[1] & 0b1000_0000) != 0;
        let temp = ((data[1] as u16) << 2) | (data[2] >> 6) as u16;
        if is_negative {
            let temp_sign_extended = temp | 0b1111_1100_0000_0000;
            Ok(temp_sign_extended as i16 as f32 * 0.25)
        }
        else {
            let temp = ((data[1] as u16) << 2) | (data[2] >> 6) as u16;
            Ok(temp as f32 * 0.25)
        }
    }
}
