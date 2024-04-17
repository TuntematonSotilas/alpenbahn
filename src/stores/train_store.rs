use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct Train {
    pub name: String
}


#[derive(Default, PartialEq, Clone, Store)]
pub struct TrainStore {
    pub trains: Vec::<Train>
}