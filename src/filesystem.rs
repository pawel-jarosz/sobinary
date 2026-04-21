use std::path::Path;
use std::io;
use std::fs;

struct ElfFileDatabase {

}

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path)?; // rekurencja
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}

pub struct ElfFinder {

}

impl ElfFinder {
    pub fn new() -> ElfFinder {
        ElfFinder {  }
    }

    pub fn find_files(&self, path: &str) {
        let dir = Path::new(path);
        visit_dirs(dir);
    }
}