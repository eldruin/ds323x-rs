use embedded_hal_mock::eh1::spi::Transaction as SpiTrans;

#[allow(unused)]
mod common;
use self::common::{destroy_ds3234, new_ds3234, BitFlags, Register};

call_test!(
    can_en_temp_conv_bat,
    enable_temperature_conversions_on_battery,
    new_ds3234,
    destroy_ds3234,
    [SpiTrans::write_vec(vec![Register::TEMP_CONV + 0x80, 0])]
);

call_test!(
    can_dis_temp_conv_bat,
    disable_temperature_conversions_on_battery,
    new_ds3234,
    destroy_ds3234,
    [SpiTrans::write_vec(vec![
        Register::TEMP_CONV + 0x80,
        BitFlags::TEMP_CONV_BAT
    ])]
);
