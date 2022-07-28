use config::{Height, Width};
use yew::{prelude::*, virtual_dom::VNode};

mod brands;
pub use brands::*;
pub mod spinner;

pub trait AsSvg {
    fn as_svg(&self) -> VNode;
}

pub trait Icon: PartialEq {
    fn render(&self, width: Width, height: Height) -> VNode;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SrcIcon {
    src: String,
}

impl Icon for SrcIcon {
    fn render(&self, width: Width, height: Height) -> VNode {
        let class = classes!(width.class(), height.class());
        html! {
            <img { class } src={ self.src.clone() } />
        }
    }
}

pub fn source() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/all.rs"))
}
