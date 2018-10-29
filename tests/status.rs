#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234, destroy_ds3231, destroy_ds3232,
              destroy_ds3234, BitFlags as BF };

get_param_test!(is_busy,     is_busy, STATUS, true,  0xFF);
get_param_test!(is_not_busy, is_busy, STATUS, false, 0xFF & !BF::BUSY);

get_param_test!(stopped,     has_been_stopped, STATUS, true,  0xFF);
get_param_test!(not_stopped, has_been_stopped, STATUS, false, 0xFF & !BF::OSC_STOP);

get_param_read_array_test!(temp_0,   get_temperature,    0.0,  TEMP_MSB, [0, 0], [0, 0]);
get_param_read_array_test!(temp_min, get_temperature, -128.0,  TEMP_MSB, [0b1000_0000, 0], [0, 0]);
get_param_read_array_test!(temp_max, get_temperature,  127.75, TEMP_MSB, [0b0111_1111, 0b1100_0000], [0, 0]);
