use std::env;
use std::io::Write;
use std::path::Path;

use walkdir::WalkDir;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("all.rs");
    let mut file = std::fs::File::create(&dest_path).unwrap();
    for entry in WalkDir::new("./src").into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let source = std::fs::read_to_string(entry.path()).unwrap();
            file.write_all(source.as_bytes()).unwrap();
        }
    }
}
