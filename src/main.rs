#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // panic!("Some error");
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();

    println!("Hello world{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Tests
#[test_case]
fn some_assertion() {
    print!("some_assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
