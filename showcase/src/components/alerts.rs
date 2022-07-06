use components::Alert;
use config::Level;
use yew::prelude::*;

pub fn render() -> Html {
    html! {
        <div class="p-4">
            <Alert level={ Level::Info } text={ "Info Alert" } />
            <Alert level={ Level::Warning } text={ "Warning Alert" } />
        </div>
    }
}
