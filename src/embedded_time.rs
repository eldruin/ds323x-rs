//! embedded-time `Clock` implementation

use crate::{
    interface::{ReadData, WriteData},
    Ds323x, Error, Rtcc,
};
use core::cell::RefCell;
use core::convert::From;
use embedded_time::{clock, Clock, Instant, Period};

/// Wrapper error
#[derive(Debug)]
pub enum WrapperError<CommE, PinE> {
    /// Device could not be acquired. It may be already acquired.
    CouldNotAcquireDevice,
    /// Other error
    Other(Error<CommE, PinE>),
}

/// Wrapper around `Ds323x` driver to support `embedded-time`.
pub struct Ds323xWrapper<DI, IC> {
    dev: RefCell<Ds323x<DI, IC>>,
}

impl<CommE, PinE, DI, IC> Clock for Ds323xWrapper<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Number of non-leap-seconds since January 1, 1970 UTC
    type Rep = u64;
    const PERIOD: Period = Period::new(1, 1);
    type ImplError = WrapperError<CommE, PinE>;

    fn now(&self) -> Result<Instant<Self>, clock::Error<Self::ImplError>> {
        let datetime = self
            .dev
            .try_borrow_mut()
            .map_err(|_| clock::Error::Other(Self::ImplError::CouldNotAcquireDevice))?
            .get_datetime()
            .map_err(|e| clock::Error::Other(Self::ImplError::Other(e)))?;
        Ok(Instant::new((datetime.timestamp_millis() as u64) / 1_000))
    }
}

impl<CommE, PinE, DI, IC> From<Ds323x<DI, IC>> for Ds323xWrapper<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    fn from(dev: Ds323x<DI, IC>) -> Self {
        Ds323xWrapper {
            dev: RefCell::new(dev),
        }
    }
}

impl<DI, IC> Ds323xWrapper<DI, IC> {
    /// Return inner `Ds323x` driver instance.
    pub fn into_inner(self) -> Ds323x<DI, IC> {
        self.dev.into_inner()
    }
}
/*
impl<CommE, PinE, DI, IC> Ds323xWrapper<DI, IC>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Run function on mutable borrowed inner device
    pub fn do_on_borrow_mut<R>(
        &self,
        f: impl FnOnce(RefMut<Ds323x<DI, IC>>) -> Result<R, Error<CommE, PinE>>,
    ) -> Result<R, WrapperError<CommE, PinE>> {
        let dev = self
            .dev
            .try_borrow_mut()
            .map_err(|_| WrapperError::<CommE, PinE>::CouldNotAcquireDevice)?;
        f(dev).map_err(WrapperError::Other)
    }
}
*/
