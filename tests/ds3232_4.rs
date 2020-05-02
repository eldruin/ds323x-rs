extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;

extern crate ds323x;
use ds323x::TempConvRate;

#[allow(unused)]
mod common;
use common::{
    destroy_ds3232, destroy_ds3234, new_ds3232, new_ds3234, BitFlags as BF, Register,
    DEVICE_ADDRESS as DEV_ADDR, DS323X_POR_STATUS,
};

macro_rules! call_method_status_test {
    ($name:ident, $method:ident, $value:expr) => {
        mod $name {
            use super::*;
            call_test!(
                can_call_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                [I2cTrans::write(DEV_ADDR, vec![Register::STATUS, $value])]
            );
            call_test!(
                can_call_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                [SpiTrans::write(vec![Register::STATUS + 0x80, $value])]
            );
        }
    };
}

#[macro_export]
macro_rules! _set_param_test_2_4 {
    ($name:ident, $method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
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
macro_rules! set_param_test_2_4 {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _set_param_test_2_4!(
            $name,
            $method,
            $value,
            [I2cTrans::write(
                DEV_ADDR,
                vec![Register::$register, $binary_value]
            )],
            [SpiTrans::write(vec![
                Register::$register + 0x80,
                $binary_value
            ])]
        );
    };
}

const DEFAULT_WRITE_STATUS: u8 = DS323X_POR_STATUS | BF::ALARM2F | BF::ALARM1F;

call_method_status_test!(
    can_en_32khz_bat,
    enable_32khz_output_on_battery,
    DEFAULT_WRITE_STATUS | BF::BB32KHZ
);
call_method_status_test!(
    can_dis_32khz_bat,
    disable_32khz_output_on_battery,
    DEFAULT_WRITE_STATUS & !BF::BB32KHZ
);

set_param_test_2_4!(
    can_set_cr_64s,
    set_temperature_conversion_rate,
    STATUS,
    TempConvRate::_64s,
    DEFAULT_WRITE_STATUS & !BF::CRATE1 & !BF::CRATE0
);
set_param_test_2_4!(
    can_set_cr_128s,
    set_temperature_conversion_rate,
    STATUS,
    TempConvRate::_128s,
    DEFAULT_WRITE_STATUS & !BF::CRATE1 | BF::CRATE0
);
set_param_test_2_4!(
    can_set_cr_256s,
    set_temperature_conversion_rate,
    STATUS,
    TempConvRate::_256s,
    DEFAULT_WRITE_STATUS | BF::CRATE1 & !BF::CRATE0
);
set_param_test_2_4!(
    can_set_cr_512s,
    set_temperature_conversion_rate,
    STATUS,
    TempConvRate::_512s,
    DEFAULT_WRITE_STATUS | BF::CRATE1 | BF::CRATE0
);
