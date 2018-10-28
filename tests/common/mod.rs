extern crate embedded_hal;

pub const DEVICE_ADDRESS: u8 = 0b110_1000;

pub struct Register;

impl Register {
    pub const SECONDS   : u8 = 0x00;
 }

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}
