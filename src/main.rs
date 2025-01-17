#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

// Panic handler when not running tests
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// Panic handler when running tsts
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // serial_println!("Hello world! {}", "test");
    println!("Hello world!");
    print!("Hello world 2!");
    // serial_println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop {}
}

// QEMU + tests plumbing

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// Tests
#[test_case]
fn some_assertion() {
    assert_eq!(1, 1);
}
