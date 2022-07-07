use commander_components::Alert;
use config::{Color, ColorPair, Level};
use yew::prelude::*;

pub fn render() -> Html {
    html! {
                <div class="p-4">
                    <h1 class="text-2xl">{ "Alert" }</h1>
                    <Alert text={ "Info Alert" } />
                    <Alert level={ Level::Warning } text={ "Warning Alert" } />
                    <Alert level={ Level::Danger } text={ "Danger Alert" } />
                    <Alert level={ Level::Success } text={ "Success Alert" } />
                    <Alert level={ Level::Dark } text={ "Dark Alert" } />
                    <Alert level={ Level::Custom {
                        text: ColorPair::new(Color::Purple700, Color::Purple800),
                        bg: ColorPair::new(Color::Purple100, Color::Purple200),
                        border: Color::Purple500,
                    } } text={ "Custom Alert" } />
                    <pre>
                        <code>
    { r#"<Alert text={ "Info Alert" } />
<Alert level={ Level::Warning } text={ "Warning Alert" } />
<Alert level={ Level::Danger } text={ "Danger Alert" } />
<Alert level={ Level::Success } text={ "Success Alert" } />
<Alert level={ Level::Dark } text={ "Dark Alert" } />
<Alert level={ Level::Custom { 
    text: ColorPair::new(Color::Purple700, Color::Purple800),
    bg: ColorPair::new(Color::Purple100, Color::Purple200),
    border: Color::Purple500,
} } text={ "Custom Alert" } />"# }
                        </code>
                    </pre>
                    <h1 class="text-2xl">{ "Label" }</h1>
                    <Alert level={ Level::Info } label={ "Heads Up" } text={ "It is your turn next!" } />
                    <pre>
                        <code>
                            { r#"<Alert level={ Level::Info } label={ "Heads Up" } text={ "It is your turn next!" } />"# }
                        </code>
                    </pre>
                    <h1 class="text-2xl">{ "Border" }</h1>
                    <Alert level={ Level::Info } text={ "Info Alert" } border={ true } />
                    <pre>
                        <code>
                            { r#"<Alert level={ Level::Info } text={ "Info Alert" } border={ true } />"# }
                        </code>
                    </pre>
                    <h1 class="text-2xl"> { "Rounded" }</h1>
                    <Alert level={ Level::Info } text={ "Info Alert" } rounded={ true } />
                    <pre>
                        <code>
                            { r#"<Alert level={ Level::Info } text={ "Info Alert" } rounded={ true } />"# }
                        </code>
                    </pre>
                </div>
            }
}
