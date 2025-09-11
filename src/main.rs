#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // generate and call "test_main" instead of "main" when testing

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    Sucsess = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "println!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga_buffer::Color;
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();
    writer.set_color(vga_buffer::ColorCode::new(Color::Red, Color::Black));
    write!(writer, "{}", info).unwrap();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Sucsess);
}

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("ok");
}