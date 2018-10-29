#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234, destroy_ds3231, destroy_ds3232,
              destroy_ds3234, BitFlags as BF };

macro_rules! call_method_test {
    ($name:ident, $method:ident, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            call_test!(can_call_ds3231, $method, new_ds3231, destroy_ds3231, $i2c_transactions);
            call_test!(can_call_ds3232, $method, new_ds3232, destroy_ds3232, $i2c_transactions);
            call_test!(can_call_ds3234, $method, new_ds3234, destroy_ds3234, $spi_transactions);
        }
    };
}

macro_rules! change_if_necessary_test {
    ($name:ident, $method:ident, $register:ident, $value_enabled:expr, $value_disabled:expr) => {
        mod $name {
            use super::*;
            call_method_test!(do_nothing_if_not_necessary, $method,
                [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$value_enabled]) ],
                [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $value_enabled]) ]);

            call_method_test!(change, $method,
                [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$value_disabled]),
                  I2cTrans::write(DEV_ADDR, vec![Register::$register, $value_enabled]) ],

                [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $value_disabled]),
                  SpiTrans::write(vec![Register::$register + 0x80, $value_enabled]) ]);
        }
    };
}

change_if_necessary_test!(enable, enable, CONTROL, 0xFF & !BF::EOSC,  0xFF);
change_if_necessary_test!(disable, disable, CONTROL, 0xFF, 0xFF & !BF::EOSC);

