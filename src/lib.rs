macro_rules! number {
    ($name:ident, $number:expr) => {
        #[no_mangle]
        pub extern "C" fn $name () -> u64 {
            $number
        }
    }
}

number!(two, 2);
