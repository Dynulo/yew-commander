use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod example;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/alerts")]
    Alerts,
    #[at("/select")]
    Select,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={ switch } />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Alerts => components::alerts::render(),
        Route::Select => html! { <components::select::Render /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
