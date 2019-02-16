use nom::types::CompleteStr;
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};
use std::collections::HashMap;

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
    name_parser<CompleteStr, String>,
    map!(nom::alpha, |CompleteStr(s)| String::from(s))
);

named!(
    weight_parser<CompleteStr, u32>,
    map_res!(delimited!(char!('('), is_not!(")"), char!(')')), |CompleteStr(s)| u32::from_str_radix(s, 10))
);

named!(
    children_parser<CompleteStr, Vec<String>>,
    separated_nonempty_list!(tag!(", "), name_parser)
);

named!(
    line_parser<CompleteStr, Line>,
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
    let lines = parse_input(&input);
    let mut graph = Graph::<String, String>::new();

    // add all the nodes
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    for line in &lines {
        let name = &line.name;
        let node = graph.add_node(name.clone());
        nodes.insert(name.clone(), node);
    }

    // add all the edges
    for line in &lines {
        if line.children.is_some() {
            for child in &line.children.clone().unwrap() {
                graph.update_edge(nodes[&line.name], nodes[child], "no_weight".to_string());
            }
        }
    }

    let source_node = graph.externals(Direction::Incoming).next();
    graph[source_node.unwrap()].clone()
}

fn parse_input(input: &str) -> Vec<Line> {
    let lines = input.split('\n');

    lines
        .filter(|l| *l != "")
        .map(|l| line_parser(CompleteStr(l)).unwrap().1)
        .collect()
}

#[test]
fn parse_simple_name() {
    assert_eq!(
        name_parser(CompleteStr::from("pbga")),
        Ok((CompleteStr::from(""), "pbga".to_string()))
    );
}

#[test]
fn parse_simple_weight() {
    assert_eq!(
        weight_parser(CompleteStr::from("(66)")),
        Ok((CompleteStr::from(""), 66))
    );
}

#[test]
fn parse_simple_list() {
    assert_eq!(
        children_parser(CompleteStr::from("ktlj, cntj, xhth")),
        Ok((
            CompleteStr::from(""),
            vec_of_strings!["ktlj", "cntj", "xhth"]
        ))
    );
}

#[test]
fn parse_line_without_children() {
    assert_eq!(
        line_parser(CompleteStr::from("pbga (66)")),
        Ok((
            CompleteStr::from(""),
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
        line_parser(CompleteStr::from("fwft (72) -> ktlj, cntj, xhth")),
        Ok((
            CompleteStr::from(""),
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
