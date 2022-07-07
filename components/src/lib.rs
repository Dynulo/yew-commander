use std::io::Read;

mod alert;
pub use alert::*;
mod menu;
pub use menu::*;
mod select;
pub use select::*;

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

    // config colors
    let color_path = std::path::Path::new(&out_dir).join("colors.html");
    std::fs::write(
        &color_path,
        format!("<p class=\"{}\"></p>", config::Color::every_color()),
    )
    .unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let local_path = std::path::Path::new(&out_dir).join("local.css");
    std::process::Command::new("tailwindcss")
        .arg("--content")
        .arg(format!(
            "{}/*.html,./src/**/*.{{html,rs}},./index.html",
            out_dir.to_string_lossy()
        ))
        .arg("-o")
        .arg(&local_path)
        .output()
        .expect("failed to execute process");
    let mut buffer = String::new();
    std::fs::File::open(&local_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    std::fs::write(
        &dest_path,
        format!(
            "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n{}\n{}",
            css(),
            buffer
        ),
    )
    .unwrap();

    std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg(&dest_path)
        .arg("--content")
        .arg(format!(
            "{}/*,./src/**/*.{{html,rs}},./index.html",
            out_dir.to_string_lossy()
        ))
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to execute process");
}
