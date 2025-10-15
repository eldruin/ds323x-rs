//! Common implementation

use super::{decimal_to_packed_bcd, hours_to_register, packed_bcd_to_decimal};
use crate::{
    interface::{ReadData, WriteData},
    BitFlags, DateTimeAccess, Datelike, Ds323x, Error, Hours, NaiveDate, NaiveDateTime, NaiveTime,
    Register, Rtcc, Timelike,
};

impl<DI, IC, E> DateTimeAccess for Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    type Error = Error<E>;

    fn datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        let mut data = [0; 8];
        self.iface.read_data(&mut data)?;

        let year = year_from_registers(
            data[Register::MONTH as usize + 1],
            data[Register::YEAR as usize + 1],
        );
        let month = packed_bcd_to_decimal(data[Register::MONTH as usize + 1] & !BitFlags::CENTURY);
        let day = packed_bcd_to_decimal(data[Register::DOM as usize + 1]);
        let hour = hours_from_register(data[Register::HOURS as usize + 1]);
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize + 1]);
        let second = packed_bcd_to_decimal(data[Register::SECONDS as usize + 1]);

        let date = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .ok_or(Error::InvalidDeviceState)?;

        date.and_hms_opt(get_h24(hour).into(), minute.into(), second.into())
            .ok_or(Error::InvalidDeviceState)
    }

    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error> {
        if datetime.year() < 2000 || datetime.year() > 2100 {
            return Err(Error::InvalidInputData);
        }
        let (month, year) = month_year_to_registers(datetime.month() as u8, datetime.year() as u16);
        let mut payload = [
            Register::SECONDS,
            decimal_to_packed_bcd(datetime.second() as u8),
            decimal_to_packed_bcd(datetime.minute() as u8),
            hours_to_register(Hours::H24(datetime.hour() as u8))?,
            datetime.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(datetime.day() as u8),
            month,
            year,
        ];
        self.iface.write_data(&mut payload)
    }
}

impl<DI, IC, E> Rtcc for Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    fn seconds(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::SECONDS)
    }

    fn minutes(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::MINUTES)
    }

    fn hours(&mut self) -> Result<Hours, Self::Error> {
        let data = self.iface.read_register(Register::HOURS)?;
        Ok(hours_from_register(data))
    }

    fn time(&mut self) -> Result<NaiveTime, Self::Error> {
        let mut data = [0; 4];
        self.iface.read_data(&mut data)?;
        let hour = hours_from_register(data[Register::HOURS as usize + 1]);
        let minute = packed_bcd_to_decimal(data[Register::MINUTES as usize + 1]);
        let second = packed_bcd_to_decimal(data[Register::SECONDS as usize + 1]);

        NaiveTime::from_hms_opt(get_h24(hour).into(), minute.into(), second.into())
            .ok_or(Error::InvalidDeviceState)
    }

    fn weekday(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::DOW)
    }

    fn day(&mut self) -> Result<u8, Self::Error> {
        self.read_register_decimal(Register::DOM)
    }

    fn month(&mut self) -> Result<u8, Self::Error> {
        let data = self.iface.read_register(Register::MONTH)?;
        let value = data & !BitFlags::CENTURY;
        Ok(packed_bcd_to_decimal(value))
    }

    fn year(&mut self) -> Result<u16, Self::Error> {
        let mut data = [0; 3];
        data[0] = Register::MONTH;
        self.iface.read_data(&mut data)?;
        Ok(year_from_registers(data[1], data[2]))
    }

    fn date(&mut self) -> Result<NaiveDate, Self::Error> {
        let mut data = [0; 4];
        data[0] = Register::DOM;
        self.iface.read_data(&mut data)?;

        let offset = Register::DOM as usize;
        let year = year_from_registers(
            data[Register::MONTH as usize + 1 - offset],
            data[Register::YEAR as usize + 1 - offset],
        );
        let month =
            packed_bcd_to_decimal(data[Register::MONTH as usize + 1 - offset] & !BitFlags::CENTURY);
        let day = packed_bcd_to_decimal(data[Register::DOM as usize + 1 - offset]);

        NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .ok_or(Error::InvalidDeviceState)
    }

    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::SECONDS, seconds)
    }

    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MINUTES, minutes)
    }

    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error> {
        let value = hours_to_register(hours)?;
        self.iface.write_register(Register::HOURS, value)
    }

    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error> {
        let mut payload = [
            Register::SECONDS,
            decimal_to_packed_bcd(time.second() as u8),
            decimal_to_packed_bcd(time.minute() as u8),
            hours_to_register(Hours::H24(time.hour() as u8))?,
        ];
        self.iface.write_data(&mut payload)
    }

    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error> {
        if !(1..=7).contains(&weekday) {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(Register::DOW, weekday)
    }

    fn set_day(&mut self, day: u8) -> Result<(), Self::Error> {
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::DOM, day)
    }

    fn set_month(&mut self, month: u8) -> Result<(), Self::Error> {
        if !(1..=12).contains(&month) {
            return Err(Error::InvalidInputData);
        }
        // keep the century bit
        let data = self.iface.read_register(Register::MONTH)?;
        let value = (data & BitFlags::CENTURY) | decimal_to_packed_bcd(month);
        self.iface.write_register(Register::MONTH, value)
    }

    fn set_year(&mut self, year: u16) -> Result<(), Self::Error> {
        if !(2000..=2100).contains(&year) {
            return Err(Error::InvalidInputData);
        }
        let data = self.iface.read_register(Register::MONTH)?;
        let month_bcd = data & !BitFlags::CENTURY;
        if year > 2099 {
            let mut data = [
                Register::MONTH,
                BitFlags::CENTURY | month_bcd,
                decimal_to_packed_bcd((year - 2100) as u8),
            ];
            self.iface.write_data(&mut data)
        } else {
            let mut data = [
                Register::MONTH,
                month_bcd,
                decimal_to_packed_bcd((year - 2000) as u8),
            ];
            self.iface.write_data(&mut data)
        }
    }

    fn set_date(&mut self, date: &rtcc::NaiveDate) -> Result<(), Self::Error> {
        if date.year() < 2000 || date.year() > 2100 {
            return Err(Error::InvalidInputData);
        }
        let (month, year) = month_year_to_registers(date.month() as u8, date.year() as u16);
        let mut payload = [
            Register::DOW,
            date.weekday().number_from_sunday() as u8,
            decimal_to_packed_bcd(date.day() as u8),
            month,
            year,
        ];
        self.iface.write_data(&mut payload)
    }
}

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    fn read_register_decimal(&mut self, register: u8) -> Result<u8, Error<E>> {
        let data = self.iface.read_register(register)?;
        Ok(packed_bcd_to_decimal(data))
    }

    fn write_register_decimal(&mut self, register: u8, decimal_number: u8) -> Result<(), Error<E>> {
        self.iface
            .write_register(register, decimal_to_packed_bcd(decimal_number))
    }
}

