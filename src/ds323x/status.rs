//! Device status

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Read whether the oscillator is running
    pub fn is_running(&mut self) -> Result<bool, Error<E>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        Ok((control & BitFlags::EOSC) == 0)
    }

    /// Read the busy status
    pub fn is_busy(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::BUSY) != 0)
    }

    /// Read whether the oscillator is stopped or has been stopped at
    /// some point.
    ///
    /// This allows a better assessment of the validity of the timekeeping data.
    ///
    /// Once this is true, it will stay as such until cleared with
    /// [`clear_has_been_stopped_flag()`](#method.clear_has_been_stopped_flag)
    pub fn has_been_stopped(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::OSC_STOP) != 0)
    }

    /// Clear flag signalling whether the oscillator is stopped or has been
    /// stopped at some point.
    ///
    /// (Does not alter the device register if already cleared).
    /// See also: [`has_been_stopped()`](#method.has_been_stopped)
    pub fn clear_has_been_stopped_flag(&mut self) -> Result<(), Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        if (status & BitFlags::OSC_STOP) != 0 {
            self.iface.write_register(Register::STATUS, status & !BitFlags::OSC_STOP)?;
        }
        Ok(())
    }

    /// Read the temperature.
    ///
    /// Note: It is possible to manually force a temperature conversion with
    /// [`convert_temperature()`](#method.convert_temperature)
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
            Ok(temp as f32 * 0.25)
        }
    }
}
