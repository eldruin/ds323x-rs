//! Common implementation

extern crate embedded_hal as hal;
use super::super::{ Ds323x, Register, BitFlags, Error };
use interface::{ ReadData, WriteData };

/// Hours in either 12-hour (AM/PM) or 24-hour format
#[derive(Debug, Clone, PartialEq)]
pub enum Hours {
    /// AM [1-12]
    AM(u8),
    /// PM [1-12]
    PM(u8),
    /// 24H format [0-23]
    H24(u8),
}

impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = E> + WriteData<Error = E>
{
    /// Read the seconds.
    pub fn get_seconds(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::SECONDS)
    }

    /// Read the minutes.
    pub fn get_minutes(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::MINUTES)
    }

    /// Read the hours.
    pub fn get_hours(&mut self) -> Result<Hours, Error<E>> {
        let data = self.iface.read_register(Register::HOURS)?;
        self.get_hours_from_register(data)
    }

    fn get_hours_from_register(&self, data: u8) -> Result<Hours, Error<E>> {
        if is_24h_format(data) {
            Ok(Hours::H24(packed_bcd_to_decimal(data & !BitFlags::H24_H12)))
        }
        else if is_am(data) {
            Ok(Hours::AM(packed_bcd_to_decimal(data & !(BitFlags::H24_H12 | BitFlags::AM_PM))))
        }
        else {
            Ok(Hours::PM(packed_bcd_to_decimal(data & !(BitFlags::H24_H12 | BitFlags::AM_PM))))
        }
    }

    /// Read the day of the week [1-7].
    pub fn get_weekday(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::DOW)
    }

    /// Read the day of the month [1-31].
    pub fn get_day(&mut self) -> Result<u8, Error<E>> {
        self.read_register_decimal(Register::DOM)
    }

    /// Read the month [1-12].
    pub fn get_month(&mut self) -> Result<u8, Error<E>> {
        let data = self.iface.read_register(Register::MONTH)?;
        let value = data & !BitFlags::CENTURY;
        Ok(packed_bcd_to_decimal(value))
    }

    /// Read the year [2000-2100].
    pub fn get_year(&mut self) -> Result<u16, Error<E>> {
        let mut data = [0; 3];
        data[0] = Register::MONTH;
        self.iface.read_data(&mut data)?;
        let century = data[1] & BitFlags::CENTURY;
        let year = packed_bcd_to_decimal(data[2]);
        if century != 0 {
            Ok(2100 + (year as u16))
        }
        else {
            Ok(2000 + (year as u16))
        }
    }

    fn read_register_decimal(&mut self, register: u8) -> Result<u8, Error<E>> {
        let data = self.iface.read_register(register)?;
        Ok(packed_bcd_to_decimal(data))
    }

    /// Set the seconds [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the seconds are out of range.
    pub fn set_seconds(&mut self, seconds: u8) -> Result<(), Error<E>> {
        if seconds > 59 {
            return Err(Error::InvalidInputData);
        }
        self.write_register_decimal(Register::SECONDS, seconds)
    }

    /// Set the minutes [0-59].
    ///
    /// Will return an `Error::InvalidInputData` if the minutes are out of range.
    pub fn set_minutes(&mut self, minutes: u8) -> Result<(), Error<E>> {
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
    pub fn set_hours(&mut self, hours: Hours) -> Result<(), Error<E>> {
        let value = self.get_hours_register_value(&hours)?;
        self.iface.write_register(Register::HOURS, value)
    }

    fn get_hours_register_value(&mut self, hours: &Hours) -> Result<u8, Error<E>> {
        match *hours {
            Hours::H24(h) if h > 23 => Err(Error::InvalidInputData),
            Hours::H24(h) => Ok(decimal_to_packed_bcd(h)),
            Hours::AM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
            Hours::AM(h) =>  Ok(BitFlags::H24_H12 | decimal_to_packed_bcd(h)),
            Hours::PM(h) if h < 1 || h > 12 => Err(Error::InvalidInputData),
            Hours::PM(h) =>  Ok(BitFlags::H24_H12 | BitFlags::AM_PM | decimal_to_packed_bcd(h)),
        }
    }

    /// Set the day of week [1-7].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_weekday(&mut self, weekday: u8) -> Result<(), Error<E>> {
        if weekday < 1 || weekday > 7 {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(Register::DOW, weekday)
    }

    /// Set the day of month [1-31].
    ///
    /// Will return an `Error::InvalidInputData` if the day is out of range.
    pub fn set_day(&mut self, day: u8) -> Result<(), Error<E>> {
        if day < 1 || day > 7 {
            return Err(Error::InvalidInputData);
        }
        self.iface.write_register(Register::DOM, day)
    }

    /// Set the month [1-12].
    ///
    /// Will return an `Error::InvalidInputData` if the month is out of range.
    pub fn set_month(&mut self, month: u8) -> Result<(), Error<E>> {
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
    pub fn set_year(&mut self, year: u16) -> Result<(), Error<E>> {
        if year < 2000 || year > 2100 {
            return Err(Error::InvalidInputData);
        }
        let data = self.iface.read_register(Register::MONTH)?;
        let month_bcd = data & !BitFlags::CENTURY;
        if year > 2099 {
            let mut data = [ Register::MONTH,
                             BitFlags::CENTURY | month_bcd,
                             decimal_to_packed_bcd((year - 2100) as u8) ];
            self.iface.write_data(&mut data)
        }
        else {
            let mut data = [ Register::MONTH, month_bcd,
                             decimal_to_packed_bcd((year - 2000) as u8) ];
            self.iface.write_data(&mut data)
        }
    }

    fn write_register_decimal(&mut self, register: u8, decimal_number: u8) -> Result<(), Error<E>> {
        self.iface.write_register(register, decimal_to_packed_bcd(decimal_number))
    }
}

fn is_24h_format(hours_data: u8) -> bool {
    hours_data & BitFlags::H24_H12 == 0
}

fn is_am(hours_data: u8) -> bool {
    hours_data & BitFlags::AM_PM == 0
}

// Transforms a decimal number to packed BCD format
fn decimal_to_packed_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

// Transforms a number in packed BCD format to decimal
fn packed_bcd_to_decimal(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_packed_bcd_to_decimal() {
        assert_eq!(0,  packed_bcd_to_decimal(0b0000_0000));
        assert_eq!(1,  packed_bcd_to_decimal(0b0000_0001));
        assert_eq!(9,  packed_bcd_to_decimal(0b0000_1001));
        assert_eq!(10, packed_bcd_to_decimal(0b0001_0000));
        assert_eq!(11, packed_bcd_to_decimal(0b0001_0001));
        assert_eq!(19, packed_bcd_to_decimal(0b0001_1001));
        assert_eq!(20, packed_bcd_to_decimal(0b0010_0000));
        assert_eq!(21, packed_bcd_to_decimal(0b0010_0001));
        assert_eq!(59, packed_bcd_to_decimal(0b0101_1001));
    }

    #[test]
    fn can_convert_decimal_to_packed_bcd() {
        assert_eq!(0b0000_0000, decimal_to_packed_bcd( 0));
        assert_eq!(0b0000_0001, decimal_to_packed_bcd( 1));
        assert_eq!(0b0000_1001, decimal_to_packed_bcd( 9));
        assert_eq!(0b0001_0000, decimal_to_packed_bcd(10));
        assert_eq!(0b0001_0001, decimal_to_packed_bcd(11));
        assert_eq!(0b0001_1001, decimal_to_packed_bcd(19));
        assert_eq!(0b0010_0000, decimal_to_packed_bcd(20));
        assert_eq!(0b0010_0001, decimal_to_packed_bcd(21));
        assert_eq!(0b0101_1001, decimal_to_packed_bcd(59));
    }
}