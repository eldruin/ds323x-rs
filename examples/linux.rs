use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Rtcc};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3231(dev);
    let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    // do something else...
    let time = rtc.time().unwrap();
    println!("Time: {}", time);

    let _dev = rtc.destroy_ds3231();
}
