extern crate embedded_hal_mock as hal;
extern crate ds323x;
use ds323x::Ds323x;
#[allow(dead_code)]
mod common;
use common::DummyOutputPin;

#[test]
fn can_create_and_destroy() {
    let dev = Ds323x::new_ds3234(hal::spi::Mock::new(&[]), DummyOutputPin);
    let (mut spi, _cs) = dev.destroy_ds3234();
    spi.done();
}
