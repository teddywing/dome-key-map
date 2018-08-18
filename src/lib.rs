#[macro_use]
extern crate combine;

use std::collections::HashMap;

use combine::*;
use combine::parser::choice::or;
use combine::parser::char::{
    newline,
    space,
    string,
    string_cmp,
    tab,
};
use combine::parser::repeat::take_until;

#[derive(Debug, Hash, Eq, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct MapAction {
    pub action: Action,
    pub kind: MapKind,
}

#[derive(Debug, PartialEq)]
struct Map {
    trigger: Trigger,
    action: Action,
    kind: MapKind,
}

type MapCollection = HashMap<Trigger, MapAction>;

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

fn headphone_button<I>() -> impl Parser<Input = I, Output = HeadphoneButton>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('<'),
        token('>'),
        choice!(
            string_cmp("play", |l, r| l.eq_ignore_ascii_case(&r))
                .map(|_| HeadphoneButton::Play),
            string_cmp("up", |l, r| l.eq_ignore_ascii_case(&r))
                .map(|_| HeadphoneButton::Up),
            string_cmp("down", |l, r| l.eq_ignore_ascii_case(&r))
                .map(|_| HeadphoneButton::Down)
        ),
    )
}

fn trigger<I>() -> impl Parser<Input = I, Output = Trigger>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many1(headphone_button())
}

fn action<I>() -> impl Parser<Input = I, Output = Action>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    take_until(newline())
}

fn whitespace_separator<I>() -> impl Parser<Input = I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    skip_many1(space().or(tab()))
}

fn map<I>() -> impl Parser<Input = I, Output = Map>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        map_kind(),
        whitespace_separator(),
        trigger(),
        whitespace_separator(),
        action()
    ).map(|(kind, _, trigger, _, action)|
        Map {
            trigger: trigger,
            action: action,
            kind: kind,
        }
    )
}

fn map_collection<I>() -> impl Parser<Input = I, Output = MapCollection>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        blank(),
        many::<Vec<Map>, _>(map().skip(blank())),
    ).map(|(_, collection)| {
        let mut maps = HashMap::new();

        for map in collection {
            maps.insert(
                map.trigger,
                MapAction {
                    action: map.action,
                    kind: map.kind,
                }
            );
        }

        maps
    })
}

fn comment<I>() -> impl Parser<Input = I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('#'),
        skip_many(satisfy(|c| c != '\n')),
    )
}


fn blank<I>() -> impl Parser<Input = I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let comment = comment().map(|_| ());
    let whitespace = whitespace_separator().map(|_| ());
    skip_many(skip_many1(newline()).or(whitespace).or(comment))
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

    #[test]
    fn headphone_button_parses_play() {
        let text = "<play>";
        let result = headphone_button().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(HeadphoneButton::Play));
    }

    #[test]
    fn headphone_button_ignores_case() {
        let text = "<Play>";
        let result = headphone_button().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(HeadphoneButton::Play));
    }

    #[test]
    fn trigger_parses_headphone_button_sequence() {
        let text = "<up><down><play>";
        let result = trigger().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(vec![
            HeadphoneButton::Up,
            HeadphoneButton::Down,
            HeadphoneButton::Play,
        ]));
    }

    #[test]
    fn action_parses_string_to_end_of_line() {
        let text = "/usr/bin/say 'hello'
";
        let expected: Action = "/usr/bin/say 'hello'".to_owned();
        let result = action().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn map_parses_map_line() {
        let text = "map <play><down> test
";
        let expected = Map {
            trigger: vec![HeadphoneButton::Play, HeadphoneButton::Down],
            action: "test".to_owned(),
            kind: MapKind::Map,
        };
        let result = map().parse(text).map(|t| t.0);

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn map_collection_parses_maps() {
        let text = "
# Test comment
    # continued

map <up><down> test
map <play> salt and pepper

# Another comment
cmd <down> /usr/bin/say 'hello'
";
        let result = map_collection().easy_parse(text).map(|t| t.0);

        let mut expected = HashMap::new();
        expected.insert(
            vec![HeadphoneButton::Up, HeadphoneButton::Down],
            MapAction {
                action: "test".to_owned(),
                kind: MapKind::Map,
            },
        );
        expected.insert(
            vec![HeadphoneButton::Play],
            MapAction {
                action: "salt and pepper".to_owned(),
                kind: MapKind::Map,
            },
        );
        expected.insert(
            vec![HeadphoneButton::Down],
            MapAction {
                action: "/usr/bin/say 'hello'".to_owned(),
                kind: MapKind::Command,
            },
        );

        assert_eq!(result, Ok(expected));
    }
}
