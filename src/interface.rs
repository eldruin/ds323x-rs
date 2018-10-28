//! I2C/SPI interfaces

#![deny(missing_docs)]

extern crate embedded_hal as hal;
use hal::blocking;
use super::{ DEVICE_ADDRESS, Error };

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS
}

/// Write data
pub trait WriteData {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<Self::Error>>;
    /// Write to two consecutive u8 registers
    fn write_two_registers(&mut self, first_register: u8, data: &[u8; 2]) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>
{
    type Error = E;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }

    fn write_two_registers(&mut self, first_register: u8, data: &[u8; 2]) -> Result<(), Error<Self::Error>> {
        let payload: [u8; 3] = [first_register, data[0], data[1]];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, E> WriteData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Write<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    type Error = E;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        self.cs.set_low();

        let payload: [u8; 2] = [register + 0x80, data];
        let result = self.spi
                         .write(&payload)
                         .map_err(Error::Comm);

        self.cs.set_high();
        result
    }

    fn write_two_registers(&mut self, first_register: u8, data: &[u8; 2]) -> Result<(), Error<E>> {
        self.cs.set_low();

        let payload: [u8; 3] = [first_register + 0x80, data[0], data[1]];
        let result = self.spi
                         .write(&payload)
                         .map_err(Error::Comm);

        self.cs.set_high();
        result
    }
}


/// Read data
pub trait ReadData {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Error<Self::Error>>;
    /// Read two u8 registers
    fn read_two_registers(&mut self, register: u8, data: &mut [u8; 2]) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_two_registers(&mut self, register: u8, data: &mut [u8; 2]) -> Result<(), Error<Self::Error>> {
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data[..])
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, E> ReadData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Transfer<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        self.cs.set_low();
        let mut data = [register, 0];
        let result = self.spi
                         .transfer(&mut data)
                         .map_err(Error::Comm);
        self.cs.set_high();
        match result {
            Ok(result) => Ok(result[1]),
            Err(e) => Err(e)
        }
    }

    fn read_two_registers(&mut self, register: u8, data: &mut [u8; 2]) -> Result<(), Error<Self::Error>> {
        self.cs.set_low();
        let mut payload = [register, 0, 0];
        let result = self.spi
                         .transfer(&mut payload)
                         .map_err(Error::Comm);
        self.cs.set_high();
        match result {
            Ok(result) => { data[0] = result[1];
                            data[1] = result[2];
                            Ok(())
                          },
            Err(e) => Err(e)
        }
    }
}
