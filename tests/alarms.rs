use embedded_hal_mock::{i2c::Transaction as I2cTrans, spi::Transaction as SpiTrans};
mod common;
use self::common::{
    destroy_ds3231, destroy_ds3232, destroy_ds3234, new_ds3231, new_ds3232, new_ds3234,
    BitFlags as BF, Register, DEVICE_ADDRESS as DEV_ADDR,
};
use ds323x::{
    Alarm1Matching as A1M, Alarm2Matching as A2M, DayAlarm1, DayAlarm2, Error, Hours, NaiveTime,
    WeekdayAlarm1, WeekdayAlarm2,
};

#[macro_export]
macro_rules! _set_invalid_alarm_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $( $value:expr ),+) => {
        #[test]
        fn $name() {
            let mut dev = $create_method(&[]);
            assert_invalid_input_data!(dev.$method($($value),*));
            $destroy_method(dev);
        }
    };
}

macro_rules! set_invalid_alarm_test {
    ($name:ident, $method:ident, $( $value:expr ),+) => {
        mod $name {
            use super::*;
            _set_invalid_alarm_test!(
                cannot_set_invalid_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $($value),*
            );
            _set_invalid_alarm_test!(
                cannot_set_invalid_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $($value),*
            );
            _set_invalid_alarm_test!(
                cannot_set_invalid_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $($value),*
            );
        }
    };
}

mod alarm1 {
    use super::*;
    set_invalid_alarm_test!(
        day_invalid_s,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(1),
            minute: 1,
            second: 60
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_min,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(1),
            minute: 60,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_second,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(1),
            minute: 59,
            second: 60
        },
        A1M::SecondsMatch
    );
    set_invalid_alarm_test!(
        day_invalid_minute,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(1),
            minute: 60,
            second: 10
        },
        A1M::MinutesAndSecondsMatch
    );
    set_invalid_alarm_test!(
        day_invalid_h,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(24),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_h_hmasm,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(24),
            minute: 1,
            second: 1
        },
        A1M::HoursMinutesAndSecondsMatch
    );
    set_invalid_alarm_test!(
        day_invalid_am1,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::AM(0),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_am2,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::AM(13),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_pm1,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::PM(0),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_pm2,
        set_alarm1_day,
        DayAlarm1 {
            day: 1,
            hour: Hours::PM(13),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_d1,
        set_alarm1_day,
        DayAlarm1 {
            day: 0,
            hour: Hours::H24(1),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_d2,
        set_alarm1_day,
        DayAlarm1 {
            day: 32,
            hour: Hours::H24(1),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );

    set_invalid_alarm_test!(
        wd_invalid_s,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 1,
            second: 60
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_min,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 60,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_h,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(24),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_h_hmasm,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(24),
            minute: 1,
            second: 1
        },
        A1M::HoursMinutesAndSecondsMatch
    );

    set_invalid_alarm_test!(
        wd_invalid_am1,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::AM(0),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_am2,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::AM(13),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_pm1,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::PM(0),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_pm2,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::PM(13),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_d1,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 0,
            hour: Hours::H24(1),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_d2,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 32,
            hour: Hours::H24(1),
            minute: 1,
            second: 1
        },
        A1M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_sec_sm,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 1,
            second: 60
        },
        A1M::SecondsMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_min_masm,
        set_alarm1_weekday,
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 60,
            second: 1
        },
        A1M::MinutesAndSecondsMatch
    );
}

