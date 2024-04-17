use yew::prelude::*;

mod components;
mod stores;

use components::train_list::TrainsList;
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