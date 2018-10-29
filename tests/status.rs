#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234, destroy_ds3231, destroy_ds3232,
              destroy_ds3234, BitFlags as BF };

get_param_test!(is_busy,     is_busy, STATUS, true, 0xFF);
get_param_test!(is_not_busy, is_busy, STATUS, false, 0xFF & !BF::BUSY);