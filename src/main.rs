#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(akane::test_runner)]
#![reexport_test_harness_main = "test_main"] // generate and call "test_main" instead of "main" when testing
#![allow(unused_imports)]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "println!");
    
    // initialize IDT
    akane::init();
    
    // invoke a breakpoint exeption
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
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