use components::Select;
use yew::prelude::*;

#[function_component]
pub fn Render() -> Html {
    let selected = use_state(|| None);
    let onchange = {
        let selected = selected.clone();
        Callback::from(move |value: (i32,)| selected.set(Some(value.0)))
    };
    html! {
        <>
            <Select<(i32,)>
                options={ vec![
                    (1,),
                    (2,),
                    (3,),
                    (4,),
                    (5,),
                ] }
                filter={ true }
                { onchange }
            />
            { "Value: " } { format!("{:?}", *selected) }
        </>
    }
}
