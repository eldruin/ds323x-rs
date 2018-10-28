#[deny(warnings)]

extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
use hal::spi::Transaction as SpiTrans;
mod common;
use common::{ DEVICE_ADDRESS as DEV_ADDR, Register as Reg, new_ds3231,
              new_ds3232, new_ds3234, BitFlags as BF };

macro_rules! call_method_test {
    ($method:ident, $i2c_transactions:expr, $spi_transactions:expr) => {
        call_test!(can_call_ds3231, $method, new_ds3231, $i2c_transactions);
        call_test!(can_call_ds3232, $method, new_ds3232, $i2c_transactions);
        call_test!(can_call_ds3234, $method, new_ds3234, $spi_transactions);
    };
}

mod do_nothing_if_already_enabled {
    use super::*;
    call_method_test!(enable,
        [ I2cTrans::write_read(DEV_ADDR, vec![Reg::CONTROL], vec![0]) ],
        [ SpiTrans::transfer(vec![Reg::CONTROL, 0], vec![Reg::CONTROL, 0]) ]);
}

mod enable {
    use super::*;
    call_method_test!(enable,
        [ I2cTrans::write_read(DEV_ADDR, vec![Reg::CONTROL], vec![BF::EOSC | 0b0101_1010]),
          I2cTrans::write(DEV_ADDR, vec![Reg::CONTROL, 0b0101_1010]) ],

        [ SpiTrans::transfer(vec![Reg::CONTROL, 0], vec![Reg::CONTROL, BF::EOSC | 0b0101_1010]),
          SpiTrans::write(vec![Reg::CONTROL + 0x80, 0b0101_1010]) ]);
}

mod do_nothing_if_already_disabled {
    use super::*;
    call_method_test!(disable,
        [ I2cTrans::write_read(DEV_ADDR, vec![Reg::CONTROL], vec![BF::EOSC]) ],
        [ SpiTrans::transfer(vec![Reg::CONTROL, 0], vec![Reg::CONTROL, BF::EOSC]) ]);
}

mod disable {
    use super::*;
    call_method_test!(disable,
        [ I2cTrans::write_read(DEV_ADDR, vec![Reg::CONTROL], vec![0b0101_1010]),
          I2cTrans::write(DEV_ADDR, vec![Reg::CONTROL, BF::EOSC | 0b0101_1010]) ],

        [ SpiTrans::transfer(vec![Reg::CONTROL, 0], vec![Reg::CONTROL, 0b0101_1010]),
          SpiTrans::write(vec![Reg::CONTROL + 0x80, BF::EOSC | 0b0101_1010]) ]);
}
