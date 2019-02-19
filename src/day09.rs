use nom::types::CompleteStr;
use nom::{many0, map_res, named, take};
use std::str::FromStr;

pub fn title() -> &'static str {
    "Day 09: Stream Processing"
}

#[derive(Debug, PartialEq)]
enum Char {
    OpenStream,
    CloseStream,
    OpenGarbage,
    CloseGarbage,
    Negate,
    Other,
}

impl FromStr for Char {
    type Err = ();

    fn from_str(s: &str) -> Result<Char, ()> {
        use self::Char::*;
        match s {
            "{" => Ok(OpenStream),
            "}" => Ok(CloseStream),
            "<" => Ok(OpenGarbage),
            ">" => Ok(CloseGarbage),
            "!" => Ok(Negate),
            _ => Ok(Other),
        }
    }
}

named!(
    take1<CompleteStr, Char>,
    map_res!(take!(1), |CompleteStr(s)| Char::from_str(s))
 );

named!(
    stream_parser<CompleteStr, Vec<Char>>,
    many0!(take1)
);

pub fn answer1(input: &str) -> i32 {
    let instructions = parse_input(&input);

    0
}

fn parse_input(input: &str) -> Vec<Char> {
    stream_parser(CompleteStr(input)).unwrap().1
}

#[test]
fn parse_stream() {
    use self::Char::*;
    assert_eq!(
        stream_parser(CompleteStr::from("<{oi!}a,<{i<a>")),
        Ok((
            CompleteStr::from(""),
            vec![
                OpenGarbage,
                OpenStream,
                Other,
                Other,
                Negate,
                CloseStream,
                Other,
                Other,
                OpenGarbage,
                OpenStream,
                Other,
                OpenGarbage,
                Other,
                CloseGarbage
            ]
        ))
    );
}
