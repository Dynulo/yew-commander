use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // config colors
    let color_path = Path::new(&out_dir).join("colors.html");
    std::fs::write(
        &color_path,
        format!("<p class=\"{}\"></p>", config::Color::every_color()),
    )
    .unwrap();

    let dest_path = Path::new(&out_dir).join("generated.css");
    Command::new("tailwindcss")
        .arg("--content")
        .arg(format!(
            "./src/**/*.{{html,rs}},{}/*",
            &out_dir.to_string_lossy()
        ))
        .arg("-o")
        .arg(&dest_path)
        .output()
        .expect("failed to execute process");
}
