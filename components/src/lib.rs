mod alert;
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
    std::fs::write(
        &dest_path,
        "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n"
    ).unwrap();
    std::fs::write(
        &dest_path,
        css(),
    ).unwrap();
    std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg(&dest_path)
        .arg("--content")
        .arg("./src/**/*.{html,rs}")
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to execute process");
}
