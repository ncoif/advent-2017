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

pub fn answer1(input: &str) -> u64 {
    let stream = parse_input(&input);
    scoring(&stream, 0, 0, false)
}

fn parse_input(input: &str) -> Vec<Char> {
    stream_parser(CompleteStr(input)).unwrap().1
}

fn scoring(stream: &[Char], depth: u64, score: u64, is_garbage: bool) -> u64 {
    use self::Char::*;
    let first_char = stream.get(0);

    match (first_char, is_garbage) {
        (Some(OpenStream), false) => scoring(&stream[1..], depth + 1, score, is_garbage),
        (Some(CloseStream), false) => scoring(&stream[1..], depth - 1, score + depth, is_garbage),
        (Some(OpenGarbage), false) => scoring(&stream[1..], depth, score, true),
        (Some(CloseGarbage), true) => scoring(&stream[1..], depth, score, false),
        (Some(Negate), _) => scoring(&stream[2..], depth, score, is_garbage),
        (Some(Other), _) => scoring(&stream[1..], depth, score, is_garbage),
        (None, _) => score,
        (_, _) => scoring(&stream[1..], depth, score, is_garbage), //skip
    }
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

#[test]
fn test_answer1() {
    assert_eq!(answer1("{}"), 1);
    assert_eq!(answer1("{{{}}}"), 6);
    assert_eq!(answer1("{{},{}}"), 5);
    assert_eq!(answer1("{{{},{},{{}}}}"), 16);
    assert_eq!(answer1("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(answer1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(answer1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(answer1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}
