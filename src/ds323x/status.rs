//! Device status

use crate::{
    interface::{ReadData, WriteData},
    BitFlags, Ds323x, Error, Register,
};

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    /// Read whether the oscillator is running
    pub fn running(&mut self) -> Result<bool, Error<E>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        Ok((control & BitFlags::EOSC) == 0)
    }

    /// Read the busy status
    pub fn busy(&mut self) -> Result<bool, Error<E>> {
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
    /// See also: [`has_been_stopped()`](#method.has_been_stopped)
    pub fn clear_has_been_stopped_flag(&mut self) -> Result<(), Error<E>> {
        let status = self.status & !BitFlags::OSC_STOP;
        self.write_status_without_clearing_alarm(status)
    }

    /// Read whether the Alarm1 has matched at some point.
    ///
    /// Once this is true, it will stay as such until cleared with
    /// [`clear_alarm1_matched_flag()`](#method.clear_alarm1_matched_flag)
    pub fn has_alarm1_matched(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::ALARM1F) != 0)
    }

    /// Clear flag signalling whether the Alarm1 has matched at some point.
    ///
    /// See also: [`has_alarm1_matched()`](#method.has_alarm1_matched)
    pub fn clear_alarm1_matched_flag(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::ALARM2F;
        self.iface.write_register(Register::STATUS, status)
    }

    /// Read whether the Alarm2 has matched at some point.
    ///
    /// Once this is true, it will stay as such until cleared with
    /// [`clear_alarm2_matched_flag()`](#method.clear_alarm2_matched_flag)
    pub fn has_alarm2_matched(&mut self) -> Result<bool, Error<E>> {
        let status = self.iface.read_register(Register::STATUS)?;
        Ok((status & BitFlags::ALARM2F) != 0)
    }

    /// Clear flag signalling whether the Alarm2 has matched at some point.
    ///
    /// See also: [`has_alarm2_matched()`](#method.has_alarm2_matched)
    pub fn clear_alarm2_matched_flag(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::ALARM1F;
        self.iface.write_register(Register::STATUS, status)
    }

    /// Read the temperature.
    ///
    /// Note: It is possible to manually force a temperature conversion with
    /// [`convert_temperature()`](#method.convert_temperature)
    pub fn temperature(&mut self) -> Result<f32, Error<E>> {
        let mut data = [Register::TEMP_MSB, 0, 0];
        self.iface.read_data(&mut data)?;
        let is_negative = (data[1] & 0b1000_0000) != 0;
        let temp = (u16::from(data[1]) << 2) | u16::from(data[2] >> 6);
        if is_negative {
            let temp_sign_extended = temp | 0b1111_1100_0000_0000;
            Ok(f32::from(temp_sign_extended as i16) * 0.25)
        } else {
            Ok(f32::from(temp) * 0.25)
        }
    }
}
