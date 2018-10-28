#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS, Register, new_ds3231,
              new_ds3232, new_ds3234 };
extern crate ds323x;
use ds323x::{ Hours, Error };

mod seconds {
    use super::*;
    get_test!(can_get_ds3231, get_seconds, new_ds3231, 1,
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1]));

    get_test!(can_get_ds3232, get_seconds, new_ds3232, 1,
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::SECONDS], vec![1]));

    get_test!(can_get_ds3234, get_seconds, new_ds3234, 1,
            SpiTrans::transfer(vec![Register::SECONDS, 0], vec![Register::SECONDS, 1]));


    set_test!(can_set_ds3231, set_seconds, new_ds3231, 1,
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1]));

    set_test!(can_set_ds3232, set_seconds, new_ds3232, 1,
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::SECONDS, 1]));

    set_test!(can_set_ds3234, set_seconds, new_ds3234, 1,
            SpiTrans::write(vec![Register::SECONDS + 0x80, 1]));

    set_invalid_test!(cannot_set_invalid_ds3231, set_seconds, new_ds3231, 60);
    set_invalid_test!(cannot_set_invalid_ds3232, set_seconds, new_ds3232, 60);
    set_invalid_test!(cannot_set_invalid_ds3234, set_seconds, new_ds3234, 60);
}

mod minutes {
    use super::*;
    get_test!(can_get_ds3231, get_minutes, new_ds3231, 1,
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::MINUTES], vec![1]));

    get_test!(can_get_ds3232, get_minutes, new_ds3232, 1,
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::MINUTES], vec![1]));

    get_test!(can_get_ds3234, get_minutes, new_ds3234, 1,
            SpiTrans::transfer(vec![Register::MINUTES, 0], vec![Register::MINUTES, 1]));


    set_test!(can_set_ds3231, set_minutes, new_ds3231, 1,
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::MINUTES, 1]));

    set_test!(can_set_ds3232, set_minutes, new_ds3232, 1,
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::MINUTES, 1]));

    set_test!(can_set_ds3234, set_minutes, new_ds3234, 1,
            SpiTrans::write(vec![Register::MINUTES + 0x80, 1]));

    set_invalid_test!(cannot_set_invalid_ds3231, set_minutes, new_ds3231, 60);
    set_invalid_test!(cannot_set_invalid_ds3232, set_minutes, new_ds3232, 60);
    set_invalid_test!(cannot_set_invalid_ds3234, set_minutes, new_ds3234, 60);
}

mod hours {
    use super::*;
    get_test!(can_get_ds3231, get_hours, new_ds3231, Hours::H24(21),
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::HOURS], vec![0b0010_0001]));

    get_test!(can_get_ds3232, get_hours, new_ds3232, Hours::H24(21),
            I2cTrans::write_read(DEVICE_ADDRESS, vec![Register::HOURS], vec![0b0010_0001]));

    get_test!(can_get_ds3234, get_hours, new_ds3234, Hours::H24(21),
            SpiTrans::transfer(vec![Register::HOURS, 0], vec![Register::HOURS, 0b0010_0001]));


    set_test!(can_set_ds3231, set_hours, new_ds3231, Hours::H24(21),
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::HOURS, 0b0010_0001]));

    set_test!(can_set_ds3232, set_hours, new_ds3232, Hours::H24(21),
            I2cTrans::write(DEVICE_ADDRESS, vec![Register::HOURS, 0b0010_0001]));

    set_test!(can_set_ds3234, set_hours, new_ds3234, Hours::H24(21),
            SpiTrans::write(vec![Register::HOURS + 0x80, 0b0010_0001]));

    set_invalid_test!(cannot_set_invalid_ds3231, set_hours, new_ds3231, Hours::H24(24));
    set_invalid_test!(cannot_set_invalid_ds3232, set_hours, new_ds3232, Hours::H24(24));
    set_invalid_test!(cannot_set_invalid_ds3234, set_hours, new_ds3234, Hours::H24(24));
}
