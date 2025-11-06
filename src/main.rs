#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // generate and call "test_main" instead of "main" when testing
#![allow(unused_imports)]

mod serial;
mod vga_buffer;

use blog_os::memory;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{
    VirtAddr,
    structures::paging::Page,
    structures::paging::{PageTable, Translate},
};

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}", "println!");

    // initialize IDT, GDT, PICS, interrputs
    blog_os::init();

    use blog_os::memory;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    println!("phys_mem_offset: {:#?}", phys_mem_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("virt {:?} -> phys {:?}", virt, phys);
    }

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
