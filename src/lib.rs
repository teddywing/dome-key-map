extern crate combine;

use std::collections::HashMap;

pub enum HeadphoneButton {
    Play,
    Up,
    Down,
}
type Trigger = Vec<HeadphoneButton>;
type Action = String;
pub enum MapKind {
    Map,
    Command,
}

pub struct Map {
    pub action: Action,
    pub kind: MapKind,
}
type MapCollection = HashMap<Trigger, Map>;

pub struct DKMapGroup {
    maps: MapCollection,
    modes: HashMap<Trigger, MapCollection>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