mod alarm2 {
    use super::*;
    set_invalid_alarm_test!(
        day_invalid_min_mm,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(1),
            minute: 60
        },
        A2M::MinutesMatch
    );
    set_invalid_alarm_test!(
        day_invalid_min,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(1),
            minute: 60
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_h,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(24),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_h_hamm,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(24),
            minute: 1
        },
        A2M::HoursAndMinutesMatch
    );
    set_invalid_alarm_test!(
        day_invalid_am1,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::AM(0),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_am2,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::AM(13),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_pm1,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::PM(0),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_pm2,
        set_alarm2_day,
        DayAlarm2 {
            day: 1,
            hour: Hours::PM(13),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_d1,
        set_alarm2_day,
        DayAlarm2 {
            day: 0,
            hour: Hours::H24(1),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        day_invalid_d2,
        set_alarm2_day,
        DayAlarm2 {
            day: 32,
            hour: Hours::H24(1),
            minute: 1
        },
        A2M::AllMatch
    );

    set_invalid_alarm_test!(
        wd_invalid_min_mm,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 60
        },
        A2M::MinutesMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_min,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 60
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_h,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(24),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_h_hmm,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(24),
            minute: 1
        },
        A2M::HoursAndMinutesMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_am1,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::AM(0),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_am2,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::AM(13),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_pm1,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::PM(0),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_pm2,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::PM(13),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_d1,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 0,
            hour: Hours::H24(1),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_d2,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 32,
            hour: Hours::H24(1),
            minute: 1
        },
        A2M::AllMatch
    );
    set_invalid_alarm_test!(
        wd_invalid_minute,
        set_alarm2_weekday,
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(1),
            minute: 60
        },
        A2M::HoursAndMinutesMatch
    );
}

macro_rules! _set_values_test {
    ($name:ident, $method:ident, $create_method:ident, $destroy_method:ident, $transactions:expr, $( $value:expr ),+) => {
        #[test]
        fn $name() {
            let trans = $transactions;
            let mut dev = $create_method(&trans);
            dev.$method($($value),*).unwrap();
            $destroy_method(dev);
        }
    };
}

macro_rules! set_values_test {
    ($name:ident, $method:ident, $i2c_transactions:expr, $spi_transactions:expr, $( $value:expr ),+) => {
        mod $name {
            use super::*;
            _set_values_test!(
                can_set_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $i2c_transactions,
                $($value),*
            );
            _set_values_test!(
                can_set_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $i2c_transactions,
                $($value),*
            );
            _set_values_test!(
                can_set_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $spi_transactions,
                $($value),*
            );
        }
    };
}

macro_rules! set_alarm_test {
    ($name:ident, $method:ident, $register:ident, [ $( $registers:expr ),+ ], $( $value:expr ),+) => {
        set_values_test!($name, $method,
            [ I2cTrans::write(DEV_ADDR, vec![Register::$register, $( $registers ),*]) ],
            [ SpiTrans::write(vec![Register::$register + 0x80, $( $registers ),*]) ],
            $($value),*
        );
    };
}

const AM: u8 = BF::ALARM_MATCH;

