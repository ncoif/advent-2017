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

named!(name_parser<&str>, map_res!(nom::alpha, std::str::from_utf8));

named!(
    weight_parser<u32>,
    map_res!(
        map_res!(nom::digit, std::str::from_utf8),
        std::str::FromStr::from_str
    )
);

named!(
    line<Line>,
    do_parse!(
        name: name_parser
            >> tag!(" (")
            >> weight: weight_parser
            >> tag!(")")
            >> (Line {
                name: name.to_string(),
                weight: weight,
                children: None
            })
    )
);

pub fn answer1(input: &str) -> String {
    // let graph = parse_input(input);

    String::from("")
}

#[test]
fn parse_line_without_children() {
    assert_eq!(
        line(b"pbga (66)"),
        Ok((
            &b""[..],
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
        line(b"fwft (72) -> ktlj, cntj, xhth"),
        Ok((
            &b""[..],
            Line {
                name: "fwft".to_string(),
                weight: 72,
                children: Some(vec![
                    "ktlj".to_string(),
                    "cntj".to_string(),
                    "xhth".to_string()
                ])
            }
        ))
    );
}

// fn parse_input(input: &str) -> Graph<(&str, u32), (&str, u32)> {
//     let mut graph = Graph::<(&str, u32), (&str, u32)>::new();
////
//     let lines = input.split('\n');
//     for line in lines.filter(|l| *l != "") {
//         dbg!(&children);
//     }
//
//     graph
//
// // let pg = deps.add_node("petgraph");
// // let fb = deps.add_node("fixedbitset");
// // let qc = deps.add_node("quickcheck");
// // let rand = deps.add_node("rand");
// // let libc = deps.add_node("libc");
// // deps.extend_with_edges(&[
// //     (pg, fb), (pg, qc),
// //     (qc, rand), (rand, libc), (qc, libc),
// // ]);
// //     input
// //         .split(char::is_whitespace)
// //         .map(|n| n.parse::<u32>())
// //         .filter(|n| n.is_ok())
// //         .map(Result::unwrap) // safe because skip invalid elements
// //         .collect()
// }

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
