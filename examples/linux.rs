extern crate ds323x;
extern crate linux_embedded_hal as hal;
use ds323x::{Ds323x, NaiveDate, Rtcc};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3231(dev);
    let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    // do something else...
    let time = rtc.get_time().unwrap();
    println!("Time: {}", time);

    let _dev = rtc.destroy_ds3231();
}