mod alarm1_day {
    use super::*;
    set_alarm_test!(
        h24,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, 2, 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        am,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, 0b0100_0010, 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::AM(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        pm,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, 0b0110_0010, 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::PM(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        match_hms_naivetime,
        set_alarm1_hms,
        ALARM1_SECONDS,
        [4, 3, 2, AM | 1],
        NaiveTime::from_hms_opt(2, 3, 4).unwrap()
    );
    set_alarm_test!(
        match_hms,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, 2, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::HoursMinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_hms_ignore_incorrect_day,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, 2, AM | 1],
        DayAlarm1 {
            day: 0,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::HoursMinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_ms_ignore_invalid_hour,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, AM, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(24),
            minute: 3,
            second: 4
        },
        A1M::MinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_ms,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, AM | 2, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::MinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_ms_ignore_incorrect_day,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, 3, AM | 2, AM | 1],
        DayAlarm1 {
            day: 0,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::MinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_s,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, AM | 3, AM | 2, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::SecondsMatch
    );
    set_alarm_test!(
        match_s_ignore_incorrect_min,
        set_alarm1_day,
        ALARM1_SECONDS,
        [4, AM, AM | 2, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 60,
            second: 4
        },
        A1M::SecondsMatch
    );
    set_alarm_test!(
        match_ops,
        set_alarm1_day,
        ALARM1_SECONDS,
        [AM | 4, AM | 3, AM | 2, AM | 1],
        DayAlarm1 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::OncePerSecond
    );
}

mod alarm1_weekday {
    use super::*;
    set_alarm_test!(
        h24,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, 2, BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        am,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, 0b0100_0010, BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::AM(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        pm,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, 0b0110_0010, BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::PM(2),
            minute: 3,
            second: 4
        },
        A1M::AllMatch
    );
    set_alarm_test!(
        match_hms_ignore_incorrect_wd,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 0,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::HoursMinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_hms,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::HoursMinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_ms,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, 3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::MinutesAndSecondsMatch
    );
    set_alarm_test!(
        match_s,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, AM | 3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::SecondsMatch
    );
    set_alarm_test!(
        match_s_ignore_incorrect_min,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [4, AM, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 60,
            second: 4
        },
        A1M::SecondsMatch
    );
    set_alarm_test!(
        match_ops,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [AM | 4, AM | 3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 4
        },
        A1M::OncePerSecond
    );
    set_alarm_test!(
        match_ops_ignore_incorrect_sec,
        set_alarm1_weekday,
        ALARM1_SECONDS,
        [AM, AM | 3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm1 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3,
            second: 60
        },
        A1M::OncePerSecond
    );
}

mod alarm2_day {
    use super::*;
    set_alarm_test!(
        h24,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, 2, 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        am,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, 0b0100_0010, 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::AM(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        pm,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, 0b0110_0010, 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::PM(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        match_hm_naivetime,
        set_alarm2_hm,
        ALARM2_MINUTES,
        [3, 2, AM | 1],
        NaiveTime::from_hms_opt(2, 3, 0).unwrap()
    );
    set_alarm_test!(
        match_hm,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, 2, AM | 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::HoursAndMinutesMatch
    );
    set_alarm_test!(
        match_hm_ignore_incorrect_day,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, 2, AM | 1],
        DayAlarm2 {
            day: 0,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::HoursAndMinutesMatch
    );
    set_alarm_test!(
        match_m,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, AM | 2, AM | 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::MinutesMatch
    );
    set_alarm_test!(
        match_m_ignore_invalid_h,
        set_alarm2_day,
        ALARM2_MINUTES,
        [3, AM, AM | 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(25),
            minute: 3
        },
        A2M::MinutesMatch
    );
    set_alarm_test!(
        match_opm,
        set_alarm2_day,
        ALARM2_MINUTES,
        [AM | 3, AM | 2, AM | 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::OncePerMinute
    );
    set_alarm_test!(
        match_opm_ignore_incorrect_min,
        set_alarm2_day,
        ALARM2_MINUTES,
        [AM, AM | 2, AM | 1],
        DayAlarm2 {
            day: 1,
            hour: Hours::H24(2),
            minute: 60
        },
        A2M::OncePerMinute
    );
}

mod alarm2_weekday {
    use super::*;
    set_alarm_test!(
        h24,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, 2, BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        am,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, 0b0100_0010, BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::AM(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        pm,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, 0b0110_0010, BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::PM(2),
            minute: 3
        },
        A2M::AllMatch
    );
    set_alarm_test!(
        match_hm,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::HoursAndMinutesMatch
    );
    set_alarm_test!(
        match_hm_ignore_incorrect_wd,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 0,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::HoursAndMinutesMatch
    );
    set_alarm_test!(
        match_m,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::MinutesMatch
    );
    set_alarm_test!(
        match_m_ignore_invalid_hour,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [3, AM, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(24),
            minute: 3
        },
        A2M::MinutesMatch
    );
    set_alarm_test!(
        match_opm,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [AM | 3, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 3
        },
        A2M::OncePerMinute
    );
    set_alarm_test!(
        match_opm_ignore_incorrect_min,
        set_alarm2_weekday,
        ALARM2_MINUTES,
        [AM, AM | 2, AM | BF::WEEKDAY | 1],
        WeekdayAlarm2 {
            weekday: 1,
            hour: Hours::H24(2),
            minute: 60
        },
        A2M::OncePerMinute
    );
}
