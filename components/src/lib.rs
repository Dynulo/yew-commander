mod alert;
use std::io::Read;

pub use alert::*;

pub fn css() -> String {
    let out = vec![
        icons::css(),
        config::css(),
        include_str!(concat!(env!("OUT_DIR"), "/generated.css")),
    ];
    out.join("\n")
}

pub fn build_tailwind() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("generated.css");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let local_path = std::path::Path::new(&out_dir).join("local.css");
    std::process::Command::new("tailwindcss")
        .arg("-c")
        .arg("./tailwind.config.js")
        .arg("-o")
        .arg(&local_path)
        .output()
        .expect("failed to execute process");
    let mut buffer = String::new();
    std::fs::File::open(&local_path).unwrap().read_to_string(&mut buffer).unwrap();
    buffer.push_str(&css());
    std::fs::write(
        &dest_path,
        buffer,
    ).unwrap();
    
    std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg(&dest_path)
        .arg("-c")
        .arg("./tailwind.config.js")
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to execute process");
}
