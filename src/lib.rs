extern crate combine;

use std::collections::HashMap;

pub enum Trigger {
    Play,
    Up,
    Down,
}
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
    maps: HashMap<Vec<Trigger>, Map>,
    modes: HashMap<Vec<Trigger>, HashMap<Vec<Trigger>, Map>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
