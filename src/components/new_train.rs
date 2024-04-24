use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console::log;

use crate::stores::train_store::{Train, TrainStore};

#[function_component(NewTrain)]
pub fn train_list() -> Html {

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            log!("sumbit");
        })
    };

   html! {
        <form {onsubmit} class="paper container">

            <input
                class="form-control"
                type="text"
                placeholder="name"
                value="" />
                
            <button
                class="btn btn-lg pull-xs-right btn-primary"
                type="submit">
                { "Done" }
            </button>
            
        </form>
   }
    
}