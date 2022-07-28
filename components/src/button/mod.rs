use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Option<Children>,
    pub class: Option<Classes>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = classes!(
        "inline-flex",
        "justify-center",
        "py-2",
        "px-4",
        "border",
        "border-gray-300",
        "rounded-md",
        "shadow-sm",
        "text-sm",
        "font-medium",
        "text-white"
    );
    if let Some(prop_classes) = &props.class {
        classes.extend(prop_classes.to_owned());
    }
    let inner = {
        if let Some(inner) = &props.children {
            inner.clone()
        } else {
            ChildrenRenderer::new(vec![])
        }
    };
    html! {
        <button class={ classes }>
            { inner }
        </button>
    }
}
