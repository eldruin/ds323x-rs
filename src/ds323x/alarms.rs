//! Alarm support

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Hours, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };
use super::{ decimal_to_packed_bcd, hours_to_register };

/// Parameters for setting Alarm1 on a date
#[derive(Debug, Clone, PartialEq)]
pub struct DateAlarm1 {
    /// Date (day of month) [1-31]
    pub date: u8,
    /// Hour
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8
}

/// Alarm1 trigger rate
#[derive(Debug, Clone, PartialEq)]
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


impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Set Alarm1 for date (day of month).
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_alarm1_date(&mut self, when: DateAlarm1, matching: Alarm1Matching) -> Result<(), Error<E>> {
        if when.date < 1    || when.date > 31 ||
           when.minute > 59 ||
           when.second > 59 {
            return Err(Error::InvalidInputData);
        }
        let match_mask = get_matching_mask_alarm1(matching);
        let mut data = [ Register::ALARM1_SECONDS,
                         decimal_to_packed_bcd(when.second) | match_mask[0],
                         decimal_to_packed_bcd(when.minute) | match_mask[1],
                         hours_to_register(&when.hour)?     | match_mask[2],
                         decimal_to_packed_bcd(when.date)   | match_mask[3]];
        self.iface.write_data(&mut data)
    }

}
