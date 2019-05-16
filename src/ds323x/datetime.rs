//! Common implementation

extern crate embedded_hal as hal;
use super::super::{BitFlags, Ds323x, Error, Register};
use super::{decimal_to_packed_bcd, hours_to_register, packed_bcd_to_decimal};
use interface::{ReadData, WriteData};

/// Date and time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    /// Year [2000-2099]
    pub year: u16,
    /// Month [1-12]
    pub month: u8,
    /// Day [1-31]
    pub day: u8,
    /// Weekday [1-7]
    pub weekday: u8,
    /// Hour in 24h/12h format
    pub hour: Hours,
    /// Minute [0-59]
    pub minute: u8,
    /// Second [0-59]
    pub second: u8,
}

/// Hours in either 12-hour (AM/PM) or 24-hour format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hours {
    /// AM [1-12]
    AM(u8),
    /// PM [1-12]
    PM(u8),
    /// 24H format [0-23]
    H24(u8),
}

impl<DI, IC, CommE, PinE> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Read the seconds.
    pub fn get_seconds(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.read_register_decimal(Register::SECONDS)
    }

    /// Read the minutes.
    pub fn get_minutes(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.read_register_decimal(Register::MINUTES)
    }

    /// Read the hours.
    pub fn get_hours(&mut self) -> Result<Hours, Error<CommE, PinE>> {
        let data = self.iface.read_register(Register::HOURS)?;
        Ok(hours_from_register(data))
    }

    /// Read the day of the week [1-7].
    pub fn get_weekday(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.read_register_decimal(Register::DOW)
    }

    /// Read the day of the month [1-31].
    pub fn get_day(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.read_register_decimal(Register::DOM)
    }

    /// Read the month [1-12].
    pub fn get_month(&mut self) -> Result<u8, Error<CommE, PinE>> {
        let data = self.iface.read_register(Register::MONTH)?;
        let value = data & !BitFlags::CENTURY;
        Ok(packed_bcd_to_decimal(value))
    }

    /// Read the year [2000-2100].
    pub fn get_year(&mut self) -> Result<u16, Error<CommE, PinE>> {
        let mut data = [0; 3];
        data[0] = Register::MONTH;
        self.iface.read_data(&mut data)?;
        Ok(year_from_registers(data[1], data[2]))
    }

    /// Read the date and time.
    pub fn get_datetime(&mut self) -> Result<DateTime, Error<CommE, PinE>> {
        let mut data = [0; 8];
        self.iface.read_data(&mut data)?;
        Ok(DateTime {
            year: year_from_registers(
                data[Register::MONTH as usize + 1],
                data[Register::YEAR as usize + 1],
            ),
            month: packed_bcd_to_decimal(data[Register::MONTH as usize + 1] & !BitFlags::CENTURY),
            day: packed_bcd_to_decimal(data[Register::DOM as usize + 1]),
            weekday: packed_bcd_to_decimal(data[Register::DOW as usize + 1]),
            hour: hours_from_register(data[Register::HOURS as usize + 1]),
            minute: packed_bcd_to_decimal(data[Register::MINUTES as usize + 1]),
            second: packed_bcd_to_decimal(data[Register::SECONDS as usize + 1]),
        })
    }

    fn read_register_decimal(&mut self, register: u8) -> Result<u8, Error<CommE, PinE>> {
        let data = self.iface.read_register(register)?;
        Ok(packed_bcd_to_decimal(data))
    }

    /// Set the seconds [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the seconds are out of range.
    pub fn set_seconds(&mut self, seconds: u8) -> Result<(), Error<CommE, PinE>> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::SECONDS, seconds)
    }

    /// Set the minutes [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the minutes are out of range.
    pub fn set_minutes(&mut self, minutes: u8) -> Result<(), Error<CommE, PinE>> {
        if minutes > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::MINUTES, minutes)
    }

    /// Set the hours.
    ///
    /// Changes the operating mode to 12h/24h depending on the parameter.
    ///
    /// Will return an `Error::InvalidInputData` if the hours are out of range.
    pub fn set_hours(&mut self, hours: Hours) -> Result<(), Error<CommE, PinE>> {
        let value = hours_to_register(hours)?;
        self.iface.write_register(Register::HOURS, value)
    }

    /// Set the day of week [1-7].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_weekday(&mut self, weekday: u8) -> Result<(), Error<CommE, PinE>> {
        if weekday < 1 || weekday > 7 {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(Register::DOW, weekday)
    }

    /// Set the day of month [1-31].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_day(&mut self, day: u8) -> Result<(), Error<CommE, PinE>> {
        if day < 1 || day > 7 {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(Register::DOM, day)
    }

    /// Set the month [1-12].
    ///
    /// Will return an `Error::InvalidInputData` if the month is out of range.
    pub fn set_month(&mut self, month: u8) -> Result<(), Error<CommE, PinE>> {
        if month < 1 || month > 12 {
            return Err(Error::InvalidInputData);
        }
        // keep the century bit
        let data = self.iface.read_register(Register::MONTH)?;
        let value = (data & BitFlags::CENTURY) | decimal_to_packed_bcd(month);
        self.iface.write_register(Register::MONTH, value)
    }

    /// Set the year [2000-2100].
    ///
    /// Will return an `Error::InvalidInputData` if the year is out of range.
    pub fn set_year(&mut self, year: u16) -> Result<(), Error<CommE, PinE>> {
        if year < 2000 || year > 2100 {
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

    /// Set the date and time.
    ///
    /// Will return an `Error::InvalidInputData` if any of the parameters is out of range.
    pub fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Error<CommE, PinE>> {
        if datetime.year < 2000
            || datetime.year > 2100
            || datetime.month < 1
            || datetime.month > 12
            || datetime.day < 1
            || datetime.day > 31
            || datetime.weekday < 1
            || datetime.weekday > 7
            || datetime.minute > 59
            || datetime.second > 59
        {
            return Err(Error::InvalidInputData);
        }
        let (month, year) = month_year_to_registers(datetime.month, datetime.year);
        let mut payload = [
            Register::SECONDS,
            decimal_to_packed_bcd(datetime.second),
            decimal_to_packed_bcd(datetime.minute),
            hours_to_register(datetime.hour)?,
            decimal_to_packed_bcd(datetime.weekday),
            decimal_to_packed_bcd(datetime.day),
            month,
            year,
        ];
        self.iface.write_data(&mut payload)
    }

    fn write_register_decimal(
        &mut self,
        register: u8,
        decimal_number: u8,
    ) -> Result<(), Error<CommE, PinE>> {
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
