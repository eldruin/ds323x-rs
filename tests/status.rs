use embedded_hal_mock::{i2c::Transaction as I2cTrans, spi::Transaction as SpiTrans};
mod common;
use common::{
    destroy_ds3231, destroy_ds3232, destroy_ds3234, new_ds3231, new_ds3232, new_ds3234,
    BitFlags as BF, Register, DEVICE_ADDRESS as DEV_ADDR,
};

get_param_test!(running, running, CONTROL, true, 0);
get_param_test!(is_not_running, running, CONTROL, false, BF::EOSC);

get_param_test!(busy, busy, STATUS, true, 0xFF);
get_param_test!(is_not_busy, busy, STATUS, false, !BF::BUSY);

get_param_test!(stopped, has_been_stopped, STATUS, true, 0xFF);
get_param_test!(not_stopped, has_been_stopped, STATUS, false, !BF::OSC_STOP);

get_param_test!(alarm1_matched, has_alarm1_matched, STATUS, true, 0xFF);
get_param_test!(
    alarm1_not_matched,
    has_alarm1_matched,
    STATUS,
    false,
    !BF::ALARM1F
);

get_param_test!(alarm2_matched, has_alarm2_matched, STATUS, true, 0xFF);
get_param_test!(
    alarm2_not_matched,
    has_alarm2_matched,
    STATUS,
    false,
    !BF::ALARM2F
);

get_param_read_array_test!(temp_0, get_temperature, 0.0, TEMP_MSB, [0, 0], [0, 0]);
get_param_read_array_test!(
    temp_min,
    get_temperature,
    -128.0,
    TEMP_MSB,
    [0b1000_0000, 0],
    [0, 0]
);
get_param_read_array_test!(
    temp_max,
    get_temperature,
    127.75,
    TEMP_MSB,
    [0b0111_1111, 0b1100_0000],
    [0, 0]
);
