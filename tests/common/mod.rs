use ds323x::{ic, interface, Ds323x};
use embedded_hal_mock::eh1::{
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
    spi::{Mock as SpiMock, Transaction as SpiTrans},
};

#[allow(unused)]
pub const DEVICE_ADDRESS: u8 = 0b110_1000;
#[allow(unused)]
pub const CONTROL_POR_VALUE: u8 = 0b0001_1100;
#[allow(unused)]
pub const DS3231_POR_STATUS: u8 = BitFlags::OSC_STOP | BitFlags::EN32KHZ;
#[allow(unused)]
pub const DS323X_POR_STATUS: u8 = BitFlags::OSC_STOP | BitFlags::BB32KHZ | BitFlags::EN32KHZ;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x01;
    pub const HOURS: u8 = 0x02;
    pub const DOW: u8 = 0x03;
    pub const DOM: u8 = 0x04;
    pub const MONTH: u8 = 0x05;
    pub const ALARM1_SECONDS: u8 = 0x07;
    pub const ALARM2_MINUTES: u8 = 0x0B;
    pub const CONTROL: u8 = 0x0E;
    pub const STATUS: u8 = 0x0F;
    pub const AGING_OFFSET: u8 = 0x10;
    pub const TEMP_MSB: u8 = 0x11;
    pub const TEMP_CONV: u8 = 0x13;
}

pub struct BitFlags;

#[allow(unused)]
impl BitFlags {
    pub const EOSC: u8 = 0b1000_0000;
    pub const BBSQW: u8 = 0b0100_0000;
    pub const TEMP_CONV: u8 = 0b0010_0000;
    pub const RS2: u8 = 0b0001_0000;
    pub const RS1: u8 = 0b0000_1000;
    pub const INTCN: u8 = 0b0000_0100;
    pub const ALARM2_INT_EN: u8 = 0b0000_0010;
    pub const ALARM1_INT_EN: u8 = 0b0000_0001;
    pub const OSC_STOP: u8 = 0b1000_0000;
    pub const BB32KHZ: u8 = 0b0100_0000;
    pub const CRATE1: u8 = 0b0010_0000;
    pub const CRATE0: u8 = 0b0001_0000;
    pub const EN32KHZ: u8 = 0b0000_1000;
    pub const BUSY: u8 = 0b0000_0100;
    pub const ALARM2F: u8 = 0b0000_0010;
    pub const ALARM1F: u8 = 0b0000_0001;
    pub const TEMP_CONV_BAT: u8 = 0b0000_0001;
    pub const ALARM_MATCH: u8 = 0b1000_0000;
    pub const WEEKDAY: u8 = 0b0100_0000;
}

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_hal::digital::ErrorType for DummyOutputPin {
    type Error = embedded_hal::digital::ErrorKind;
}

pub fn new_ds3231(
    transactions: &[I2cTrans],
) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3231> {
    Ds323x::new_ds3231(I2cMock::new(transactions))
}

pub fn new_ds3232(
    transactions: &[I2cTrans],
) -> Ds323x<interface::I2cInterface<I2cMock>, ic::DS3232> {
    Ds323x::new_ds3232(I2cMock::new(transactions))
}

pub fn new_ds3234(
    transactions: &[SpiTrans<u8>],
) -> Ds323x<interface::SpiInterface<SpiMock<u8>, DummyOutputPin>, ic::DS3234> {
    Ds323x::new_ds3234(SpiMock::new(transactions), DummyOutputPin)
}

pub fn destroy_ds3231(dev: Ds323x<interface::I2cInterface<I2cMock>, ic::DS3231>) {
    dev.destroy_ds3231().done();
}

pub fn destroy_ds3232(dev: Ds323x<interface::I2cInterface<I2cMock>, ic::DS3232>) {
    dev.destroy_ds3232().done();
}

pub fn destroy_ds3234(dev: Ds323x<interface::SpiInterface<SpiMock<u8>, DummyOutputPin>, ic::DS3234>) {
    dev.destroy_ds3234().0.done();
}

#[macro_export]
macro_rules! get_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $expected:expr, $transactions:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            assert_eq!($expected, dev.$method().unwrap());
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! set_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $value:expr, $transactions:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method($value).unwrap();
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! assert_invalid_input_data {
    ($result:expr) => {
        match $result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error not returned."),
        }
    };
}

#[macro_export]
macro_rules! set_invalid_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $value:expr) => {
        #[test]
        fn $name() {
            let mut dev = $create_method(&[]);
            assert_invalid_input_data!(dev.$method($value));
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! call_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $transactions:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method().unwrap();
            $destroy_method(dev);
        }
    };
}

#[macro_export]
macro_rules! _get_param_test {
    ($name:ident, $method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            get_test!(
                can_get_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $value,
                $i2c_transactions
            );
            get_test!(
                can_get_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $value,
                $i2c_transactions
            );
            get_test!(
                can_get_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $value,
                $spi_transactions
            );
        }
    };
}

#[macro_export]
macro_rules! get_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _get_param_test!(
            $name,
            $method,
            $value,
            [I2cTrans::write_read(
                DEV_ADDR,
                vec![Register::$register],
                vec![$binary_value]
            )],
            [SpiTrans::transfer(
                vec![Register::$register, 0],
                vec![Register::$register, $binary_value]
            )]
        );
    };
}

#[macro_export]
macro_rules! transactions_i2c_read {
    ($register1:ident, [ $( $read_bin:expr ),+ ], [ $( $read_bin2:expr ),* ]) => {
        [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register1], vec![$( $read_bin ),*]) ]
    }
}

#[macro_export]
macro_rules! transactions_spi_read {
    ($register1:ident, [ $( $read_bin:expr ),+ ], [ $( $read_bin2:expr ),+ ]) => {
        [ SpiTrans::transfer(vec![Register::$register1, $( $read_bin2 ),*], vec![Register::$register1, $( $read_bin ),*]) ]
    }
}

#[macro_export]
macro_rules! get_param_read_array_test {
    ($name:ident, $method:ident, $value:expr, $register1:ident, [ $( $read_bin:expr ),+ ], [ $( $read_bin2:expr ),+ ]) => {
        _get_param_test!($name, $method, $value,
            transactions_i2c_read!($register1, [ $( $read_bin ),* ], [ ]),
            transactions_spi_read!($register1, [ $( $read_bin ),* ], [ $( $read_bin2 ),* ]) );
    };
}

#[macro_export]
macro_rules! _set_param_test {
    ($name:ident, $method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            set_test!(
                can_set_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $value,
                $i2c_transactions
            );
            set_test!(
                can_set_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $value,
                $i2c_transactions
            );
            set_test!(
                can_set_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $value,
                $spi_transactions
            );
        }
    };
}

#[macro_export]
macro_rules! set_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _set_param_test!(
            $name,
            $method,
            $value,
            [I2cTrans::write(
                DEV_ADDR,
                vec![Register::$register, $binary_value]
            )],
            [SpiTrans::write_vec(vec![
                Register::$register + 0x80,
                $binary_value
            ])]
        );
    };
}
