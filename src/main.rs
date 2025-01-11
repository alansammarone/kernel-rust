#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    println!("Hello world{}", "!");
    // println!("Test");
    // panic!("Some error");

    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    // loop {}
}
