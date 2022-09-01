# Cummer OS
A joke operating system dedicated to one of my friends, originally started with my custom toolkit [OS-TK](https://github.com/Hachem-H/OS-TK.back), but it quickly became a hassle since I had to manage creating a product and a gift at the same time (with a limited time). So with 2 weeks to spare, I decided to rewrite the entire kernel in [rust](https://rust-lang.org) since a lot of the work has already been done in packaged, (IDT/GDT, boot loaders, ports, ...).
And yes, this OS should work on real hardware (tested).

It has a variety of features, such as a  virtual file system to store kernel loaded files `(VFS)`, a VGA and keyboard driver, and more importantly, three simple games for you to enjoy (`TicTacToe`, `A random number guesser` and `A simple text adventure`) and even a full command line shell allowing you to interact with the shell.

What it doesn't have is a real ACPI rooting, so if you are running this outside an emulator (like `qemu`), you wont be able to shutoff the machine other than using a force power-off, that shouldn't be an issue either since I haven't implemented a heap, all static data is loaded from the stack (implementing paging and page tables would have been probably better but honestly it didn't seem necessary, it's a one use project and will probably never develop this ever again). Using the stack, I made a stupid `VFS` implementation which just loads static strings from memory and maps them file names. 

## Getting started

To first get started, make sure you have rust installed from the [rust website](https://www.rust-lang.org/learn/get-started), after you have that, make sure you are running the `nightly` branch of rust and that you have the `bootimage` package installed.

```sh
$ rustup toolchain install nightly
$ cargo install bootimage
```

When you have that ready, you can simply clone the repository and compile the OS using cargo.

```sh
$ git clone https://github.com/Hachem-H/Cummer-OS.git
$ cd Cummer-OS
$ cargo bootimage
```

## Emulating in QEMU

If you want to run this OS in an emulator, I would personally recommend using `qemu` as it's what I used to develop it and it also allows raw binary files (not ISOs, and its hard to get it to work using virtual-box). The repository is already configured to use `qemu` as a default emulator, so all you have to do is run the `cargo run` command and let it do it's thing.

## Running on real hardware

Yes, this piece of technical wonder can run on real hardware and I have tested it myself, all you have to is load the generated binary on a USB using the `dd` command. 

```sh
# All this after running `cargo run` or `cargo bootimage`
$ dd if=target/target/debug/bootimage-cummer-os.bin of=/dev/[Your device ID]
$ sync
```

## Project Structure

```sh
.
├── .cargo
│   └── config.toml       # preconfigure commands and QEMU
├── src
│   ├── files.rs          # Preloaded static files
│   ├── gdt.rs            # GDT and TSS code
│   ├── interrupts.rs     # IDT and interrupt handles
│   ├── kernel.rs         # The kernel and shell
│   ├── keyboard.rs       # Keyboard driver
│   ├── main.rs           # Entry point and pre-kernel init
│   ├── number_guesser.rs # Random number guessing game
│   ├── text_adventure.rs # Simple text adventure game
│   ├── tictactoe.rs      # TicTacToe clone
│   ├── vfs.rs            # Virtual File system
│   └── vga_buffer.rs     # VGA/Display driver
├── Cargo.toml            # Project Setup and dependencies
├── LICENSE
├── README.md
├── rust-toolchain.toml   # Tells rust what toolchain to use
└── target.json           # Custom architecture schematic
```

