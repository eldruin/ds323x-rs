use ds323x::SqWFreq;
use embedded_hal_mock::{i2c::Transaction as I2cTrans, spi::Transaction as SpiTrans};

mod common;
use self::common::{
    destroy_ds3231, destroy_ds3232, destroy_ds3234, new_ds3231, new_ds3232, new_ds3234,
    BitFlags as BF, Register, CONTROL_POR_VALUE, DEVICE_ADDRESS as DEV_ADDR, DS3231_POR_STATUS,
    DS323X_POR_STATUS,
};

macro_rules! call_triple_test {
    ($name:ident, $method:ident, $i2c_transactions:expr, $spi_transactions:expr) => {
        mod $name {
            use super::*;
            call_test!(
                can_call_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                $i2c_transactions
            );
            call_test!(
                can_call_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                $i2c_transactions
            );
            call_test!(
                can_call_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                $spi_transactions
            );
        }
    };
}

macro_rules! call_method_test {
    ($name:ident, $method:ident, $register:ident, $value_enabled:expr) => {
        call_triple_test!(
            $name,
            $method,
            [I2cTrans::write(
                DEV_ADDR,
                vec![Register::$register, $value_enabled]
            )],
            [SpiTrans::write(vec![
                Register::$register + 0x80,
                $value_enabled
            ])]
        );
    };
}

macro_rules! call_method_status_test {
    ($name:ident, $method:ident, $value_ds3231:expr, $value_ds323x:expr) => {
        mod $name {
            use super::*;
            call_test!(
                can_call_ds3231,
                $method,
                new_ds3231,
                destroy_ds3231,
                [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::STATUS, $value_ds3231]
                )]
            );
            call_test!(
                can_call_ds3232,
                $method,
                new_ds3232,
                destroy_ds3232,
                [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::STATUS, $value_ds323x]
                )]
            );
            call_test!(
                can_call_ds3234,
                $method,
                new_ds3234,
                destroy_ds3234,
                [SpiTrans::write(vec![
                    Register::STATUS + 0x80,
                    $value_ds323x
                ])]
            );
        }
    };
}

macro_rules! change_if_necessary_test {
    ($name:ident, $method:ident, $register:ident, $value_enabled:expr, $value_disabled:expr) => {
        mod $name {
            use super::*;
            call_triple_test!(
                do_nothing_if_not_necessary,
                $method,
                [I2cTrans::write_read(
                    DEV_ADDR,
                    vec![Register::$register],
                    vec![$value_enabled]
                )],
                [SpiTrans::transfer(
                    vec![Register::$register, 0],
                    vec![Register::$register, $value_enabled]
                )]
            );

            call_triple_test!(
                change,
                $method,
                [
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::$register],
                        vec![$value_disabled]
                    ),
                    I2cTrans::write(DEV_ADDR, vec![Register::$register, $value_enabled])
                ],
                [
                    SpiTrans::transfer(
                        vec![Register::$register, 0],
                        vec![Register::$register, $value_disabled]
                    ),
                    SpiTrans::write(vec![Register::$register + 0x80, $value_enabled])
                ]
            );
        }
    };
}

call_method_test!(enable, enable, CONTROL, CONTROL_POR_VALUE & !BF::EOSC);
call_method_test!(disable, disable, CONTROL, CONTROL_POR_VALUE | BF::EOSC);
call_method_status_test!(
    en_32khz_out,
    enable_32khz_output,
    DS3231_POR_STATUS | BF::EN32KHZ | BF::ALARM2F | BF::ALARM1F,
    DS323X_POR_STATUS | BF::EN32KHZ | BF::ALARM2F | BF::ALARM1F
);
call_method_status_test!(
    dis_32khz_out,
    disable_32khz_output,
    DS3231_POR_STATUS & !BF::EN32KHZ | BF::ALARM2F | BF::ALARM1F,
    DS323X_POR_STATUS & !BF::EN32KHZ | BF::ALARM2F | BF::ALARM1F
);

call_method_status_test!(
    clear_alarm1_matched,
    clear_alarm1_matched_flag,
    DS3231_POR_STATUS | BF::ALARM2F,
    DS323X_POR_STATUS | BF::ALARM2F
);

call_method_status_test!(
    clear_alarm2_matched,
    clear_alarm2_matched_flag,
    DS3231_POR_STATUS | BF::ALARM1F,
    DS323X_POR_STATUS | BF::ALARM1F
);

call_method_status_test!(
    clr_stop,
    clear_has_been_stopped_flag,
    DS3231_POR_STATUS & !BF::OSC_STOP | BF::ALARM2F | BF::ALARM1F,
    DS323X_POR_STATUS & !BF::OSC_STOP | BF::ALARM2F | BF::ALARM1F
);

change_if_necessary_test!(
    conv_temp,
    convert_temperature,
    CONTROL,
    CONTROL_POR_VALUE | BF::TEMP_CONV,
    CONTROL_POR_VALUE & !BF::TEMP_CONV
);

call_method_test!(
    en_al1_int,
    enable_alarm1_interrupts,
    CONTROL,
    CONTROL_POR_VALUE | BF::ALARM1_INT_EN
);
call_method_test!(
    dis_al1_int,
    disable_alarm1_interrupts,
    CONTROL,
    CONTROL_POR_VALUE & !BF::ALARM1_INT_EN
);

call_method_test!(
    en_al2_int,
    enable_alarm2_interrupts,
    CONTROL,
    CONTROL_POR_VALUE | BF::ALARM2_INT_EN
);
call_method_test!(
    dis_al2_int,
    disable_alarm2_interrupts,
    CONTROL,
    CONTROL_POR_VALUE & !BF::ALARM2_INT_EN
);

set_param_test!(
    set_aging_offset_min,
    set_aging_offset,
    AGING_OFFSET,
    -128,
    0b1000_0000
);
set_param_test!(
    set_aging_offset_max,
    set_aging_offset,
    AGING_OFFSET,
    127,
    127
);

get_param_test!(
    get_aging_offset_min,
    get_aging_offset,
    AGING_OFFSET,
    -128,
    0b1000_0000
);
get_param_test!(
    get_aging_offset_max,
    get_aging_offset,
    AGING_OFFSET,
    127,
    127
);

call_method_test!(
    int_sqw_out_int,
    use_int_sqw_output_as_interrupt,
    CONTROL,
    CONTROL_POR_VALUE | BF::INTCN
);
call_method_test!(
    int_sqw_out_sqw,
    use_int_sqw_output_as_square_wave,
    CONTROL,
    CONTROL_POR_VALUE & !BF::INTCN
);

call_method_test!(
    enable_sqw,
    enable_square_wave,
    CONTROL,
    CONTROL_POR_VALUE | BF::BBSQW
);
call_method_test!(
    disable_sqw,
    disable_square_wave,
    CONTROL,
    CONTROL_POR_VALUE & !BF::BBSQW
);

set_param_test!(
    set_sqw_freq_1,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::_1Hz,
    CONTROL_POR_VALUE & !BF::RS2 & !BF::RS1
);
set_param_test!(
    set_sqw_freq_1_024,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::_1_024Hz,
    CONTROL_POR_VALUE & !BF::RS2 | BF::RS1
);
set_param_test!(
    set_sqw_freq_4_096,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::_4_096Hz,
    CONTROL_POR_VALUE | BF::RS2 & !BF::RS1
);
set_param_test!(
    set_sqw_freq_8_192,
    set_square_wave_frequency,
    CONTROL,
    SqWFreq::_8_192Hz,
    CONTROL_POR_VALUE | BF::RS2 | BF::RS1
);
