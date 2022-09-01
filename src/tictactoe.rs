use crate::keyboard;
use crate::print;
use crate::println;
use crate::vga_buffer;

static mut BOARD: [char; 3 * 3] = [' '; 3 * 3];
static mut IS_RUNNING: bool = true;
static mut CURRENT_PLAYER: u8 = 0;

fn get_player() -> char {
    unsafe {
        if CURRENT_PLAYER % 2 == 0 {
            'X'
        } else {
            'O'
        }
    }
}

fn draw_board() {
    unsafe {
        println!("   |   |   ");
        println!(" {} | {} | {} ", BOARD[0], BOARD[1], BOARD[2]);
        println!("   |   |   ");
        println!("---+---+---");
        println!("   |   |   ");
        println!(" {} | {} | {} ", BOARD[3], BOARD[4], BOARD[5]);
        println!("   |   |   ");
        println!("---+---+---");
        println!("   |   |   ");
        println!(" {} | {} | {} ", BOARD[6], BOARD[7], BOARD[8]);
        println!("   |   |   ");
    }
}

fn get_input() {
    let mut x;
    let mut y;

    loop {
        print!("Enter X and Y coordinate [1-3] (Ex: 1 1): ");
        let mut input_buffer: [u8; 256] = [0; 256];
        keyboard::get_input(&mut input_buffer);
        let choice = crate::to_string(&input_buffer);
        if choice.len() == 3 {
            let mut choices = choice.split_at(1);
            choices.1 = choices.1.trim_start();

            let x_temp = usize::from_str_radix(choices.0, 10);
            let y_temp = usize::from_str_radix(choices.1, 10);

            if x_temp.is_ok() && y_temp.is_ok() {
                x = x_temp.unwrap();
                y = y_temp.unwrap();

                if x < 1 || x > 3 || y < 1 || y > 3 {
                    println!("Dufus, between 1-3");
                } else {
                    let index = ((x - 1) + (y - 1) * 3) as usize;
                    unsafe {
                        if BOARD[index] == ' ' {
                            BOARD[index] = get_player();
                            vga_buffer::WRITER.lock().clear_screen();
                            break;
                        } else {
                            println!("Bozo the spot is already taken. smh");
                        }
                    }
                }
            } else {
                if x_temp.is_err() {
                    println!("ERR: {:?}", x_temp.err());
                }
                if y_temp.is_err() {
                    println!("ERR: {:?}", y_temp.err());
                }
            }
        }

        println!("bozo, like this (1 3) for x=1 & y=3");
    }
}

fn check_win() -> bool {
    unsafe {
        for i in 0..3 {
            if BOARD[i + 0 * 3] == BOARD[i + 1 * 3]
                && BOARD[i + 0 * 3] == BOARD[i + 2 * 3]
                && BOARD[i + 0 * 3] != ' '
            {
                IS_RUNNING = false;
                return true;
            }
        }

        for i in 0..3 {
            if BOARD[0 + i * 3] == BOARD[1 + i * 3]
                && BOARD[0 + i * 3] == BOARD[2 + i * 3]
                && BOARD[0 + i * 3] != ' '
            {
                IS_RUNNING = false;
                return true;
            }
        }

        if BOARD[0 + 0 * 3] == BOARD[1 + 1 * 3]
            && BOARD[1 + 1 * 3] == BOARD[2 + 2 * 3]
            && BOARD[1 + 1 * 3] != ' '
        {
            IS_RUNNING = false;
            return true;
        }

        if BOARD[0 + 2 * 3] == BOARD[1 + 1 * 3]
            && BOARD[1 + 1 * 3] == BOARD[2 + 0 * 3]
            && BOARD[1 + 1 * 3] != ' '
        {
            IS_RUNNING = false;
            return true;
        }
    }

    return false;
}

pub fn run() {
    vga_buffer::WRITER.lock().clear_screen();

    unsafe {
        while IS_RUNNING {
            draw_board();
            get_input();

            if !check_win() {
                CURRENT_PLAYER += 1;
            }
        }
    }

    println!("{} WON!\n", get_player());
    keyboard::wait();
}
