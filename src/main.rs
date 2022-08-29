#![no_std]
#![no_main]

mod vga_buffer;

#[rustfmt::skip]
fn intro() {
    println!("  .,-:::::  ...    :::.        :   .        :  .,:::::: :::::::..");
    println!(",;;;'````'  ;;     ;;;;;,.    ;;;  ;;,.    ;;; ;;;;'''' ;;;;``;;;;");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::Yellow, vga_buffer::Color::Black));
    println!("[[[        [['     [[[[[[[, ,[[[[, [[[[, ,[[[[, [[cccc   [[[,/[[[\'");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightGreen, vga_buffer::Color::Black));
    println!("$$$        $$      $$$$$$$$$$$\"$$$ $$$$$$$$\"$$$ $$\"\"\"\"   $$$$$$c");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightRed, vga_buffer::Color::Black));
    println!("`88bo,__,o,88    .d888888 Y88\" 888o888 Y88\" 888o888oo,__ 888b \"88bo,");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::Red, vga_buffer::Color::Black));
    println!("  \"YUMMMMMP\"\"YmmMMMM\"\"MMM  M\'  \"MMMMMM  M\'  \"MMM\"\"\"\"YUMMMMMMM   \"W\"");

    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black));
    println!("\n       A joke operating system made for a friend by Hachem H.");
    println!("                        Happy birthday mate\n\n");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightGray, vga_buffer::Color::Black));
    println!("To get a list of commands, start by typing `help`.");
    println!("Or because we are cool human creatures just type ?.");
    println!("Much shorter for you bous.\n\n");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black));
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    intro();

    loop {}
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
    loop {}
}
