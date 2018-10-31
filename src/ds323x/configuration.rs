//! Device configuration

extern crate embedded_hal as hal;
use super::super::{ Ds323x, SqWFreq, Register, BitFlags, Error };
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
    pub fn enable_32khz_output(&mut self) -> Result<(), Error<E>> {
        // avoid clearing alarm flags
        let status = self.status | BitFlags::EN32KHZ | BitFlags::ALARM2F | BitFlags::ALARM1F;
        self.iface.write_register(Register::STATUS, status)?;
        self.status = status;
        Ok(())
    }

    /// Disable the 32kHz output.
    pub fn disable_32khz_output(&mut self) -> Result<(), Error<E>> {
        // avoid clearing alarm flags
        let status = self.status & !BitFlags::EN32KHZ | BitFlags::ALARM2F | BitFlags::ALARM1F;
        self.iface.write_register(Register::STATUS, status)?;
        self.status = status;
        Ok(())
    }

    /// Set the aging offset.
    pub fn set_aging_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.iface.write_register(Register::AGING_OFFSET, offset as u8)
    }

    /// Read the aging offset.
    pub fn get_aging_offset(&mut self) -> Result<i8, Error<E>> {
        let offset = self.iface.read_register(Register::AGING_OFFSET)?;
        Ok(offset as i8)
    }

    /// Set the interrupt/square-wave output to be used as interrupt output.
    pub fn use_int_sqw_output_as_interrupt(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::INTCN)
    }

    /// Set the interrupt/square-wave output to be used as square-wave output.
    pub fn use_int_sqw_output_as_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::INTCN)
    }

    /// Enable battery-backed square wave generation.
    pub fn enable_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::BBSQW)
    }

    /// Disable battery-backed square wave generation.
    pub fn disable_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::BBSQW)
    }

    /// Set the square-wave output frequency.
    pub fn set_square_wave_frequency(&mut self, freq: SqWFreq) -> Result<(), Error<E>> {
        let new_control;
        match freq {
            SqWFreq::_1Hz     => new_control = self.control & !BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_1_024Hz => new_control = self.control & !BitFlags::RS2 |  BitFlags::RS1,
            SqWFreq::_4_096Hz => new_control = self.control |  BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_8_192Hz => new_control = self.control |  BitFlags::RS2 |  BitFlags::RS1,
        }
        self.write_control(new_control)
    }

    fn write_control(&mut self, control: u8) -> Result<(), Error<E>> {
        self.iface.write_register(Register::CONTROL, control)?;
        self.control = control;
        Ok(())
    }
}
