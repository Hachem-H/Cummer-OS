use crate::gdt;
use crate::keyboard;
use crate::print;
use crate::vga_buffer;

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
pub static CLOCK_COUNTER: Mutex<u64> = Mutex::new(0);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        *CLOCK_COUNTER.lock() += 1;

        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => unsafe {
                    if keyboard::IS_GET {
                        if character.is_ascii_alphanumeric()
                            || character.is_ascii_punctuation()
                            || character.is_ascii_graphic()
                            || character == ' '
                            || character == '\n'
                        {
                            print!("{}", character);
                        }

                        match character {
                            '\x0A' => {
                                keyboard::IS_DONE = true;
                                keyboard::IS_GET = false;
                            }

                            '\x09' => {
                                print!("    ");
                                keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX + 0] = b' ';
                                keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX + 1] = b' ';
                                keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX + 2] = b' ';
                                keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX + 3] = b' ';
                                keyboard::INPUT_BUFFER_INDEX += 4;
                            }

                            '\x08' => {
                                if keyboard::INPUT_BUFFER_INDEX != 0 {
                                    vga_buffer::WRITER.lock().delete_byte();

                                    keyboard::INPUT_BUFFER_INDEX -= 1;
                                    keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX] = 0;
                                }
                            }

                            _ => {
                                keyboard::INPUT_BUFFER[keyboard::INPUT_BUFFER_INDEX] =
                                    character as u8;
                                keyboard::INPUT_BUFFER_INDEX += 1;
                            }
                        }
                    }

                    if keyboard::IS_WAIT {
                        if character == '\n' {
                            keyboard::IS_WAIT = false;
                        }
                    }
                },
                DecodedKey::RawKey(_) => {}
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

pub fn init() {
    IDT.load();
}
