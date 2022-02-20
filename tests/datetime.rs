use embedded_hal_mock::{i2c::Transaction as I2cTrans, spi::Transaction as SpiTrans};
mod common;
use self::common::{
    destroy_ds3231, destroy_ds3232, destroy_ds3234, new_ds3231, new_ds3232, new_ds3234, Register,
    DEVICE_ADDRESS as DEV_ADDR,
};
#[allow(unused)] // Rust 1.31.0 is confused due to the macros
use ds323x::Rtcc;
use ds323x::{DateTimeAccess, Error, Hours, NaiveDate, NaiveTime};

macro_rules! read_set_param_write_two_test {
    ($name:ident, $method:ident, $value:expr, $register:ident, $binary_value1_read:expr, $bin1:expr, $bin2:expr) => {
        _set_param_test!(
            $name,
            $method,
            $value,
            [
                I2cTrans::write_read(
                    DEV_ADDR,
                    vec![Register::$register],
                    vec![$binary_value1_read]
                ),
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $bin1, $bin2])
            ],
            [
                SpiTrans::transfer(
                    vec![Register::$register, 0],
                    vec![Register::$register, $binary_value1_read]
                ),
                SpiTrans::write(vec![Register::$register + 0x80, $bin1, $bin2])
            ]
        );
    };
}

macro_rules! read_set_param_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $binary_value_read:expr, $binary_value_write:expr) => {
        _set_param_test!(
            $name,
            $method,
            $value,
            [
                I2cTrans::write_read(
                    DEV_ADDR,
                    vec![Register::$register],
                    vec![$binary_value_read]
                ),
                I2cTrans::write(DEV_ADDR, vec![Register::$register, $binary_value_write])
            ],
            [
                SpiTrans::transfer(
                    vec![Register::$register, 0],
                    vec![Register::$register, $binary_value_read]
                ),
                SpiTrans::write(vec![Register::$register + 0x80, $binary_value_write])
            ]
        );
    };
}

macro_rules! set_invalid_param_test {
    ($name:ident, $method:ident, $value:expr) => {
        mod $name {
            use super::*;
            set_invalid_test!(
                cannot_set_invalid_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $value
            );
            set_invalid_test!(
                cannot_set_invalid_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $value
            );
            set_invalid_test!(
                cannot_set_invalid_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $value
            );
        }
    };
}

macro_rules! set_invalid_param_range_test {
    ($name:ident, $method:ident, $too_small_value:expr, $too_big_value:expr) => {
        mod $name {
            use super::*;
            set_invalid_param_test!(too_small, $method, $too_small_value);
            set_invalid_param_test!(too_big, $method, $too_big_value);
        }
    };
}

macro_rules! for_all {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_ds3231, new_ds3231, destroy_ds3231);
            $name!(for_ds3232, new_ds3232, destroy_ds3232);
            $name!(for_ds3234, new_ds3234, destroy_ds3234);
        }
    };
}

// TODO set/get date
// TODO set/get time

mod seconds {
    use super::*;
    get_param_test!(get, seconds, SECONDS, 1, 1);
    set_param_test!(set, set_seconds, SECONDS, 1, 1);
    set_invalid_param_test!(invalid, set_seconds, 60);
}

mod minutes {
    use super::*;
    get_param_test!(get, minutes, MINUTES, 1, 1);
    set_param_test!(set, set_minutes, MINUTES, 1, 1);
    set_invalid_param_test!(invalid, set_minutes, 60);
}

mod hours_24h {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_param_test!(set, set_hours, HOURS, Hours::H24(21), 0b0010_0001);
    set_invalid_param_test!(invalid, set_hours, Hours::H24(24));
}

mod hours_12h_am {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::AM(12), 0b0101_0010);
    set_param_test!(set, set_hours, HOURS, Hours::AM(12), 0b0101_0010);
    set_invalid_param_range_test!(invalid, set_hours, Hours::AM(0), Hours::AM(13));
}

mod hours_12h_pm {
    use super::*;
    get_param_test!(get, hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_param_test!(set, set_hours, HOURS, Hours::PM(12), 0b0111_0010);
    set_invalid_param_range_test!(invalid, set_hours, Hours::PM(0), Hours::PM(13));
}

mod weekday {
    use super::*;
    get_param_test!(get, weekday, DOW, 1, 1);
    set_param_test!(set, set_weekday, DOW, 1, 1);
    set_invalid_param_range_test!(invalid, set_weekday, 0, 8);
}

mod day {
    use super::*;
    get_param_test!(get, day, DOM, 1, 1);
    set_param_test!(set, set_day, DOM, 1, 1);
    set_invalid_param_range_test!(invalid, set_day, 0, 32);
}

mod month {
    use super::*;
    get_param_test!(get, month, MONTH, 1, 1);
    read_set_param_test!(set, set_month, MONTH, 12, 0b0000_0010, 0b0001_0010);
    set_invalid_param_range_test!(invalid, set_month, 0, 13);

    mod keeps_century {
        use super::*;
        get_param_test!(get, month, MONTH, 12, 0b1001_0010);
        read_set_param_test!(set, set_month, MONTH, 12, 0b1000_0010, 0b1001_0010);
    }
}

mod year {
    use super::*;
    get_param_read_array_test!(century0_get, year, 2099, MONTH, [0, 0b1001_1001], [0, 0]);
    read_set_param_write_two_test!(
        century0_set,
        set_year,
        2099,
        MONTH,
        0b1001_0010,
        0b0001_0010,
        0b1001_1001
    );

