extern crate embedded_hal;
extern crate ds323x;
use self::ds323x::{ Ds323x, interface, ic };
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTrans };
use hal::spi::{ Mock as SpiMock, Transaction as SpiTrans };

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


pub fn new_ds3231(transactions: &[I2cTrans]) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3231> {
    Ds323x::new_ds3231(I2cMock::new(&transactions))
}

pub fn new_ds3232(transactions: &[I2cTrans]) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3232> {
    Ds323x::new_ds3232(I2cMock::new(&transactions))
}

pub fn new_ds3234(transactions: &[SpiTrans])
    -> Ds323x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::DS3234> {
    Ds323x::new_ds3234(SpiMock::new(&transactions), DummyOutputPin)
}

#[macro_export]
macro_rules! get_test {
    ($name:ident, $method:ident, $create_method:ident, $expected:expr, $transaction:expr) => {
        #[test]
        fn $name() {
            let transactions = [ $transaction ];
            let mut dev = $create_method(&transactions);
            assert_eq!($expected, dev.$method().unwrap());
        }
    };
}

#[macro_export]
macro_rules! set_test {
    ($name:ident, $method:ident, $create_method:ident, $value:expr, $transaction:expr) => {
        #[test]
        fn $name() {
            let transactions = [ $transaction ];
            let mut dev = $create_method(&transactions);
            dev.$method($value).unwrap();
        }
    };
}

