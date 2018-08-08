extern crate combine;

use std::collections::HashMap;

type Trigger = String;
type Action = String;
pub enum MapKind {
    Map,
    Command,
}

pub struct Map {
    pub action: Action,
    pub kind: MapKind,
}

pub struct DKMapGroup {
    maps: HashMap<Trigger, Map>,
    modes: HashMap<Trigger, HashMap<Trigger, Map>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