fn hours_from_register(data: u8) -> Hours {
    if is_24h_format(data) {
        Hours::H24(packed_bcd_to_decimal(data & !BitFlags::H24_H12))
    } else if is_am(data) {
        Hours::AM(packed_bcd_to_decimal(
            data & !(BitFlags::H24_H12 | BitFlags::AM_PM),
        ))
    } else {
        Hours::PM(packed_bcd_to_decimal(
            data & !(BitFlags::H24_H12 | BitFlags::AM_PM),
        ))
    }
}

fn year_from_registers(month: u8, year: u8) -> u16 {
    let century = month & BitFlags::CENTURY;
    let year = packed_bcd_to_decimal(year);
    if century != 0 {
        2100 + u16::from(year)
    } else {
        2000 + u16::from(year)
    }
}

fn month_year_to_registers(month: u8, year: u16) -> (u8, u8) {
    if year > 2099 {
        let month = BitFlags::CENTURY | decimal_to_packed_bcd(month);
        (month, decimal_to_packed_bcd((year - 2100) as u8))
    } else {
        (
            decimal_to_packed_bcd(month),
            decimal_to_packed_bcd((year - 2000) as u8),
        )
    }
}

fn is_24h_format(hours_data: u8) -> bool {
    hours_data & BitFlags::H24_H12 == 0
}

fn is_am(hours_data: u8) -> bool {
    hours_data & BitFlags::AM_PM == 0
}

fn get_h24(hour: Hours) -> u8 {
    match hour {
        Hours::H24(h) => h,
        Hours::AM(h) => h,
        Hours::PM(h) => h + 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_to_h24() {
        assert_eq!(0, get_h24(Hours::H24(0)));
        assert_eq!(0, get_h24(Hours::AM(0)));
        assert_eq!(12, get_h24(Hours::PM(0)));

        assert_eq!(1, get_h24(Hours::H24(1)));
        assert_eq!(1, get_h24(Hours::AM(1)));
        assert_eq!(13, get_h24(Hours::PM(1)));

        assert_eq!(23, get_h24(Hours::H24(23)));
        assert_eq!(12, get_h24(Hours::AM(12)));
        assert_eq!(23, get_h24(Hours::PM(11)));
    }
}
