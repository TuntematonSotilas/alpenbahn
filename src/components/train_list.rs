use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::train_store::{Train, TrainStore};

#[function_component(TrainsList)]
pub fn train_list() -> Html {
    
    let mut trains = Vec::<Train>::new();
    trains.push(Train { name: "T1".to_string() });
    trains.push(Train { name: "T2".to_string() });
    
    let dispatch = Dispatch::<TrainStore>::new();
    dispatch.reduce_mut(|state| state.trains = trains);

    let state = dispatch.get();

    state.trains.iter().map(|train| html! {
        <p>{&train.name}</p>
    }).collect()
    
}