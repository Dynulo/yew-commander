use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub children: Children,
}

#[function_component]
pub fn Example(_props: &ExampleProps) -> Html {
    html! {
        <></>
    }
}
