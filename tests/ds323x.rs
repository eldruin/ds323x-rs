#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234 };
extern crate ds323x;
use ds323x::{ Hours, DateTime, Error };

macro_rules! _get_param_test {
    ($method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        get_test!(can_get_ds3231, $method, new_ds3231, $value, $i2c_transactions);
        get_test!(can_get_ds3232, $method, new_ds3232, $value, $i2c_transactions);
        get_test!(can_get_ds3234, $method, new_ds3234, $value, $spi_transactions);
    };
}

macro_rules! get_param_test {
    ($method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _get_param_test!($method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value]) ],
            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value]) ]);
    };
}

macro_rules! get_param_read_array_test {
    ($method:ident, $value:expr, $register1:ident, [ $( $bin:expr ),+ ], [ $( $bin2:expr ),+ ]) => {
        _get_param_test!($method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register1], vec![$( $bin ),*]) ],
            [ SpiTrans::transfer(vec![Register::$register1, $( $bin2 ),*], vec![Register::$register1, $( $bin ),*]) ]);
    };
}

macro_rules! _set_param_test {
    ($method:ident, $value:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        set_test!(can_set_ds3231, $method, new_ds3231, $value, $i2c_transactions);
        set_test!(can_set_ds3232, $method, new_ds3232, $value, $i2c_transactions);
        set_test!(can_set_ds3234, $method, new_ds3234, $value, $spi_transactions);
    };
}

macro_rules! set_param_test {
    ($method:ident, $register:ident, $value:expr, $binary_value:expr) => {
        _set_param_test!($method, $value,
            [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value]) ],
            [ SpiTrans::write(vec![Register::$register + 0x80, $binary_value]) ]);
    };
}

macro_rules! read_set_param_write_two_test {
    ($method:ident, $value:expr, $register:ident, $binary_value1_read:expr, $bin1:expr, $bin2:expr) => {
        _set_param_test!($method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value1_read]),
              I2cTrans::write(DEV_ADDR, vec![Register::$register, $bin1, $bin2]) ],

            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value1_read]),
              SpiTrans::write(vec![Register::$register + 0x80, $bin1, $bin2]) ]);
    };
}

macro_rules! read_set_param_test {
    ($method:ident, $register:ident, $value:expr, $binary_value_read:expr, $binary_value_write:expr) => {
        _set_param_test!($method, $value,
            [ I2cTrans::write_read(DEV_ADDR, vec![Register::$register], vec![$binary_value_read]),
              I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value_write]) ],

            [ SpiTrans::transfer(vec![Register::$register, 0], vec![Register::$register, $binary_value_read]),
              SpiTrans::write(vec![Register::$register + 0x80, $binary_value_write]) ]);
    };
}

macro_rules! set_invalid_param_test {
    ($method:ident, $value:expr) => {
        set_invalid_test!(cannot_set_invalid_ds3231, $method, new_ds3231, $value);
        set_invalid_test!(cannot_set_invalid_ds3232, $method, new_ds3232, $value);
        set_invalid_test!(cannot_set_invalid_ds3234, $method, new_ds3234, $value);
    };
}

macro_rules! set_invalid_param_range_test {
    ($method:ident, $too_small_value:expr, $too_big_value:expr) => {
        mod too_small {
            use super::*;
            set_invalid_param_test!($method, $too_small_value);
        }

        mod too_big {
            use super::*;
            set_invalid_param_test!($method, $too_big_value);
        }
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
    set_invalid_param_range_test!(set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_param_test!(set_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_invalid_param_range_test!(set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get_weekday, DOW, 1, 1);
    set_param_test!(set_weekday, DOW, 1, 1);
    set_invalid_param_range_test!(set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get_day, DOM, 1, 1);
    set_param_test!(set_day, DOM, 1, 1);
    set_invalid_param_range_test!(set_day, 0, 8);
}

mod month {
    use super::*;
    get_param_test!(get_month, MONTH, 1, 1);
    read_set_param_test!(set_month, MONTH, 12, 0b0000_0010, 0b0001_0010);
    set_invalid_param_range_test!(set_month, 0, 13);

    mod keeps_century {
        use super::*;
        get_param_test!(get_month, MONTH, 12, 0b1001_0010);
        read_set_param_test!(set_month, MONTH, 12, 0b1000_0010, 0b1001_0010);
    }
}

mod year {
    use super::*;
    mod century0 {
        use super::*;
        get_param_read_array_test!(get_year, 2099, MONTH, [ 0, 0b1001_1001 ], [0, 0]);
        read_set_param_write_two_test!(set_year, 2099, MONTH, 0b1001_0010, 0b0001_0010, 0b1001_1001);
    }
    mod century1 {
        use super::*;
        get_param_read_array_test!(get_year, 2100, MONTH, [ 0b1000_0000, 0 ], [0, 0]);
        read_set_param_write_two_test!(set_year, 2100, MONTH, 0b0001_0010, 0b1001_0010, 0);
    }
    set_invalid_param_range_test!(set_year, 1999, 2101);
}

mod datetime {
    use super::*;
    const DT : DateTime = DateTime { year: 2018, month: 8, day: 13, weekday: 2,
                              hour: Hours::H24(23), minute: 59, second: 58 };
    get_param_read_array_test!(get_datetime, DT, SECONDS,
        [0b0101_1000, 0b0101_1001, 0b0010_0011, 0b0000_0010,
         0b0001_0011, 0b0000_1000, 0b0001_1000],
        [0, 0, 0, 0, 0, 0, 0]);
}
