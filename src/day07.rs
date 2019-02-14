use nom::types::CompleteStr;
use petgraph::Graph;

pub fn title() -> &'static str {
    "Day 7: Recursive Circus"
}

#[derive(Debug, PartialEq)]
struct Line {
    name: String,
    weight: u32,
    children: Option<Vec<String>>,
}

named!(
    name_parser<String>,
    map_res!(
        map_res!(nom::alpha, std::str::from_utf8),
        std::str::FromStr::from_str
    )
);

named!(
    weight_parser<u32>,
    map_res!(
        map_res!(
            delimited!(char!('('), is_not!(")"), char!(')')),
            std::str::from_utf8
        ),
        std::str::FromStr::from_str
    )
);

named!(
    children_parser<Vec<String>>,
    separated_nonempty_list!(tag!(", "), name_parser) // map_res!(many0!(name: name_parser >> opt!(tag!(", "))), std::str::FromStr::from_str)
);

named!(
    line_parser<Line>,
    do_parse!(
        name: name_parser
            >> tag!(" ")
            >> weight: weight_parser
            >> opt!(tag!(" -> "))
            >> children: opt!(children_parser)
            >> (Line {
                name: name.to_string(),
                weight: weight,
                children: children
            })
    )
);

pub fn answer1(input: &str) -> String {
    let graph = parse_input(input);

    String::from("")
}

fn parse_input(input: &str) -> Graph<(&str, u32), (&str, u32)> {
    let mut graph = Graph::<(&str, u32), (&str, u32)>::new();

    let lines = input.split('\n');
    for line in lines.filter(|l| *l != "") {
        let line = line_parser(line.as_bytes());
        println!("{:?}", line);
        //dbg!(&children);
    }

    graph

    // let pg = deps.add_node("petgraph");
    // let fb = deps.add_node("fixedbitset");
    // let qc = deps.add_node("quickcheck");
    // let rand = deps.add_node("rand");
    // let libc = deps.add_node("libc");
    // deps.extend_with_edges(&[
    //     (pg, fb), (pg, qc),
    //     (qc, rand), (rand, libc), (qc, libc),
    // ]);
}

#[test]
fn parse_simple_name() {
    assert_eq!(name_parser(b"pbga "), Ok((&b" "[..], "pbga".to_string())));
    assert_eq!(name_parser(b"pbga\n"), Ok((&b"\n"[..], "pbga".to_string())));
}

#[test]
fn parse_simple_weight() {
    assert_eq!(weight_parser(b"(66) "), Ok((&b" "[..], 66)));
    assert_eq!(weight_parser(b"(66)\n"), Ok((&b"\n"[..], 66)));
}

#[test]
fn parse_simple_list() {
    assert_eq!(
        children_parser(b"ktlj, cntj, xhth\n"),
        Ok((&b"\n"[..], vec_of_strings!["ktlj", "cntj", "xhth"]))
    );
    assert_eq!(
        children_parser(b"ktlj, cntj, xhth "),
        Ok((&b" "[..], vec_of_strings!["ktlj", "cntj", "xhth"]))
    );
}

#[test]
fn parse_line_without_children() {
    assert_eq!(
        line_parser(b"pbga (66)\n"),
        Ok((
            &b"\n"[..],
            Line {
                name: "pbga".to_string(),
                weight: 66,
                children: None
            }
        ))
    );
}

#[test]
fn parse_line_with_children() {
    assert_eq!(
        line_parser(b"fwft (72) -> ktlj, cntj, xhth\n"),
        Ok((
            &b"\n"[..],
            Line {
                name: "fwft".to_string(),
                weight: 72,
                children: Some(vec_of_strings!["ktlj", "cntj", "xhth"])
            }
        ))
    );
}

#[test]
fn test_answer1() {
    let input = String::from(
        r#"
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#,
    );

    assert_eq!(answer1(&input), "tknk");
}
