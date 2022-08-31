#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod gdt;
mod interrupts;
mod kernel;
mod keyboard;
mod tictactoe;
mod vga_buffer;

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn to_string(buffer: &[u8]) -> &str {
    let mut string = core::str::from_utf8(buffer)
        .unwrap()
        .trim_matches(0 as char);
    if string.ends_with('\n') {
        string = string.trim_end();
    }
    string
}

fn init() {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    kernel::main();
    hlt();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_buffer::WRITER
        .lock()
        .set_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::White,
            vga_buffer::Color::Red,
        ));
    print!("{}", info);
    hlt();
}
