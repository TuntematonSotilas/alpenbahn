use yew::prelude::*;

mod components;

use components::train::TrainsList;
use components::navbar::MyNavBar;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <MyNavBar/>
            <TrainsList/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}