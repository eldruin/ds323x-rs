//! Alarm support

use super::super::{BitFlags, Ds323x, Error, Hours, Register};
use super::{decimal_to_packed_bcd, hours_to_register};
use crate::ds323x::{NaiveTime, Timelike};
use interface::{ReadData, WriteData};

/// Parameters for setting Alarm1 on a day of the month
///
/// Depending on the matching strategy, some fields may not be relevant. In this
/// case, invalid values are ignored and the minimum valid values are used instead to
/// configure the alarm:
/// - Second, minute and hour: 0
/// - Day: 1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DayAlarm1 {
    /// Day of the month [1-31]
    pub day: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

/// Parameters for setting Alarm1 on a weekday
///
/// Depending on the matching strategy, some fields may not be relevant. In this
/// case, invalid values are ignored and the minimum valid values are used instead to
/// configure the alarm:
/// - Second, minute and hour: 0
/// - Weekday: 1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeekdayAlarm1 {
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

/// Alarm1 trigger rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alarm1Matching {
    /// Alarm once per second.
    OncePerSecond,
    /// Alarm when seconds match.
    SecondsMatch,
    /// Alarm when minutes and seconds match.
    MinutesAndSecondsMatch,
    /// Alarm when hours, minutes and seconds match.
    HoursMinutesAndSecondsMatch,
    /// Alarm when date/weekday, hours, minutes and seconds match.
    AllMatch,
}

/// Parameters for setting Alarm2 on a day of the month
///
/// Depending on the matching strategy, some fields may not be relevant. In this
/// case, invalid values are ignored and the minimum valid values are used instead to
/// configure the alarm:
/// - Minute and hour: 0
/// - Day: 1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DayAlarm2 {
    /// Day of month [1-31]
    pub day: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
}

/// Parameters for setting Alarm2 on a weekday
///
/// Depending on the matching strategy, some fields may not be relevant. In this
/// case, invalid values are ignored and the minimum valid values are used instead to
/// configure the alarm:
/// - Minute and hour: 0
/// - Weekday: 1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeekdayAlarm2 {
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
}

/// Alarm2 trigger rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alarm2Matching {
    /// Alarm once per minute. (00 seconds of every minute)
    OncePerMinute,
    /// Alarm when minutes match.
    MinutesMatch,
    /// Alarm when hours and minutes match.
    HoursAndMinutesMatch,
    /// Alarm when date/weekday, hours and minutes match.
    AllMatch,
}

fn get_matching_mask_alarm1(matching: Alarm1Matching) -> [u8; 4] {
    const AM: u8 = BitFlags::ALARM_MATCH;
    match matching {
        Alarm1Matching::OncePerSecond => [AM, AM, AM, AM],
        Alarm1Matching::SecondsMatch => [0, AM, AM, AM],
        Alarm1Matching::MinutesAndSecondsMatch => [0, 0, AM, AM],
        Alarm1Matching::HoursMinutesAndSecondsMatch => [0, 0, 0, AM],
        Alarm1Matching::AllMatch => [0, 0, 0, 0],
    }
}

fn get_matching_mask_alarm2(matching: Alarm2Matching) -> [u8; 3] {
    const AM: u8 = BitFlags::ALARM_MATCH;
    match matching {
        Alarm2Matching::OncePerMinute => [AM, AM, AM],
        Alarm2Matching::MinutesMatch => [0, AM, AM],
        Alarm2Matching::HoursAndMinutesMatch => [0, 0, AM],
        Alarm2Matching::AllMatch => [0, 0, 0],
    }
}

/// Test if hour value is valid
fn is_hour_valid(hours: Hours) -> bool {
    match hours {
        Hours::H24(h) if h > 23 => true,
        Hours::AM(h) if h < 1 || h > 12 => true,
        Hours::PM(h) if h < 1 || h > 12 => true,
        _ => false,
    }
}

