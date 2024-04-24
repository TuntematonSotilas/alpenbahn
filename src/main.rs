use yew::prelude::*;

mod components;
mod stores;

use components::train_list::TrainsList;
use components::new_train::NewTrain;
use components::navbar::MyNavBar;

#[function_component]
fn App() -> Html {
    html! {
        <div class="col">
            <MyNavBar/>
            <TrainsList/>
            <NewTrain/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}