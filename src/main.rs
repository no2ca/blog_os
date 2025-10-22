#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // generate and call "test_main" instead of "main" when testing
#![allow(unused_imports)]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}", "println!");
    
    // initialize IDT
    blog_os::init();
    
    // provoke a stack overflow
    /*
    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();
    */
    
    // invoke a page fault exeption
    // let ptr = 0x2031b2 as *mut u8;
    // unsafe {
    //     let x = *ptr;
    //     println!("x = {}", x);
    // }
    // unsafe { *ptr = 42; }
    // println!("write worked");

    // invoke a breakpoint exeption
    // x86_64::instructions::interrupts::int3();
    
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:#?}", level_4_page_table);

    #[cfg(test)]
    test_main();


    println!("It did not crash!");
    blog_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga_buffer::Color;
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();
    writer.set_color(vga_buffer::ColorCode::new(Color::Red, Color::Black));
    write!(writer, "{}", info).unwrap();
    blog_os::hlt_loop();
}