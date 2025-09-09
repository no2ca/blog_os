#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "Hello Writer!\nHello New Line!\n").unwrap();
    print!("Hello                  {}", "println!");
    panic!("This panic is intentional!!");
    // loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga_buffer::Color;
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();
    writer.set_color(vga_buffer::ColorCode::new(Color::Red, Color::Black));
    write!(writer, "{}", info).unwrap();
    loop {}
}
