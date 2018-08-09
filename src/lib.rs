extern crate combine;

use std::collections::HashMap;

use combine::*;
use combine::parser::choice::or;
use combine::parser::char::string;

pub enum HeadphoneButton {
    Play,
    Up,
    Down,
}
type Trigger = Vec<HeadphoneButton>;
type Action = String;

#[derive(Debug, PartialEq)]
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


fn map_kind<I>() -> impl Parser<Input = I, Output = MapKind>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    or(
        string("map").map(|_| MapKind::Map),
        string("cmd").map(|_| MapKind::Command),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_kind_parses_kind_map() {
        let text = "map";
        let result = map_kind().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(MapKind::Map));
    }

    #[test]
    fn map_kind_parses_kind_command() {
        let text = "cmd";
        let result = map_kind().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(MapKind::Command));
    }
}
