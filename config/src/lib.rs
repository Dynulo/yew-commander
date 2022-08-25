mod color;
pub use color::*;

mod level;
pub use level::*;

mod size;
pub use size::*;

pub const fn source() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/all.rs"))
}
