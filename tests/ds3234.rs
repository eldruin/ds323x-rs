extern crate embedded_hal_mock as hal;
use hal::spi::Transaction as SpiTrans;
extern crate ds323x;

#[allow(unused)]
mod common;
use common::{ new_ds3234, destroy_ds3234, Register, BitFlags };

#[test]
fn can_create_and_destroy_ds3234() {
    let dev = new_ds3234(&[]);
    destroy_ds3234(dev);
}

call_test!(can_en_temp_conv_bat, enable_temperature_conversions_on_battery, new_ds3234, destroy_ds3234,
    [ SpiTrans::write(vec![Register::TEMP_CONV + 0x80, 0]) ]);

call_test!(can_dis_temp_conv_bat, disable_temperature_conversions_on_battery, new_ds3234, destroy_ds3234,
    [ SpiTrans::write(vec![Register::TEMP_CONV + 0x80, BitFlags::TEMP_CONV_BAT]) ]);