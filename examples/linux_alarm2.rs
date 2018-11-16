//! Set the Alarm2 to each day at the same time and enable interrupts on output.
//!
//! The INT/SQW output pin will be set to 1 when it the alarm matches.

extern crate linux_embedded_hal as hal;
extern crate ds323x;
use ds323x::{ Ds323x, Hours, DayAlarm2, Alarm2Matching };

fn main() {
    let dev = hal::Spidev::open("/dev/spidev0.0").unwrap();
    let chip_select = hal::Pin::new(24);
    let mut rtc = Ds323x::new_ds3234(dev, chip_select);
    let alarm2 = DayAlarm2 {
        day: 1, // does not matter given the chosen matching
        hour: Hours::AM(11),
        minute: 2
    };
    rtc.set_alarm2_day(alarm2, Alarm2Matching::HoursAndMinutesMatch).unwrap();
    rtc.use_int_sqw_output_as_interrupt().unwrap();
    rtc.enable_alarm2_interrupts().unwrap();

    let (_dev, _chip_select_pin) = rtc.destroy_ds3234();
}
