#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "Hello Writer!\nHello New Line!\n").unwrap();
    println!("Hello {}", "println!");
    panic!("Wooo!! This panic is intentional!!");
    // loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let mut red_writer = vga_buffer::Writer::new_writer_red();
    write!(red_writer, "{}", info).unwrap();
    loop {}
}
