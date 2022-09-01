# Cummer OS
A joke operating system dedicated to one of my friends, originally started with my custom toolkit [OS-TK](https://github.com/Hachem-H/OS-TK.back), but it quickly became a hastle since I had to manage creating a product and a gift at the same time (with a limited time). So with 2 weeks to spare, I decided to rewrite the entire kernel in [rust](https://rust-lang.org) since a lot of the work has already been done in packaged, (IDT/GDT, bootloaders, ports, ...).
And yes, this OS should work on real hardware (tested).

**Requirements:**
- `rust` (with the `nightly toolchain`)
- `rust bootimage`; Installed with `cargo install bootimage`
- `qemu`

## Running/Emulating
I preconfigured cargo to run the OS in qemu.
```sh
$ cargo run
```

## Flashing on USB to boot from real hardware
```sh
$ cargo build
$ dd if=/target/target/debug/bootimage-cummer-os.bin of=/dev/[device ID]
$ sync
```
