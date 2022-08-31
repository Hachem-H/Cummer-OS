use crate::keyboard;
use crate::print;
use crate::println;
use crate::tictactoe;
use crate::vga_buffer;

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
    println!("  \"YUMMMMMP\"\"YmmMMMM\"\"MMM  M\'  \"MMMMMcM  M\'  \"MMM\"\"\"\"YUMMMMMMM   \"W\"");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black));
    println!("\n       A joke operating system made for a friend by Hachem H.");
    println!("                        Happy birthday mate\n\n");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightGray, vga_buffer::Color::Black));
    println!("To get a list of commands, start by typing `help`.");
    println!("Or because we are cool human creatures just type ?.");
    println!("Much shorter for you bous.\n\n");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black));
}

#[rustfmt::skip]
fn help() {
    println!("Imagine needing help, what a noob, cant even use an OS which was never used");
    println!("before I will be nice and not tell you to go to the arch wiki like a pleb.");
    println!("here are the commands");
    println!("");
    println!("    - help/?: This is quite obvious i think.");
    println!("    - intro: if for some reason you didnt read it the first time.");
    println!("    - poweroff/commit die: these are obvious wtf.");
    println!("    - reboot/resurect: okay this is getting annoying.");
    println!("");
    println!("    - cd/rmdir/mkdir/touch/etc... dont exist, cause there is no real");
    println!("                                  filesystem, only a virtual one. Cut me");
    println!("                                  some slack, this entire thing took me a week.");
    println!("    - echo [phrase]: prints back the phrase if thats your jazz");
    println!("    - cat [file]: prints the data in a file");
    println!("    - ls: shows all file");
    println!("We got dem games");
    println!("    - tictactoe: every project of mine should have this oml.");
    println!("");
}

#[rustfmt::skip]
fn print_prompt() {
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::Yellow,    vga_buffer::Color::Black));
    print!("[");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White,     vga_buffer::Color::Black));
    print!("GigaChad");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightBlue, vga_buffer::Color::Black));
    print!("@");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightCyan, vga_buffer::Color::Black));
    print!("CummerOS");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::Yellow,    vga_buffer::Color::Black));
    print!("]");
    vga_buffer::WRITER.lock().set_color(vga_buffer::ColorCode::new(vga_buffer::Color::White,     vga_buffer::Color::Black));
    print!(" -> ");
}

pub fn main() {
    let vfs = crate::vfs::VFS.lock();

    intro();
    loop {
        let mut input_buffer: [u8; 256] = [0; 256];
        print_prompt();
        keyboard::get_input(&mut input_buffer);
        let command = crate::to_string(&input_buffer);

        match command {
            "tictactoe" => tictactoe::run(),

            "poweroff" | "commit die" => unimplemented!(),
            "reboot" | "resurect" => unimplemented!(),

            "help" | "?" => help(),
            "intro" => intro(),
            "clear" => vga_buffer::WRITER.lock().clear_screen(),

            "ls" => {
                for file in vfs.files {
                    println!("{} ", file.0);
                }
            }

            command if command.starts_with("cat ") => {
                let file = command.trim_start_matches("cat ");
                let data = vfs.get(file);
                match data {
                    Some(data) => println!("{}", data),
                    None => println!("What file is that, it doesnt exist lmfao"),
                }
            }

            command if command.starts_with("cat ") => {
                let file = command.trim_start_matches("cat ");
                let data = vfs.get(file);
                match data {
                    Some(data) => println!("{}", data),
                    None => println!("What file is that, it doesnt exist lmfao"),
                }
            }

            command if command.starts_with("echo ") => {
                let text = command.trim_start_matches("echo ");
                println!("{}", text);
            }

            "" => {}
            _ => println!("You idiot, the command \"{}\" doesn't exist, smh", command),
        }
    }
}
