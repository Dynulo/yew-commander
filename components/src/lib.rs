use std::path::PathBuf;

pub use icons;

mod alert;
pub use alert::*;
mod button;
pub use button::*;
mod menu;
pub use menu::*;
mod select;
pub use select::*;

pub fn source() -> String {
    let out = vec![
        icons::source(),
        config::source(),
        include_str!(concat!(env!("OUT_DIR"), "/all.rs")),
    ];
    out.join("\n")
}

pub fn build_tailwind(src: PathBuf) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    // config colors
    let color_path = std::path::Path::new(&out_dir).join("colors.html");
    std::fs::write(
        &color_path,
        format!("<p class=\"{}\"></p>", config::Color::every_color()),
    )
    .unwrap();
    let source_path = std::path::Path::new(&out_dir).join("source.rs");
    std::fs::write(
        &source_path,
        source(),
    )
    .unwrap();

    std::process::Command::new("tailwindcss")
        .arg("--content")
        .arg(format!(
            "{}/*,{}/**/*.{{html,rs}},./index.html",
            out_dir.to_string_lossy(),
            src.display(),
        ))
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to execute process");

    
        println!("cargo:rerun-if-changed={}", src.display());
}
