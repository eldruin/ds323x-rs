extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS, Register, new_ds3231,
              new_ds3232, new_ds3234 };

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
