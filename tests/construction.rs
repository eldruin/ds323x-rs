mod common;
use common::{ new_ds3231, destroy_ds3231, new_ds3232, destroy_ds3232,
              new_ds3234, destroy_ds3234 };

macro_rules! construction_test {
    ($name:ident, $create:ident, $destroy:ident) => {
        #[test]
        fn $name() {
            let dev = $create(&[]);
            $destroy(dev);
        }
    };
}

construction_test!(can_create_ds3231, new_ds3231, destroy_ds3231);
construction_test!(can_create_ds3232, new_ds3232, destroy_ds3232);
construction_test!(can_create_ds3234, new_ds3234, destroy_ds3234);
