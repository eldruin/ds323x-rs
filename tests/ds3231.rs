extern crate embedded_hal_mock as hal;
extern crate ds323x;
use ds323x::Ds323x;

#[test]
fn can_create_and_destroy() {
    let dev = Ds323x::new_ds3231(hal::i2c::Mock::new(&[]));
    let mut i2c = dev.destroy_ds3231();
    i2c.done();
}
    