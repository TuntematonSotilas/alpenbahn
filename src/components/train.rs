use yew::prelude::*;

struct Train {
    name: String
}

#[function_component(TrainsList)]
pub fn train_list() -> Html {
    let mut trains = Vec::<Train>::new();
    trains.push(Train { name: "T1".to_string() });
    trains.push(Train { name: "T2".to_string() });
    let state_trains = use_state(|| trains);

    state_trains.iter().map(|train| html! {
        <p>{&train.name}</p>
    }).collect()
}