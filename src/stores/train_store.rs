use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub enum LocoType {
    #[default]
    Pacific
} 

#[derive(Default, Clone, PartialEq)]
pub enum WagonType {
    #[default]
    Passenger,
    Goods,
} 

#[derive(Default, Clone, PartialEq)]
pub struct Loco {
    pub typ: LocoType
}

#[derive(Default, Clone, PartialEq)]
pub struct Wagon {
    pub typ: WagonType
}


#[derive(Default, Clone, PartialEq)]
pub struct Train {
    pub loco: Loco,
    pub name: String,
    pub wagons: Vec<Wagon>,
}


#[derive(Default, PartialEq, Clone, Store)]
pub struct TrainStore {
    pub trains: Vec::<Train>
}