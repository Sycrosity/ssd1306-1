//! Helpers for use in examples and tests

use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};
use embedded_hal::
    digital::{ErrorType, OutputPin};
use embedded_hal_async::{
    i2c,
    spi::{self, SpiBus},
};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Error {}

impl embedded_hal::digital::Error for Error {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl i2c::Error for Error {
    fn kind(&self) -> i2c::ErrorKind {
        i2c::ErrorKind::Other
    }
}

impl spi::Error for Error {
    fn kind(&self) -> spi::ErrorKind {
        spi::ErrorKind::Other
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SpiStub;

impl spi::ErrorType for SpiStub {
    type Error = Error;
}

impl SpiBus<u8> for SpiStub {
    async fn read(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct I2cStub;

impl i2c::ErrorType for I2cStub {
    type Error = Error;
}

impl i2c::I2c for I2cStub {
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PinStub;

impl ErrorType for PinStub {
    type Error = Error;
}

impl OutputPin for PinStub {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct StubInterface;

impl AsyncWriteOnlyDataCommand for StubInterface {
    async fn send_commands(
        &mut self,
        _cmd: display_interface::DataFormat<'_>,
    ) -> Result<(), DisplayError> {
        Ok(())
    }
    async fn send_data(
        &mut self,
        _buf: display_interface::DataFormat<'_>,
    ) -> Result<(), DisplayError> {
        Ok(())
    }
}
