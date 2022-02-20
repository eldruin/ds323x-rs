//! Device configuration

use crate::{
    interface::{ReadData, WriteData},
    BitFlags, Ds323x, Error, Register, SqWFreq,
};

impl<DI, IC, CommE, PinE> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Enable the oscillator (set the clock running) (default).
    pub fn enable(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control & !BitFlags::EOSC)
    }

    /// Disable the oscillator (stops the clock).
    pub fn disable(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control | BitFlags::EOSC)
    }

    /// Force a temperature conversion and time compensation with TXCO algorithm.
    ///
    /// The *busy* status should be checked before doing this. See [`busy()`](#method.busy)
    pub fn convert_temperature(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.iface.read_register(Register::CONTROL)?;
        // do not overwrite if a conversion is in progress
        if (control & BitFlags::TEMP_CONV) == 0 {
            self.iface
                .write_register(Register::CONTROL, control | BitFlags::TEMP_CONV)?;
        }
        Ok(())
    }

    /// Enable the 32kHz output. (enabled per default)
    pub fn enable_32khz_output(&mut self) -> Result<(), Error<CommE, PinE>> {
        let status = self.status | BitFlags::EN32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Disable the 32kHz output.
    pub fn disable_32khz_output(&mut self) -> Result<(), Error<CommE, PinE>> {
        let status = self.status & !BitFlags::EN32KHZ;
        self.write_status_without_clearing_alarm(status)
    }

    /// Set the aging offset.
    pub fn set_aging_offset(&mut self, offset: i8) -> Result<(), Error<CommE, PinE>> {
        self.iface
            .write_register(Register::AGING_OFFSET, offset as u8)
    }

    /// Read the aging offset.
    pub fn aging_offset(&mut self) -> Result<i8, Error<CommE, PinE>> {
        let offset = self.iface.read_register(Register::AGING_OFFSET)?;
        Ok(offset as i8)
    }

    /// Set the interrupt/square-wave output to be used as interrupt output.
    pub fn use_int_sqw_output_as_interrupt(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control | BitFlags::INTCN)
    }

    /// Set the interrupt/square-wave output to be used as square-wave output. (default)
    pub fn use_int_sqw_output_as_square_wave(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control & !BitFlags::INTCN)
    }

    /// Enable battery-backed square wave generation.
    pub fn enable_square_wave(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control | BitFlags::BBSQW)
    }

    /// Disable battery-backed square wave generation.
    pub fn disable_square_wave(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control & !BitFlags::BBSQW)
    }

    /// Set the square-wave output frequency.
    pub fn set_square_wave_frequency(&mut self, freq: SqWFreq) -> Result<(), Error<CommE, PinE>> {
        let new_control;
        match freq {
            SqWFreq::_1Hz => new_control = self.control & !BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_1_024Hz => new_control = self.control & !BitFlags::RS2 | BitFlags::RS1,
            SqWFreq::_4_096Hz => new_control = self.control | BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_8_192Hz => new_control = self.control | BitFlags::RS2 | BitFlags::RS1,
        }
        self.write_control(new_control)
    }

    /// Enable Alarm1 interrupts.
    pub fn enable_alarm1_interrupts(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control | BitFlags::ALARM1_INT_EN)
    }

    /// Disable Alarm1 interrupts.
    pub fn disable_alarm1_interrupts(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control & !BitFlags::ALARM1_INT_EN)
    }

    /// Enable Alarm2 interrupts.
    pub fn enable_alarm2_interrupts(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control | BitFlags::ALARM2_INT_EN)
    }

    /// Disable Alarm2 interrupts.
    pub fn disable_alarm2_interrupts(&mut self) -> Result<(), Error<CommE, PinE>> {
        let control = self.control;
        self.write_control(control & !BitFlags::ALARM2_INT_EN)
    }

    fn write_control(&mut self, control: u8) -> Result<(), Error<CommE, PinE>> {
        self.iface.write_register(Register::CONTROL, control)?;
        self.control = control;
        Ok(())
    }

    pub(crate) fn write_status_without_clearing_alarm(
        &mut self,
        status: u8,
    ) -> Result<(), Error<CommE, PinE>> {
        // avoid clearing alarm flags
        let new_status = status | BitFlags::ALARM2F | BitFlags::ALARM1F;
        self.iface.write_register(Register::STATUS, new_status)?;
        self.status = status;
        Ok(())
    }
}
