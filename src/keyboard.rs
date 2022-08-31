pub static mut IS_GET: bool = false;
pub static mut IS_DONE: bool = false;
pub static mut INPUT_BUFFER: [u8; 256] = [0; 256];
pub static mut INPUT_BUFFER_INDEX: usize = 0;

pub static mut IS_WAIT: bool = false;

pub fn get_input(buffer: &mut [u8]) {
    unsafe {
        IS_GET = true;
        INPUT_BUFFER = [0; 256];
        INPUT_BUFFER_INDEX = 0;

        while !IS_DONE {}

        if IS_DONE {
            for i in 0..INPUT_BUFFER_INDEX {
                buffer[i] = INPUT_BUFFER[i];
            }
            IS_DONE = false;
        }
    }
}

pub fn wait() {
    unsafe {
        IS_WAIT = true;
        while IS_WAIT {}
    }
}
