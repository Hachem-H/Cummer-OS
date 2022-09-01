use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use crate::interrupts;
use crate::keyboard;
use crate::vga_buffer;

use crate::print;
use crate::println;

pub fn run() {
    vga_buffer::WRITER.lock().clear_screen();

    let seed = *interrupts::CLOCK_COUNTER.lock();
    let mut generator = SmallRng::seed_from_u64(seed);
    let random_number = generator.next_u64() % 100;

    loop {
        let mut input_buffer: [u8; 256] = [0; 256];
        print!("Guess a number between 0-100: ");
        keyboard::get_input(&mut input_buffer);
        let number_string = crate::to_string(&input_buffer);
        let number = u64::from_str_radix(number_string, 10);

        match number {
            Ok(number) => {
                if number < random_number {
                    vga_buffer::WRITER.lock().clear_screen();
                    println!("Guess higher");
                } else if number > random_number {
                    vga_buffer::WRITER.lock().clear_screen();
                    println!("Guess lower");
                } else if number == random_number {
                    vga_buffer::WRITER.lock().clear_screen();
                    println!("Correct! you gud son");
                    break;
                }
            }
            Err(_) => println!("Not a valid number bru: {}", number_string),
        }
    }

    keyboard::wait();
}
