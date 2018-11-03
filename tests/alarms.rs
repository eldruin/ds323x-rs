#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register, new_ds3231,
              new_ds3232, new_ds3234, destroy_ds3231, destroy_ds3232,
              destroy_ds3234, BitFlags as BF };
extern crate ds323x;
use ds323x::{ DateAlarm1, WeekdayAlarm1, Alarm1Matching as A1M, Hours, Error };
use ds323x::{ DateAlarm1, Alarm1Matching as A1M, Hours, Error };

#[macro_export]
macro_rules! _set_invalid_alarm_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $alarm:expr, $matching:expr) => {
        #[test]
        fn $name() {
            let mut dev = $create_method(&[]);
            assert_invalid_input_data!(dev.$method($alarm, $matching));
            $destroy_method(dev);
        }
    };
}

macro_rules! set_invalid_alarm_test {
    ($name:ident, $method:ident, $alarm:expr, $matching:expr) => {
        mod $name {
            use super::*;
            _set_invalid_alarm_test!(cannot_set_invalid_ds3231, $method, new_ds3231, destroy_ds3231, $alarm, $matching);
            _set_invalid_alarm_test!(cannot_set_invalid_ds3232, $method, new_ds3232, destroy_ds3232, $alarm, $matching);
            _set_invalid_alarm_test!(cannot_set_invalid_ds3234, $method, new_ds3234, destroy_ds3234, $alarm, $matching);
        }
    };
}

mod alarm1 {
    use super::*;
    set_invalid_alarm_test!(date_invalid_s,   set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(1),  minute: 1,  second: 60 }, A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_min, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(1),  minute: 60, second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_h,   set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(24), minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_am1, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::AM(0),   minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_am2, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::AM(13),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_pm1, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::PM(0),   minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_pm2, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::PM(13),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_d1,  set_alarm1_date, DateAlarm1{ date: 0,  hour: Hours::H24(1),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(date_invalid_d2,  set_alarm1_date, DateAlarm1{ date: 32, hour: Hours::H24(1),  minute: 1,  second: 1 },  A1M::AllMatch);

    set_invalid_alarm_test!(wd_invalid_s,   set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(1),  minute: 1,  second: 60 }, A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_min, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(1),  minute: 60, second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_h,   set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(24), minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_am1, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::AM(0),   minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_am2, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::AM(13),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_pm1, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::PM(0),   minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_pm2, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::PM(13),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_d1,  set_alarm1_weekday, WeekdayAlarm1{ weekday: 0,  hour: Hours::H24(1),  minute: 1,  second: 1 },  A1M::AllMatch);
    set_invalid_alarm_test!(wd_invalid_d2,  set_alarm1_weekday, WeekdayAlarm1{ weekday: 32, hour: Hours::H24(1),  minute: 1,  second: 1 },  A1M::AllMatch);
}
}

macro_rules! _set_values_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $value1:expr, $value2:expr, $transactions:expr) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method($value1, $value2).unwrap();
            $destroy_method(dev);
        }
    };
}

macro_rules! set_values_test {
    ($name:ident, $method:ident, $value1:expr, $value2:expr, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            _set_values_test!(can_set_ds3231, $method, new_ds3231, destroy_ds3231, $value1, $value2, $i2c_transactions);
            _set_values_test!(can_set_ds3232, $method, new_ds3232, destroy_ds3232, $value1, $value2, $i2c_transactions);
            _set_values_test!(can_set_ds3234, $method, new_ds3234, destroy_ds3234, $value1, $value2, $spi_transactions);
        }
    };
}

macro_rules! set_alarm_test {
    ($name:ident, $method:ident, $alarm:expr, $matching:expr, $register:ident, [ $( $registers:expr ),+ ]) => {
        set_values_test!($name, $method, $alarm, $matching,
            [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $( $registers ),*]) ],
            [ SpiTrans::write(vec![Register::$register + 0x80, $( $registers ),*]) ]);
    };
}

const AM : u8 = BF::ALARM_MATCH;

mod alarm1_date {
    use super::*;
    set_alarm_test!(h24, set_alarm1_date, DateAlarm1{ date: 1, hour: Hours::H24(2), minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 2, 1]);
    set_alarm_test!(am,  set_alarm1_date, DateAlarm1{ date: 1, hour: Hours::AM(2),  minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 0b0100_0010, 1]);
    set_alarm_test!(pm,  set_alarm1_date, DateAlarm1{ date: 1, hour: Hours::PM(2),  minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 0b0110_0010, 1]);

    set_alarm_test!(match_hms, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::HoursMinutesAndSecondsMatch,
                    ALARM1_SECONDS, [     4,      3,      2, AM | 1]);
    set_alarm_test!(match_ms,  set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::MinutesAndSecondsMatch,
                    ALARM1_SECONDS, [     4,      3, AM | 2, AM | 1]);
    set_alarm_test!(match_s,   set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::SecondsMatch,
                    ALARM1_SECONDS, [     4, AM | 3, AM | 2, AM | 1]);
    set_alarm_test!(match_ops, set_alarm1_date, DateAlarm1{ date: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::OncePerSecond,
                    ALARM1_SECONDS, [AM | 4, AM | 3, AM | 2, AM | 1]);
}

mod alarm1_weekday {
    use super::*;
    set_alarm_test!(h24, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 2,           BF::WEEKDAY | 1]);
    set_alarm_test!(am,  set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::AM(2),  minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 0b0100_0010, BF::WEEKDAY | 1]);
    set_alarm_test!(pm,  set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::PM(2),  minute: 3, second: 4 }, A1M::AllMatch,
                    ALARM1_SECONDS, [4, 3, 0b0110_0010, BF::WEEKDAY | 1]);

    set_alarm_test!(match_hms, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::HoursMinutesAndSecondsMatch,
                    ALARM1_SECONDS, [     4,      3,      2, AM | BF::WEEKDAY | 1]);
    set_alarm_test!(match_ms,  set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::MinutesAndSecondsMatch,
                    ALARM1_SECONDS, [     4,      3, AM | 2, AM | BF::WEEKDAY | 1]);
    set_alarm_test!(match_s,   set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::SecondsMatch,
                    ALARM1_SECONDS, [     4, AM | 3, AM | 2, AM | BF::WEEKDAY | 1]);
    set_alarm_test!(match_ops, set_alarm1_weekday, WeekdayAlarm1{ weekday: 1,  hour: Hours::H24(2), minute: 3, second: 4 }, A1M::OncePerSecond,
                    ALARM1_SECONDS, [AM | 4, AM | 3, AM | 2, AM | BF::WEEKDAY | 1]);
}
