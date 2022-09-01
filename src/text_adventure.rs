use crate::keyboard;
use crate::print;
use crate::println;
use crate::vga_buffer;

fn get_input(prompt: &str) -> bool {
    loop {
        let mut input_buffer: [u8; 256] = [0; 256];
        print!("{} (Y/N): ", prompt);
        keyboard::get_input(&mut input_buffer);
        let command = crate::to_string(&input_buffer);

        if command.len() != 1 {
            println!("Only one character supported");
            continue;
        }

        let choice = command.chars().nth(0).unwrap();
        if choice == 'Y' || choice == 'y' {
            return true;
        } else if choice == 'N' || choice == 'n' {
            return false;
        } else {
            println!("{} is not a valid option bruh.", choice);
        }
    }
}

pub fn run() {
    vga_buffer::WRITER.lock().clear_screen();
    println!("Welcome to this space adventure");
    println!("You wake up on a space station");

    println!("It seems like you are in a closet, a small nondescript closet");
    println!("There is a control room near you.");
    let control_room = get_input("Go to control room?");
    if control_room {
        println!(
            "The guard sees you. Without saying a word, he pulls his laser gun and kills you."
        );
        println!("Game over.");
    } else {
        println!("You decide to go in an airlock room instead");
        println!("The door is locked you need an ID.\n");

        let get_id = get_input("There is a guard next door, shoot him and steal the ID?");
        if get_id {
            println!("You steal the ID but you notice that there is no lock on this side");
            println!("You look around and find the lock room but there are people here");

            let fight = get_input("Fight the people?");
            if fight {
                println!("This is not a marvel movie, you dont win, you die");
            } else {
                println!("They wanted to help you, they lead you to the lock");
                println!("You use the key");
                println!("You escape");
            }
        } else {
            println!("You absolute moron, you live your entire life stuck on this spaceship");
            println!("You die of starvation");
            println!("Game over.");
        }
    }

    println!("");
    keyboard::wait();
}
