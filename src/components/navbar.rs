use yew::prelude::*;

#[function_component(MyNavBar)]
pub fn navbar() -> Html {
    html! {
         <nav class="border fixed split-nav">
            <div class="nav-brand">
                {"Train Sim"}
            </div>
         </nav>
    }
}