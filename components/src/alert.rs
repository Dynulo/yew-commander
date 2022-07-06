use config::{Level, Width, Height};
use icons::Icon;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AlertProps {
    pub level: Level,
    pub label: Option<String>,
    pub text: String,

    pub children: Option<Children>,
}

#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let label = props.label.clone().map(|label| html! {
        <span class="font-medium">{ label }</span>
    }).unwrap_or_default();
    let children = props.children.clone().unwrap_or_default();
    let mut class = classes!(
        "p-4",
        "mb-4",
        "text-sm",
        "rounded-lg"
    );
    class.extend(props.level.text().as_text_classes());
    class.extend(props.level.bg().as_bg_classes());
    html! {
        <div {class} role="alert">
            { label } { &props.text }
            { children }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AlertIconProps<I>
where
    I: Icon,
{
    pub icon: I,
}

#[function_component]
pub fn AlertIcon<I>(props: &AlertIconProps<I>) -> Html
where
    I: Icon,
{
    html! {
        { props.icon.render(Width::W5, Height::H5) }
    }
}
    