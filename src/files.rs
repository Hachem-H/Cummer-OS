pub fn intro() -> &'static str {
    r#"CummerOS is a joke operating system created by me as a birthday present 
for one of my friends. It started with a project of mine called OS-TK, but 
soon realized that developing that project, and the kernel at the same time 
would set unrealistic time constraints, so with a week to spare, I rewrote 
the entire kernel in rust since most the boiler plate code is already setup.

The filesystem is not real, there is HDD/SSD drivers or anything of that 
sorts, its more of a memory map, but I mean, that what most operating 
systems do ;). It has a complete-ish shell with a series of commands accessible 
through `help` and maybe a set of games if i had the time."#
}

pub fn pron() -> &'static str {
    r#"Why are you here?
*bonk* stop you horni now
smh, he thinks that i will add pron to this OS"#
}

pub fn best_feet() -> &'static str {
    r#"PEOPLE WITH THE BEST FEET:
=========================

- Hachem
- Hachem H.
- Hachem Haydar
- H. Haydar
- Mafem"#
}

pub fn some_file() -> &'static str {
    r#"You can ignore this file, its mainly used to test if my VFS is setup properly and memory and what not.
Yk all that boring jargon."#
}