    get_param_read_array_test!(century1_get, year, 2100, MONTH, [0b1000_0000, 0], [0, 0]);
    read_set_param_write_two_test!(
        century1_set,
        set_year,
        2100,
        MONTH,
        0b0001_0010,
        0b1001_0010,
        0
    );

    set_invalid_param_range_test!(invalid, set_year, 1999, 2101);
}

macro_rules! invalid_dt_test {
    ($name:ident, $create_method:ident, $destroy_method:ident) => {
        mod $name {
            use super::*;
            #[test]
            fn datetime_too_small() {
                let dt = NaiveDate::from_ymd(1999, 1, 2).and_hms(3, 4, 5);
                let mut dev = $create_method(&[]);
                assert_invalid_input_data!(dev.set_datetime(&dt));
                $destroy_method(dev);
            }
            #[test]
            fn datetime_too_big() {
                let dt = NaiveDate::from_ymd(2101, 1, 2).and_hms(3, 4, 5);
                let mut dev = $create_method(&[]);
                assert_invalid_input_data!(dev.set_datetime(&dt));
                $destroy_method(dev);
            }
            #[test]
            fn date_too_small() {
                let d = NaiveDate::from_ymd(1999, 1, 2);
                let mut dev = $create_method(&[]);
                assert_invalid_input_data!(dev.set_date(&d));
                $destroy_method(dev);
            }
            #[test]
            fn date_too_big() {
                let d = NaiveDate::from_ymd(2101, 1, 2);
                let mut dev = $create_method(&[]);
                assert_invalid_input_data!(dev.set_date(&d));
                $destroy_method(dev);
            }
        }
    };
}

macro_rules! transactions_i2c_write {
    ($register:ident, [ $( $exp_bin:expr ),+ ]) => {
        [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $( $exp_bin ),*]) ]
    };
}

macro_rules! transactions_spi_write {
    ($register:ident, [ $( $exp_bin:expr ),+ ]) => {
            [ SpiTrans::write(vec![Register::$register + 0x80, $( $exp_bin ),*]) ]
    };
}

macro_rules! dt_test {
    ($name:ident, $create_method:ident, $destroy_method:ident,
    $mac_trans_read:ident, $mac_trans_write:ident) => {
        mod $name {
            use super::*;
            #[test]
            fn get_datetime() {
                let dt = NaiveDate::from_ymd(2018, 8, 13).and_hms(23, 59, 58);
                let mut dev = $create_method(&$mac_trans_read!(
                    SECONDS,
                    [
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0000_1000,
                        0b0001_1000
                    ],
                    [0, 0, 0, 0, 0, 0, 0]
                ));
                assert_eq!(dt, dev.datetime().unwrap());
                $destroy_method(dev);
            }

            #[test]
            fn set_datetime() {
                let dt = NaiveDate::from_ymd(2018, 8, 13).and_hms(23, 59, 58);
                let mut dev = $create_method(&$mac_trans_write!(
                    SECONDS,
                    [
                        0b0101_1000,
                        0b0101_1001,
                        0b0010_0011,
                        0b0000_0010,
                        0b0001_0011,
                        0b0000_1000,
                        0b0001_1000
                    ]
                ));
                dev.set_datetime(&dt).unwrap();
                $destroy_method(dev);
            }

            #[test]
            fn get_date() {
                let d = NaiveDate::from_ymd(2018, 8, 13);
                let mut dev = $create_method(&$mac_trans_read!(
                    DOM,
                    [0b0001_0011, 0b0000_1000, 0b0001_1000],
                    [0, 0, 0]
                ));
                assert_eq!(d, dev.date().unwrap());
                $destroy_method(dev);
            }

            #[test]
            fn set_date() {
                let d = NaiveDate::from_ymd(2018, 8, 13);
                let mut dev = $create_method(&$mac_trans_write!(
                    DOW,
                    [0b0000_0010, 0b0001_0011, 0b0000_1000, 0b0001_1000]
                ));
                dev.set_date(&d).unwrap();
                $destroy_method(dev);
            }

            #[test]
            fn set_date_century() {
                let d = NaiveDate::from_ymd(2100, 8, 13);
                let mut dev = $create_method(&$mac_trans_write!(
                    DOW,
                    [0b0000_0110, 0b0001_0011, 0b1000_1000, 0]
                ));
                dev.set_date(&d).unwrap();
                $destroy_method(dev);
            }

            #[test]
            fn get_time() {
                let t = NaiveTime::from_hms(23, 59, 58);
                let mut dev = $create_method(&$mac_trans_read!(
                    SECONDS,
                    [0b0101_1000, 0b0101_1001, 0b0010_0011],
                    [0, 0, 0]
                ));
                assert_eq!(t, dev.time().unwrap());
                $destroy_method(dev);
            }

            #[test]
            fn set_time() {
                let t = NaiveTime::from_hms(23, 59, 58);
                let mut dev = $create_method(&$mac_trans_write!(
                    SECONDS,
                    [0b0101_1000, 0b0101_1001, 0b0010_0011]
                ));
                dev.set_time(&t).unwrap();
                $destroy_method(dev);
            }
        }
    };
}

mod datetime {
    use super::*;

    dt_test!(
        for_ds3231,
        new_ds3231,
        destroy_ds3231,
        transactions_i2c_read,
        transactions_i2c_write
    );
    dt_test!(
        for_ds3232,
        new_ds3232,
        destroy_ds3232,
        transactions_i2c_read,
        transactions_i2c_write
    );
    dt_test!(
        for_ds3234,
        new_ds3234,
        destroy_ds3234,
        transactions_spi_read,
        transactions_spi_write
    );

    for_all!(invalid_dt_test);
}
