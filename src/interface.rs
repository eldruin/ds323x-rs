//! I2C/SPI interfaces

#![deny(missing_docs)]

extern crate embedded_hal as hal;
use super::{Error, DEVICE_ADDRESS};
use hal::blocking;

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
}

/// Write data
pub trait WriteData {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>,
{
    type Error = Error<E, ()>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, CommE, PinE> WriteData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Write<u8, Error = CommE>,
    CS: hal::digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;

        let payload: [u8; 2] = [register + 0x80, data];
        let result = self.spi.write(&payload).map_err(Error::Comm);

        self.cs.set_high().map_err(Error::Pin)?;
        result
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        payload[0] += 0x80;
        let result = self.spi.write(&payload).map_err(Error::Comm);

        self.cs.set_high().map_err(Error::Pin)?;
        result
    }
}

/// Read data
pub trait ReadData {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>,
{
    type Error = Error<E, ()>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let len = payload.len();
        self.i2c
            .write_read(DEVICE_ADDRESS, &[payload[0]], &mut payload[1..len])
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, CommE, PinE> ReadData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Transfer<u8, Error = CommE>,
    CS: hal::digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        let mut data = [register, 0];
        let result = self.spi.transfer(&mut data).map_err(Error::Comm);
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(result?[1])
    }

    fn read_data(&mut self, mut payload: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        let result = self.spi.transfer(&mut payload).map_err(Error::Comm);
        self.cs.set_high().map_err(Error::Pin)?;
        result?;
        Ok(())
    }
}
