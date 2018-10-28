extern crate embedded_hal_mock as hal;
extern crate ds323x;
use ds323x::{ Ds323x, interface, ic };
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTrans };
use hal::spi::{ Mock as SpiMock, Transaction as SpiTrans };
mod common;
use common::{ DEVICE_ADDRESS, Register, DummyOutputPin };

fn new_ds3231(transactions: &[I2cTrans]) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3231> {
    Ds323x::new_ds3231(I2cMock::new(&transactions))
}

fn new_ds3232(transactions: &[I2cTrans]) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3232> {
    Ds323x::new_ds3232(I2cMock::new(&transactions))
}

fn new_ds3234(transactions: &[SpiTrans])
    -> Ds323x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::DS3234> {
    Ds323x::new_ds3234(SpiMock::new(&transactions), DummyOutputPin)
}

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

get_test!(can_get_seconds_ds3231, get_seconds, new_ds3231, 1,
          I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1]));

get_test!(can_get_seconds_ds3232, get_seconds, new_ds3232, 1,
          I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1]));

get_test!(can_get_seconds_ds3234, get_seconds, new_ds3234, 1,
          SpiTrans::transfer(vec![Register::SECONDS, 0], vec![Register::SECONDS, 1]));


set_test!(can_set_seconds_ds3231, set_seconds, new_ds3231, 1,
          I2cTrans::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1]));

set_test!(can_set_seconds_ds3232, set_seconds, new_ds3232, 1,
          I2cTrans::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1]));

set_test!(can_set_seconds_ds3234, set_seconds, new_ds3234, 1,
          SpiTrans::write(vec![Register::SECONDS + 0x80, 1]));
