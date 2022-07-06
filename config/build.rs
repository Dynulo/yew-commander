use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.css");
    Command::new("tailwindcss")
        .arg("--content")
        .arg("./src/**/*.{html,rs}")
        .arg("-o")
        .arg(&dest_path)
        .output()
        .expect("failed to execute process");
}
