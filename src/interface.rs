//! I2C/SPI interfaces

use crate::{private, Error, DEVICE_ADDRESS};
use embedded_hal::{i2c, spi};

/// I2C interface
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// SPI interface
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write(DEVICE_ADDRESS, payload).map_err(Error::Comm)
    }
}

impl<SPI, E> WriteData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register + 0x80, data];
        self.spi.write(&payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;
        self.spi.write(payload).map_err(Error::Comm)
    }
}

/// Read data
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
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

impl<SPI, E> ReadData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [register, 0];
        let result = self.spi.transfer_in_place(&mut data).map_err(Error::Comm);
        result.and(Ok(data[1]))
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.transfer_in_place(payload).map_err(Error::Comm)
    }
}
