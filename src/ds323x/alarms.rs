//! Alarm support

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Hours, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };
use super::{ decimal_to_packed_bcd, hours_to_register };

/// Parameters for setting Alarm1 on a day of the month
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DayAlarm1 {
    /// Day of the month [1-31]
    pub day: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8
}

/// Parameters for setting Alarm1 on a weekday
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeekdayAlarm1 {
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8
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
    AllMatch
}


/// Parameters for setting Alarm2 on a day of the month
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DayAlarm2 {
    /// Day of month [1-31]
    pub day: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8
}

/// Parameters for setting Alarm2 on a weekday
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeekdayAlarm2 {
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8
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
    AllMatch
}

fn get_matching_mask_alarm1(matching: Alarm1Matching) -> [u8; 4] {
    const AM : u8 = BitFlags::ALARM_MATCH;
    match matching {
        Alarm1Matching::OncePerSecond               => [AM, AM, AM, AM],
        Alarm1Matching::SecondsMatch                => [ 0, AM, AM, AM],
        Alarm1Matching::MinutesAndSecondsMatch      => [ 0,  0, AM, AM],
        Alarm1Matching::HoursMinutesAndSecondsMatch => [ 0,  0,  0, AM],
        Alarm1Matching::AllMatch                    => [ 0,  0,  0,  0],
    }
}

fn get_matching_mask_alarm2(matching: Alarm2Matching) -> [u8; 3] {
    const AM : u8 = BitFlags::ALARM_MATCH;
    match matching {
        Alarm2Matching::OncePerMinute        => [AM, AM, AM],
        Alarm2Matching::MinutesMatch         => [ 0, AM, AM],
        Alarm2Matching::HoursAndMinutesMatch => [ 0,  0, AM],
        Alarm2Matching::AllMatch             => [ 0,  0,  0],
    }
}


impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Set Alarm1 for day of the month.
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_alarm1_day(&mut self, when: DayAlarm1, matching: Alarm1Matching) -> Result<(), Error<E>> {
        if when.day < 1    || when.day > 31 ||
           when.minute > 59 ||
           when.second > 59 {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm1(matching);
        let mut data = [ Register::ALARM1_SECONDS,
                         decimal_to_packed_bcd(when.second) | match_mask[0],
                         decimal_to_packed_bcd(when.minute) | match_mask[1],
                         hours_to_register(when.hour)?      | match_mask[2],
                         decimal_to_packed_bcd(when.day)    | match_mask[3]];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm1 for weekday.
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_alarm1_weekday(&mut self, when: WeekdayAlarm1, matching: Alarm1Matching) -> Result<(), Error<E>> {
        if when.weekday < 1    || when.weekday > 7 ||
           when.minute > 59 ||
           when.second > 59 {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm1(matching);
        let mut data = [ Register::ALARM1_SECONDS,
                         decimal_to_packed_bcd(when.second)  | match_mask[0],
                         decimal_to_packed_bcd(when.minute)  | match_mask[1],
                         hours_to_register(when.hour)?       | match_mask[2],
                         decimal_to_packed_bcd(when.weekday) | match_mask[3] | BitFlags::WEEKDAY];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm2 for date (day of month).
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_alarm2_day(&mut self, when: DayAlarm2, matching: Alarm2Matching) -> Result<(), Error<E>> {
        if when.day < 1    || when.day > 31 ||
           when.minute > 59 {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm2(matching);
        let mut data = [ Register::ALARM2_MINUTES,
                         decimal_to_packed_bcd(when.minute) | match_mask[0],
                         hours_to_register(when.hour)?      | match_mask[1],
                         decimal_to_packed_bcd(when.day)    | match_mask[2]];
        self.iface.write_data(&mut data)
    }

    /// Set Alarm2 for weekday.
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_alarm2_weekday(&mut self, when: WeekdayAlarm2, matching: Alarm2Matching) -> Result<(), Error<E>> {
        if when.weekday < 1    || when.weekday > 7 ||
           when.minute > 59 {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm2(matching);
        let mut data = [ Register::ALARM2_MINUTES,
                         decimal_to_packed_bcd(when.minute)  | match_mask[0],
                         hours_to_register(when.hour)?       | match_mask[1],
                         decimal_to_packed_bcd(when.weekday) | match_mask[2] | BitFlags::WEEKDAY];
        self.iface.write_data(&mut data)
    }
}
