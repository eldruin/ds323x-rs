extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate ds323x;

use linux_embedded_hal::I2cdev;
use ds323x::Ds323x;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3231(dev);
    println!("Seconds: {}", rtc.get_seconds().unwrap());
}
