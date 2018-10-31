#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;

#[allow(unused)]
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register,
              new_ds3232, new_ds3234, destroy_ds3232,
              destroy_ds3234, BitFlags as BF, DS323X_POR_STATUS };

macro_rules! call_method_status_test {
    ($name:ident, $method:ident, $value:expr) => {
        mod $name {
            use super::*;
            call_test!(can_call_ds3232, $method, new_ds3232, destroy_ds3232,
                [ I2cTrans::write(DEV_ADDR, vec![Register::STATUS, $value]) ]);
            call_test!(can_call_ds3234, $method, new_ds3234, destroy_ds3234,
                [ SpiTrans::write(vec![Register::STATUS + 0x80, $value]) ]);
        }
    };
}

#[test]
fn can_create_and_destroy_ds3232() {
    let dev = new_ds3232(&[]);
    destroy_ds3232(dev);
}

#[test]
fn can_create_and_destroy_ds3234() {
    let dev = new_ds3234(&[]);
    destroy_ds3234(dev);
}

const DEFAULT_WRITE_STATUS: u8 = DS323X_POR_STATUS | BF::ALARM2F | BF::ALARM1F;

call_method_status_test!(can_en_32khz_bat, enable_32khz_output_on_battery,
    DEFAULT_WRITE_STATUS |  BF::BB32KHZ );
call_method_status_test!(can_dis_32khz_bat, disable_32khz_output_on_battery,
    DEFAULT_WRITE_STATUS & !BF::BB32KHZ);

