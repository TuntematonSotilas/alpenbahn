use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;

use crate::stores::train_store::{Loco, LocoType, Train, TrainStore};

#[function_component(TrainsList)]
pub fn train_list() -> Html {
    
    let dispatch = use_dispatch::<TrainStore>();

     
    // This runs only once, on the first render of the component.
    use_effect_with(
        (), // empty deps
        move |_| {
            // Set fake data
            let mut trains = Vec::<Train>::new();
            trains.push(Train { name: "Pacific".to_string(), loco: Loco { typ: LocoType::Pacific}, wagons: Vec::new() });
                   
            dispatch.set(TrainStore { trains });
            || {}
        },
    );

    let dispatch = use_dispatch::<TrainStore>();
    let state: Rc<TrainStore> = dispatch.get();

    state.trains.iter().map(|train| html! {
        <p>{&train.name}</p>
    }).collect()
    
}