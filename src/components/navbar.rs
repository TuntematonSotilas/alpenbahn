use yew::prelude::*;

#[function_component(MyNavBar)]
pub fn navbar() -> Html {
    html! {
         <header class="sticky">
            <div class="container">
                <div class="nav-brand">
                    {"Train Sim"}
                </div>
            </div>
         </header>
    }
}