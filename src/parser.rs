use std::collections::HashMap;

use combine::*;
use combine::easy::Errors as CombineErrors;
use combine::parser::choice::or;
use combine::parser::char::{
    newline,
    space,
    string,
    string_cmp,
    tab,
};
use combine::parser::repeat::take_until;
use combine::stream::state::{SourcePosition, State};

#[repr(C)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum HeadphoneButton {
    Play,
    Up,
    Down,
}
type Trigger = Vec<HeadphoneButton>;
type Action = String;

#[repr(C)]
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

#[derive(Debug, PartialEq)]
struct Mode {
    trigger: Trigger,
    maps: MapCollection,
}

#[derive(Debug, PartialEq)]
pub struct MapGroup {
    pub maps: MapCollection,
    pub modes: HashMap<Trigger, MapCollection>,
}

#[derive(Debug, PartialEq)]
enum Definition {
    Map(Map),
    Mode(Mode),
}

impl MapGroup {
    pub fn parse(
        mappings: &str
    ) -> Result<MapGroup, CombineErrors<char, &str, SourcePosition>> {
        let input = State::new(mappings);
        map_group().easy_parse(input).map(|t| t.0)
    }
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

fn mode<I>() -> impl Parser<Input = I, Output = Mode>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("mode"),
        whitespace_separator(),
        trigger(),
        whitespace_separator(),
        token('{'),
        map_collection(),
        token('}'), // Verify that this is parsed on its own line, not inside a map
    ).map(|(_, _, trigger, _, _, collection, _)|
        Mode {
            trigger: trigger,
            maps: collection,
        }
    )
}

fn definitions<I>() -> impl Parser<Input = I, Output = Vec<Definition>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        blank(),
        many(
            choice!(
                try(map()).map(|map| Definition::Map(map)),
                try(mode()).map(|mode| Definition::Mode(mode))
            ).skip(blank())
        )
    ).map(|(_, definitions)| definitions)
}

fn map_group<I>() -> impl Parser<Input = I, Output = MapGroup>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    definitions()
        .map(|definitions| {
            let mut maps = HashMap::new();
            let mut modes = HashMap::new();

            for definition in definitions {
                match definition {
                    Definition::Map(map) => {
                        maps.insert(
                            map.trigger,
                            MapAction {
                                action: map.action,
                                kind: map.kind,
                            }
                        );
                    },
                    Definition::Mode(mode) => {
                        modes.insert(
                            mode.trigger,
                            mode.maps,
                        );
                    },
                }
            }

            MapGroup {
                maps: maps,
                modes: modes,
            }
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

    #[test]
    fn mode_parses_a_mode() {
        let text = "mode <down><up> {
	cmd <up><play> echo hello
	map <down> insert {}
  	}";
        let result = mode().parse(text).map(|t| t.0);

        let mut expected = Mode {
            trigger: vec![HeadphoneButton::Down, HeadphoneButton::Up],
            maps: HashMap::new(),
        };

        expected.maps.insert(
            vec![HeadphoneButton::Up, HeadphoneButton::Play],
            MapAction {
                action: "echo hello".to_owned(),
                kind: MapKind::Command,
            },
        );
        expected.maps.insert(
            vec![HeadphoneButton::Down],
            MapAction {
                action: "insert {}".to_owned(),
                kind: MapKind::Map,
            },
        );

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn definitions_parses_modes_and_maps() {
        let text = "

mode <up> {
    cmd <down> j
}
map <play> m
mode <down><up> {
    cmd <down> j
}

map <down> k
";
        let result = definitions().easy_parse(text).map(|t| t.0);

        let mut mode_up_maps = HashMap::new();
        mode_up_maps.insert(
            vec![HeadphoneButton::Down],
            MapAction {
                action: "j".to_owned(),
                kind: MapKind::Command,
            }
        );

        let mut mode_down_up_maps = HashMap::new();
        mode_down_up_maps.insert(
            vec![HeadphoneButton::Down],
            MapAction {
                action: "j".to_owned(),
                kind: MapKind::Command,
            }
        );

        let expected = vec![
            Definition::Mode(Mode {
                trigger: vec![HeadphoneButton::Up],
                maps: mode_up_maps,
            }),
            Definition::Map(Map {
                trigger: vec![HeadphoneButton::Play],
                action: "m".to_owned(),
                kind: MapKind::Map,
            }),
            Definition::Mode(Mode {
                trigger: vec![HeadphoneButton::Down, HeadphoneButton::Up],
                maps: mode_down_up_maps,
            }),
            Definition::Map(Map {
                trigger: vec![HeadphoneButton::Down],
                action: "k".to_owned(),
                kind: MapKind::Map,
            }),
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn map_group_parses_a_whole_map_file_string() {
        let text = "map <play> some text

# The following does nothing
cmd <down> /bin/echo nothing

mode <down><up> {
    map <play> p
}

cmd <play> /usr/bin/say hello
";
        let result = map_group().easy_parse(text).map(|t| t.0);

        let mut maps: MapCollection = HashMap::new();
        let mut modes: HashMap<Trigger, MapCollection> = HashMap::new();
        let mut mode_maps: MapCollection = HashMap::new();

        maps.insert(
            vec![HeadphoneButton::Down],
            MapAction {
                action: "/bin/echo nothing".to_owned(),
                kind: MapKind::Command,
            },
        );
        maps.insert(
            vec![HeadphoneButton::Play],
            MapAction {
                action: "/usr/bin/say hello".to_owned(),
                kind: MapKind::Command,
            },
        );

        mode_maps.insert(
            vec![HeadphoneButton::Play],
            MapAction {
                action: "p".to_owned(),
                kind: MapKind::Map,
            },
        );
        modes.insert(
            vec![HeadphoneButton::Down, HeadphoneButton::Up],
            mode_maps,
        );

        let expected = MapGroup {
            maps: maps,
            modes: modes,
        };

        assert_eq!(result, Ok(expected));
    }
}
