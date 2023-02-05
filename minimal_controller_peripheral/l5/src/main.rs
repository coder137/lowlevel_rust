#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod entry_point;
mod rust_entry_point;

mod blink;

#[cfg(not(test))]
#[no_mangle]
fn main() -> ! {
    const TIME: u32 = 100_000;
    blink::blink_init();
    blink::blink_set();
    loop {
        blink::_spin_delay(TIME);
        blink::blink_reset();
        blink::_spin_delay(TIME);
        blink::blink_set();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_tests_work() {
        assert_eq!(1, 1);
    }
}
