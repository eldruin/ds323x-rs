#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234, destroy_ds3231, destroy_ds3232,
              destroy_ds3234 };
extern crate ds323x;
use ds323x::{ Hours, DateTime, Error };

macro_rules! set_param_write_array_test {
    ($name:ident, $method:ident, $value:expr, $register:ident, [ $( $exp_bin:expr ),+ ] ) => {
        _set_param_test!($name, $method, $value,
            [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $( $exp_bin ),*]) ],
            [ SpiTrans::write(vec![Register::$register + 0x80, $( $exp_bin ),*]) ]);
    };
}

macro_rules! read_set_param_write_two_test {
    ($name:ident, $method:ident, $value:expr, $register:ident, $binary_value1_read:expr, $bin1:expr, $bin2:expr) => {
        _set_param_test!($name, $method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value1_read]),
              I2cTrans::write(DEV_ADDR, vec![Register::$register, $bin1, $bin2]) ],

            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value1_read]),
              SpiTrans::write(vec![Register::$register + 0x80, $bin1, $bin2]) ]);
    };
}

macro_rules! read_set_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value_read:expr, $binary_value_write:expr) => {
        _set_param_test!($name, $method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value_read]),
              I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value_write]) ],

            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value_read]),
              SpiTrans::write(vec![Register::$register + 0x80, $binary_value_write]) ]);
    };
}

macro_rules! set_invalid_param_test {
    ($name:ident, $method:ident, $value:expr) => {
        mod $name {
            use super::*;
            set_invalid_test!(cannot_set_invalid_ds3231, $method, new_ds3231, destroy_ds3231, $value);
            set_invalid_test!(cannot_set_invalid_ds3232, $method, new_ds3232, destroy_ds3232, $value);
            set_invalid_test!(cannot_set_invalid_ds3234, $method, new_ds3234, destroy_ds3234, $value);
        }
    };
}

macro_rules! set_invalid_param_range_test {
    ($name:ident, $method:ident, $too_small_value:expr, $too_big_value:expr) => {
        mod $name {
            use super::*;
            set_invalid_param_test!(too_small, $method, $too_small_value);
            set_invalid_param_test!(too_big,   $method, $too_big_value);
        }
    };
}

mod seconds {
    use super::*;
    get_param_test!(get, get_seconds, SECONDS, 1, 1);
    set_param_test!(set, set_seconds, SECONDS, 1, 1);
    set_invalid_param_test!(invalid, set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get, get_minutes, MINUTES, 1, 1);
    set_param_test!(set, set_minutes, MINUTES, 1, 1);
    set_invalid_param_test!(invalid, set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_param_test!(set, set_hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_invalid_param_test!(invalid, set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::AM(12), 0b0101_0010);
    set_param_test!(set, set_hours, HOURS, Hours::AM(12), 0b0101_0010);
    set_invalid_param_range_test!(invalid, set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get, get_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_param_test!(set, set_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_invalid_param_range_test!(invalid, set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get, get_weekday, DOW, 1, 1);
    set_param_test!(set, set_weekday, DOW, 1, 1);
    set_invalid_param_range_test!(invalid, set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get, get_day, DOM, 1, 1);
    set_param_test!(set, set_day, DOM, 1, 1);
    set_invalid_param_range_test!(invalid, set_day, 0, 8);
}

mod month {
    use super::*;
    get_param_test!(get, get_month, MONTH, 1, 1);
    read_set_param_test!(set, set_month, MONTH, 12, 0b0000_0010, 0b0001_0010);
    set_invalid_param_range_test!(invalid, set_month, 0, 13);

    mod keeps_century {
        use super::*;
        get_param_test!(get, get_month, MONTH, 12, 0b1001_0010);
        read_set_param_test!(set, set_month, MONTH, 12, 0b1000_0010, 0b1001_0010);
    }
}

mod year {
    use super::*;
    get_param_read_array_test!(century0_get, get_year, 2099, MONTH, [ 0, 0b1001_1001 ], [0, 0]);
    read_set_param_write_two_test!(century0_set, set_year, 2099, MONTH, 0b1001_0010, 0b0001_0010, 0b1001_1001);

    get_param_read_array_test!(century1_get, get_year, 2100, MONTH, [ 0b1000_0000, 0 ], [0, 0]);
    read_set_param_write_two_test!(century1_set, set_year, 2100, MONTH, 0b0001_0010, 0b1001_0010, 0);

    set_invalid_param_range_test!(invalid, set_year, 1999, 2101);
}

macro_rules! invalid_dt_test {
    ($name:ident, $year:expr, $month:expr, $day:expr, $weekday:expr,
     $hour:expr, $minute:expr, $second:expr) => {
        mod $name {
            use super::*;
            const DT : DateTime = DateTime { year: $year, month: $month, day: $day, weekday: $weekday,
                                             hour: $hour, minute: $minute, second: $second };
            set_invalid_param_test!($name, set_datetime, &DT);
        }
    };
}

mod datetime {
    use super::*;
    const DT : DateTime = DateTime { year: 2018, month: 8, day: 13, weekday: 2,
                                     hour: Hours::H24(23), minute: 59, second: 58 };
    get_param_read_array_test!(get, get_datetime, DT, SECONDS,
        [0b0101_1000, 0b0101_1001, 0b0010_0011, 0b0000_0010,
         0b0001_0011, 0b0000_1000, 0b0001_1000],
        [0, 0, 0, 0, 0, 0, 0]);

    set_param_write_array_test!(set, set_datetime, &DT, SECONDS,
        [0b0101_1000, 0b0101_1001, 0b0010_0011, 0b0000_0010,
         0b0001_0011, 0b0000_1000, 0b0001_1000]);

    invalid_dt_test!(too_small_year,  1999, 8,  13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_year,    2101, 8,  13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_month, 2018, 0,  13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_month,   2018, 13, 13, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_day,   2018, 8,   0, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_day,     2018, 8,  32, 2, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_small_wd,    2018, 8,  13, 0, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_wd,      2018, 8,  13, 8, Hours::H24(23), 59, 58);
    invalid_dt_test!(too_big_hours,   2018, 8,  13, 2, Hours::H24(24), 59, 58);
    invalid_dt_test!(too_big_min,     2018, 8,  13, 2, Hours::H24(24), 60, 58);
    invalid_dt_test!(too_big_seconds, 2018, 8,  13, 2, Hours::H24(24), 59, 60);
}

