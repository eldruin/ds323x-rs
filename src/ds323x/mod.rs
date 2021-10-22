mod alarms;
mod configuration;
mod status;
pub use self::alarms::{
    Alarm1Matching, Alarm2Matching, DayAlarm1, DayAlarm2, WeekdayAlarm1, WeekdayAlarm2,
};
mod datetime;
use crate::{BitFlags, Error, Hours, NaiveTime, Timelike};

// Transforms a decimal number to packed BCD format
fn decimal_to_packed_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

// Transforms a number in packed BCD format to decimal
fn packed_bcd_to_decimal(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

fn hours_to_register<CommE, PinE>(hours: Hours) -> Result<u8, Error<CommE, PinE>> {
    match hours {
        Hours::H24(h) if h > 23 => Err(Error::InvalidInputData),
        Hours::H24(h) => Ok(decimal_to_packed_bcd(h)),
        Hours::AM(h) if !(1..=12).contains(&h) => Err(Error::InvalidInputData),
        Hours::AM(h) => Ok(BitFlags::H24_H12 | decimal_to_packed_bcd(h)),
        Hours::PM(h) if !(1..=12).contains(&h) => Err(Error::InvalidInputData),
        Hours::PM(h) => Ok(BitFlags::H24_H12 | BitFlags::AM_PM | decimal_to_packed_bcd(h)),
    }
}

fn some_or_invalid_error<T, CommE, PinE>(data: Option<T>) -> Result<T, Error<CommE, PinE>> {
    if let Some(data) = data {
        Ok(data)
    } else {
        Err(Error::InvalidDeviceState)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_some_then_get_inner() {
        match some_or_invalid_error::<u8, (), ()>(Some(1)) {
            Ok(1) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn if_none_then_error() {
        match some_or_invalid_error::<u8, (), ()>(None) {
            Err(Error::InvalidDeviceState) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn can_convert_packed_bcd_to_decimal() {
        assert_eq!(0, packed_bcd_to_decimal(0b0000_0000));
        assert_eq!(1, packed_bcd_to_decimal(0b0000_0001));
        assert_eq!(9, packed_bcd_to_decimal(0b0000_1001));
        assert_eq!(10, packed_bcd_to_decimal(0b0001_0000));
        assert_eq!(11, packed_bcd_to_decimal(0b0001_0001));
        assert_eq!(19, packed_bcd_to_decimal(0b0001_1001));
        assert_eq!(20, packed_bcd_to_decimal(0b0010_0000));
        assert_eq!(21, packed_bcd_to_decimal(0b0010_0001));
        assert_eq!(59, packed_bcd_to_decimal(0b0101_1001));
    }

    #[test]
    fn can_convert_decimal_to_packed_bcd() {
        assert_eq!(0b0000_0000, decimal_to_packed_bcd(0));
        assert_eq!(0b0000_0001, decimal_to_packed_bcd(1));
        assert_eq!(0b0000_1001, decimal_to_packed_bcd(9));
        assert_eq!(0b0001_0000, decimal_to_packed_bcd(10));
        assert_eq!(0b0001_0001, decimal_to_packed_bcd(11));
        assert_eq!(0b0001_1001, decimal_to_packed_bcd(19));
        assert_eq!(0b0010_0000, decimal_to_packed_bcd(20));
        assert_eq!(0b0010_0001, decimal_to_packed_bcd(21));
        assert_eq!(0b0101_1001, decimal_to_packed_bcd(59));
    }
}
