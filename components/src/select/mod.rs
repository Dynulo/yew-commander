use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};
use yew_hooks::prelude::*;

mod option;
pub use option::SelectOption;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectProps<O>
where
    O: SelectOption + PartialEq,
{
    pub options: Vec<O>,
    #[prop_or(false)]
    pub filter: bool,

    pub onchange: Callback<O>,
}

#[function_component]
pub fn Select<O>(props: &SelectProps<O>) -> Html
where
    O: SelectOption + PartialEq + Clone + 'static,
{
    let expanded = use_bool_toggle(false);
    let selected = use_state(|| None);
    let index = use_state(|| None);
    let filter = use_state(String::new);

    let reset_filter = {
        let filter = filter.clone();
        use_timeout(
            move || {
                filter.set(String::new());
            },
            200,
        )
    };

    // Collapse the select if it is expanded and the user clicks outside of it.
    let node = use_node_ref();
    use_click_away(node.clone(), {
        let reset_filter = reset_filter.clone();
        let expanded = expanded.clone();
        move |_: Event| {
            reset_filter.reset();
            expanded.set(false);
        }
    });
    use_event(node.clone(), "keydown", {
        let index_state = index.clone();
        let expanded = expanded.clone();
        let options = props.options.clone();
        let reset_filter = reset_filter.clone();
        let selected = selected.clone();
        let onchange = props.onchange.clone();
        move |e: KeyboardEvent| {
            let mut index = *index_state;
            if index.is_none() {
                index = Some(options.len());
            };
            match e.key().as_str() {
                "Down" | "ArrowDown" => {
                    if index.unwrap() < options.len() - 1 {
                        index = Some(index.unwrap() + 1);
                    }
                    if index.unwrap() == options.len() {
                        index = Some(0);
                    }
                }
                "Up" | "ArrowUp" => {
                    if index.unwrap() != 0 {
                        index = Some(index.unwrap() - 1);
                    }
                }
                "Esc" | "Escape" | "Space" => {
                    reset_filter.reset();
                    expanded.set(false);
                }
                _ => (),
            }
            index_state.set(index);
            selected.set(Some(options[index.unwrap()].clone()));
            onchange.emit(options[index.unwrap()].clone());
        }
    });

    let mut dropdown_class = classes!(
        "absolute",
        "z-10",
        "mt-1",
        "w-full",
        "bg-white",
        "dark:bg-slate-700",
        "shadow-lg",
        "max-h-56",
        "rounded-md",
        "py-1",
        "text-base",
        "ring-1",
        "ring-black",
        "ring-opacity-5",
        "overflow-auto",
        "focus:outline-none",
        "sm:text-sm",
        "transition",
        "origin-top",
    );
    if *expanded {
        dropdown_class.extend(classes!(
            "ease-out",
            "duration-75",
            "opacity-100",
            "scale-100",
        ))
    } else {
        dropdown_class.extend(classes!("ease-in", "duration-100", "opacity-0", "scale-0",));
    };

    let options = props.options.clone().into_iter().enumerate().map(|(i, option)| {
        if props.filter && !filter.is_empty() && !option.label().contains(&*filter) {
            return VNode::default();
        }
        let mut class = classes!("cursor-default", "select-none", "relative", "py-2", "pl-3", "pr-9");
        let check = if *selected == Some(option.to_owned()) {
            class.extend(classes!("bg-slate-100", "dark:bg-slate-800", "text-slate-900", "dark:text-slate-200"));
            html! {
                <span class="text-indigo-600 dark:text-indigo-300 absolute inset-y-0 right-0 flex items-center pr-4">
                    <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
                        aria-hidden="true">
                        <path fill-rule="evenodd"
                            d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                            clip-rule="evenodd" />
                    </svg>
                </span>
            }
        } else {
            VNode::default()
        };
        let icon = option.icon().unwrap_or_default();
        let select_handler = {
            let option = option.clone();
            let index = index.clone();
            let selected = selected.clone();
            let reset_filter = reset_filter.clone();
            let expanded = expanded.clone();
            let onchange = props.onchange.clone();
            Callback::from(move |_| {
                index.set(Some(i));
                selected.set(Some(option.clone()));
                reset_filter.reset();
                expanded.set(false);
                onchange.emit(option.clone());
            })
        };
        html! {
            <li
                onclick = { select_handler }
                { class } id="listbox-option-0"
                role="option">
                <div class="flex items-center">
                    { icon }
                    <span class="font-normal ml-3 block truncate"> { option.label() } </span>
                </div>
                { check }
            </li>
        }
    }).collect::<Vec<_>>();

    let expand_handler = { Callback::from(move |_| expanded.toggle()) };

    let filter_handler = {
        let filter = filter.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            filter.set(input.value());
        })
    };

    let current = if let Some(selected) = &*selected {
        let icon = selected.icon().unwrap_or_default();
        html! {
            <>
                { icon }
                <span class="ml-3 block truncate"> { selected.label() } </span>
            </>
        }
    } else {
        html! {
            <span class="ml-3 block truncate"> { "Select" } </span>
        }
    };

    let search = if props.filter {
        html! {
            <input class="focus:border-current focus:ring-0 focus:outline-none  block w-full py-2 pl-3 text-sm leading-5 font-medium rounded-md dark:bg-slate-700" type="text" placeholder="Search..." value={ filter.to_string() } oninput={ filter_handler } />
        }
    } else {
        VNode::default()
    };

    html! {
        <div class="mt-1 relative" ref={ node }>
            <button type="button"
                onclick = { expand_handler }
                class="relative w-full rounded-md shadow-sm pl-3 pr-10 py-2 text-left cursor-default sm:text-sm bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:focus:ring-blue-500 dark:focus:border-blue-500"
                aria-haspopup="listbox">
                <span class="flex items-center">
                    { current }
                </span>
                <span class="ml-3 absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                    <svg class="h-5 w-5 text-slate-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"
                        fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd"
                            d="M10 3a1 1 0 01.707.293l3 3a1 1 0 01-1.414 1.414L10 5.414 7.707 7.707a1 1 0 01-1.414-1.414l3-3A1 1 0 0110 3zm-3.707 9.293a1 1 0 011.414 0L10 14.586l2.293-2.293a1 1 0 011.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
                            clip-rule="evenodd" />
                    </svg>
                </span>
            </button>
            <ul class={ dropdown_class }
                tabindex="-1" role="listbox">
                { search }
                { options }
            </ul>
        </div>
    }
}