/// Amend invalid hour values
fn amend_hour(hours: Hours) -> Hours {
    match hours {
        Hours::H24(h) if h > 23 => Hours::H24(0),
        Hours::H24(h) => Hours::H24(h),
        Hours::AM(h) if h < 1 || h > 12 => Hours::AM(1),
        Hours::AM(h) => Hours::AM(h),
        Hours::PM(h) if h < 1 || h > 12 => Hours::PM(1),
        Hours::PM(h) => Hours::PM(h),
    }
}

impl<DI, IC, CommE, PinE> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Set Alarm1 for day of the month.
    ///
    /// Will return an `Error::InvalidInputData` if any of the used parameters
    /// (depending on the matching startegy) is out of range. Any unused
    /// parameter is set to the corresponding minimum valid value:
    /// - Second, minute, hour: 0
    /// - Day: 1
    pub fn set_alarm1_day(
        &mut self,
        when: DayAlarm1,
        matching: Alarm1Matching,
    ) -> Result<(), Error<CommE, PinE>> {
        let day_invalid = when.day < 1 || when.day > 31;
        let hour_invalid = is_hour_valid(when.hour);
        let minute_invalid = when.minute > 59;
        let second_invalid = when.second > 59;

        let day = if day_invalid { 1 } else { when.day };
        let hour = amend_hour(when.hour);
        let minute = if minute_invalid { 0 } else { when.minute };

        if (matching == Alarm1Matching::AllMatch && (day_invalid || hour_invalid))
            || (hour_invalid && matching == Alarm1Matching::HoursMinutesAndSecondsMatch)
            || ((matching != Alarm1Matching::SecondsMatch
                && matching != Alarm1Matching::OncePerSecond)
                && minute_invalid)
            || second_invalid
        {
            return Err(Error::InvalidInputData);
        }

        let match_mask = get_matching_mask_alarm1(matching);
        let mut data = [
            Register::ALARM1_SECONDS,
            decimal_to_packed_bcd(when.second) | match_mask[0],
            decimal_to_packed_bcd(minute) | match_mask[1],
            hours_to_register(hour)? | match_mask[2],
            decimal_to_packed_bcd(day) | match_mask[3],
        ];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm1 for a time (fires when hours, minutes and seconds match).
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    /// The day is not used by this matching strategy and is set to 1.
    pub fn set_alarm1_hms(&mut self, when: NaiveTime) -> Result<(), Error<CommE, PinE>> {
        let alarm = DayAlarm1 {
            day: 1,
            hour: Hours::H24(when.hour() as u8),
            minute: when.minute() as u8,
            second: when.second() as u8,
        };
        self.set_alarm1_day(alarm, Alarm1Matching::HoursMinutesAndSecondsMatch)
    }

    /// Set Alarm1 for weekday.
    ///
    /// Will return an `Error::InvalidInputData` if any of the used parameters
    /// (depending on the matching startegy) is out of range. Any unused
    /// parameter is set to the corresponding minimum valid value:
    /// - Second, minute, hour: 0
    /// - Weekday: 1
    pub fn set_alarm1_weekday(
        &mut self,
        when: WeekdayAlarm1,
        matching: Alarm1Matching,
    ) -> Result<(), Error<CommE, PinE>> {
        let weekday_invalid = when.weekday < 1 || when.weekday > 7;
        let hour_invalid = is_hour_valid(when.hour);
        let minute_invalid = when.minute > 59;
        let second_invalid = when.second > 59;

        let weekday = if weekday_invalid { 1 } else { when.weekday };
        let hour = amend_hour(when.hour);
        let minute = if minute_invalid { 0 } else { when.minute };
        let second = if second_invalid { 0 } else { when.second };

        if ((hour_invalid || weekday_invalid) && matching == Alarm1Matching::AllMatch)
            || (hour_invalid && matching == Alarm1Matching::HoursMinutesAndSecondsMatch)
            || (minute_invalid
                && (matching != Alarm1Matching::OncePerSecond
                    && matching != Alarm1Matching::SecondsMatch))
            || (second_invalid && matching != Alarm1Matching::OncePerSecond)
        {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm1(matching);
        let mut data = [
            Register::ALARM1_SECONDS,
            decimal_to_packed_bcd(second) | match_mask[0],
            decimal_to_packed_bcd(minute) | match_mask[1],
            hours_to_register(hour)? | match_mask[2],
            decimal_to_packed_bcd(weekday) | match_mask[3] | BitFlags::WEEKDAY,
        ];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm2 for date (day of month).
    ///
    /// Will return an `Error::InvalidInputData` if any of the used parameters
    /// (depending on the matching startegy) is out of range. Any unused
    /// parameter is set to the corresponding minimum valid value:
    /// - Minute, hour: 0
    /// - Day: 1
    pub fn set_alarm2_day(
        &mut self,
        when: DayAlarm2,
        matching: Alarm2Matching,
    ) -> Result<(), Error<CommE, PinE>> {
        let day_invalid = when.day < 1 || when.day > 31;
        let hour_invalid = is_hour_valid(when.hour);
        let minute_invalid = when.minute > 59;

        let day = if day_invalid { 1 } else { when.day };
        let hour = amend_hour(when.hour);
        let minute = if minute_invalid { 0 } else { when.minute };

        if ((day_invalid || hour_invalid) && matching == Alarm2Matching::AllMatch)
            || (hour_invalid && matching == Alarm2Matching::HoursAndMinutesMatch)
            || (matching != Alarm2Matching::OncePerMinute && minute_invalid)
        {
            return Err(Error::InvalidInputData);
        }

        let match_mask = get_matching_mask_alarm2(matching);
        let mut data = [
            Register::ALARM2_MINUTES,
            decimal_to_packed_bcd(minute) | match_mask[0],
            hours_to_register(hour)? | match_mask[1],
            decimal_to_packed_bcd(day) | match_mask[2],
        ];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm2 for a time (fires when hours and minutes match).
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    /// The day is not used by this matching strategy and is set to 1.
    pub fn set_alarm2_hm(&mut self, when: NaiveTime) -> Result<(), Error<CommE, PinE>> {
        let alarm = DayAlarm2 {
            day: 1,
            hour: Hours::H24(when.hour() as u8),
            minute: when.minute() as u8,
        };
        self.set_alarm2_day(alarm, Alarm2Matching::HoursAndMinutesMatch)
    }

    /// Set Alarm2 for weekday.
    ///
    /// Will return an `Error::InvalidInputData` if any of the used parameters
    /// (depending on the matching startegy) is out of range. Any unused
    /// parameter is set to the corresponding minimum valid value:
    /// - Minute, hour: 0
    /// - Weekday: 1
    pub fn set_alarm2_weekday(
        &mut self,
        when: WeekdayAlarm2,
        matching: Alarm2Matching,
    ) -> Result<(), Error<CommE, PinE>> {
        let weekday_invalid = when.weekday < 1 || when.weekday > 7;
        let hour_invalid = is_hour_valid(when.hour);
        let minute_invalid = when.minute > 59;

        let weekday = if weekday_invalid { 1 } else { when.weekday };
        let hour = amend_hour(when.hour);
        let minute = if minute_invalid { 0 } else { when.minute };

        if (matching == Alarm2Matching::AllMatch && (weekday_invalid || hour_invalid))
            || (matching == Alarm2Matching::HoursAndMinutesMatch && hour_invalid)
            || (minute_invalid && matching != Alarm2Matching::OncePerMinute)
        {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm2(matching);
        let mut data = [
            Register::ALARM2_MINUTES,
            decimal_to_packed_bcd(minute) | match_mask[0],
            hours_to_register(hour)? | match_mask[1],
            decimal_to_packed_bcd(weekday) | match_mask[2] | BitFlags::WEEKDAY,
        ];
        self.iface.write_data(&mut data)
    }
}
