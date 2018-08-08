extern crate combine;

use std::collections::HashMap;

type Trigger = String;
type Action = String;

pub struct Map {
    pub trigger: Trigger,
    pub action: Action,
    pub kind: String,
}

pub struct DKMapGroup {
    maps: HashMap<Trigger, Action>,
    modes: HashMap<Trigger, Map>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
