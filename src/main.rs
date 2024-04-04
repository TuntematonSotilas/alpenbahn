use yew::prelude::*;

mod components;

use components::train::TrainsList;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <TrainsList/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}