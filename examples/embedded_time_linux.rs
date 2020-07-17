use ds323x::{Ds323x, Ds323xWrapper, NaiveDate, Rtcc};
use embedded_time::{duration::units::Hours, Clock};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3231(dev);
    let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    let rtc = Ds323xWrapper::from(rtc);
    // do something else...
    let instant = rtc.now().unwrap();
    let hours_since_epoch = instant.duration_since_epoch::<Hours>().unwrap();
    println!("Hours since epoch: {}", hours_since_epoch);

    let _dev = rtc.into_inner().destroy_ds3231();
}
