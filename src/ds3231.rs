//! Functions exclusive of DS3231

use crate::{ic, interface::I2cInterface, BitFlags, Ds323x, CONTROL_POR_VALUE};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C, E> Ds323x<I2cInterface<I2C>, ic::DS3231>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Create a new instance of the DS3231 device.
    pub fn new_ds3231(i2c: I2C) -> Self {
        const STATUS_POR_VALUE: u8 = BitFlags::OSC_STOP | BitFlags::EN32KHZ;
        Ds323x {
            iface: I2cInterface { i2c },
            control: CONTROL_POR_VALUE,
            status: STATUS_POR_VALUE,
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_ds3231(self) -> I2C {
        self.iface.i2c
    }
}
