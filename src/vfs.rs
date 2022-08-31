use lazy_static::lazy_static;
use spin::Mutex;

use crate::files;

lazy_static! {
    pub static ref VFS: Mutex<VirtualFileSystem> = Mutex::new(VirtualFileSystem::new());
}

pub struct VirtualFileSystem {
    pub files: [(&'static str, &'static str); 4],
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            files: [
                ("intro.txt", files::intro()),
                ("some file.txt", files::some_file()),
                ("people with the best feet.txt", files::best_feet()),
                ("pron >//<.txt", files::pron()),
            ],
        }
    }

    pub fn get(&self, path: &str) -> Option<&str> {
        let index = self.files.iter().position(|&file| file.0 == path);
        match index {
            Some(index) => Some(self.files[index].1),
            None => None,
        }
    }
}
