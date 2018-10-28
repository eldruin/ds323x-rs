#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234 };
extern crate ds323x;
use ds323x::{ Hours, Error };

macro_rules! get_param_test {
    ($method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        get_test!(can_get_ds3231, $method, new_ds3231, $value,
                I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value]));

        get_test!(can_get_ds3232, $method, new_ds3232, $value,
                I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value]));

        get_test!(can_get_ds3234, $method, new_ds3234, $value,
                SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value]));
    };
}

macro_rules! set_param_test {
    ($method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        set_test!(can_set_ds3231, $method, new_ds3231, $value,
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value]));

        set_test!(can_set_ds3232, $method, new_ds3232, $value,
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value]));

        set_test!(can_set_ds3234, $method, new_ds3234, $value,
                SpiTrans::write(vec![Register::$register + 0x80, $binary_value]));
    };
}

macro_rules! set_invalid_param_test {
    ($method:ident, $value:expr) => {
        set_invalid_test!(cannot_set_invalid_ds3231, $method, new_ds3231, $value);
        set_invalid_test!(cannot_set_invalid_ds3232, $method, new_ds3232, $value);
        set_invalid_test!(cannot_set_invalid_ds3234, $method, new_ds3234, $value);
    };
}

mod seconds {
    use super::*;
    get_param_test!(get_seconds, SECONDS, 1, 1);
    set_param_test!(set_seconds, SECONDS, 1, 1);
    set_invalid_param_test!(set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get_minutes, MINUTES, 1, 1);
    set_param_test!(set_minutes, MINUTES, 1, 1);
    set_invalid_param_test!(set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get_hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_param_test!(set_hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_invalid_param_test!(set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get_hours, HOURS, Hours::AM(12), 0b0101_0010);
    set_param_test!(set_hours, HOURS, Hours::AM(12), 0b0101_0010);

    mod too_small {
        use super::*;
        set_invalid_param_test!(set_hours, Hours::AM(0));
    }

    mod too_big {
        use super::*;
        set_invalid_param_test!(set_hours, Hours::AM(13));
    }
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_param_test!(set_hours, HOURS, Hours::PM(12), 0b0111_0010);

    mod too_small {
        use super::*;
        set_invalid_param_test!(set_hours, Hours::PM(0));
    }

    mod too_big {
        use super::*;
        set_invalid_param_test!(set_hours, Hours::PM(13));
    }
}
