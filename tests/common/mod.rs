extern crate embedded_hal;
extern crate ds323x;
use self::ds323x::{ Ds323x, interface, ic };
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTrans };
use hal::spi::{ Mock as SpiMock, Transaction as SpiTrans };

pub const DEVICE_ADDRESS   : u8 = 0b110_1000;
#[allow(unused)]
pub const CONTROL_POR_VALUE: u8 = 0b0001_1100;

pub struct Register;

#[allow(unused)]
impl Register {
    pub const SECONDS      : u8 = 0x00;
    pub const MINUTES      : u8 = 0x01;
    pub const HOURS        : u8 = 0x02;
    pub const DOW          : u8 = 0x03;
    pub const DOM          : u8 = 0x04;
    pub const MONTH        : u8 = 0x05;
    pub const CONTROL      : u8 = 0x0E;
    pub const STATUS       : u8 = 0x0F;
    pub const AGING_OFFSET : u8 = 0x10;
    pub const TEMP_MSB     : u8 = 0x11;
}

pub struct BitFlags;

#[allow(unused)]
impl BitFlags {
    pub const EOSC       : u8 = 0b1000_0000;
    pub const TEMP_CONV  : u8 = 0b0010_0000;
    pub const BUSY       : u8 = 0b0000_0100;
    pub const EN32KHZ    : u8 = 0b0000_1000;
    pub const OSC_STOP   : u8 = 0b1000_0000;
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

pub fn destroy_ds3231(dev: Ds323x<interface::I2cInterface<I2cMock>, ic::DS3231>) {
    dev.destroy_ds3231().done();
}

pub fn destroy_ds3232(dev: Ds323x<interface::I2cInterface<I2cMock>, ic::DS3232>) {
    dev.destroy_ds3232().done();
}

pub fn destroy_ds3234(dev: Ds323x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::DS3234>) {
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
macro_rules! set_invalid_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $value:expr) => {
        #[test]
        fn $name() {
            let mut dev = $create_method(&[]);
            match dev.$method($value) {
                Err(Error::InvalidInputData) => (),
                _ => panic!("InvalidInputData error not returned.")
            }
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
            get_test!(can_get_ds3231, $method, new_ds3231, destroy_ds3231, $value, $i2c_transactions);
            get_test!(can_get_ds3232, $method, new_ds3232, destroy_ds3232, $value, $i2c_transactions);
            get_test!(can_get_ds3234, $method, new_ds3234, destroy_ds3234, $value, $spi_transactions);
        }
    };
}

#[macro_export]
macro_rules! get_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _get_param_test!($name, $method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value]) ],
            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value]) ]);
    };
}

#[macro_export]
macro_rules! get_param_read_array_test {
    ($name:ident, $method:ident, $value:expr, $register1:ident, [ $( $read_bin:expr ),+ ], [ $( $read_bin2:expr ),+ ]) => {
        _get_param_test!($name, $method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register1], vec![$( $read_bin ),*]) ],
            [ SpiTrans::transfer(vec![Register::$register1, $( $read_bin2 ),*], vec![Register::$register1, $( $read_bin ),*]) ]);
    };
}

#[macro_export]
macro_rules! _set_param_test {
    ($name:ident, $method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            set_test!(can_set_ds3231, $method, new_ds3231, destroy_ds3231, $value, $i2c_transactions);
            set_test!(can_set_ds3232, $method, new_ds3232, destroy_ds3232, $value, $i2c_transactions);
            set_test!(can_set_ds3234, $method, new_ds3234, destroy_ds3234, $value, $spi_transactions);
        }
    };
}

#[macro_export]
macro_rules! set_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _set_param_test!($name, $method, $value,
            [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value]) ],
            [ SpiTrans::write(vec![Register::$register + 0x80, $binary_value]) ]);
    };
}
