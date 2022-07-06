mod color;
pub use color::*;

mod level;
pub use level::*;

mod size;
pub use size::*;

pub fn css() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/generated.css"))
}
