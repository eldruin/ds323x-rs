//! Set the Alarm1 to each week on a week day at a specific time
extern crate linux_embedded_hal as hal;
extern crate ds323x;
use ds323x::{ Ds323x, Hours, WeekdayAlarm1, Alarm1Matching };

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut rtc = Ds323x::new_ds3232(dev);
    let alarm1 = WeekdayAlarm1 {
        weekday: 1,
        hour: Hours::H24(7),
        minute: 2,
        second: 15
    };
    rtc.set_alarm1_weekday(alarm1, Alarm1Matching::AllMatch).unwrap();

    let _dev = rtc.destroy_ds3232();
}
