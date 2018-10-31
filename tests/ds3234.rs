#[deny(warnings)]

extern crate embedded_hal_mock as hal;
extern crate ds323x;

#[allow(unused)]
mod common;
use common::{ new_ds3234, destroy_ds3234 };

#[test]
fn can_create_and_destroy_ds3234() {
    let dev = new_ds3234(&[]);
    destroy_ds3234(dev);
}
