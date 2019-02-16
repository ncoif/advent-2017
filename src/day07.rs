use nom::types::CompleteStr;
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};
use std::collections::{HashMap, HashSet};

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

struct GraphForRecursion {
    graph: Graph<(String, u32), u32>,
}

impl GraphForRecursion {
    fn leafs(&self) -> Vec<NodeIndex> {
        self.graph.externals(Direction::Outgoing).collect()
    }

    fn parent(&self, node: NodeIndex) -> Option<NodeIndex> {
        let mut parents = self.graph.neighbors_directed(node, Direction::Incoming);
        parents.next()
    }

    fn children(&self, node: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .neighbors_directed(node, Direction::Outgoing)
            .collect()
    }

    fn children_weight(&self, node: NodeIndex) -> u32 {
        self.children(node)
            .iter()
            .map(|idx| self.graph[*idx].1)
            .sum()
    }

    // return the NodeIndex of the unbalanced child, as well as the invalid_weight and expected weight
    fn children_weigh_unbalanced(&self, node: NodeIndex) -> Option<(NodeIndex, u32, u32)> {
        let children = self.children(node);
        if children.is_empty() {
            None
        } else {
            let mut weight_frequency: HashMap<u32, usize> = HashMap::new();
            for child in &children {
                let child_weight = self.graph[*child].1;
                weight_frequency
                    .entry(child_weight)
                    .and_modify(|f| *f += 1)
                    .or_insert(1);
            }

            if weight_frequency.len() == 1 {
                None
            } else {
                println!("weight_frequency {:?}", weight_frequency);
                let (expected_weight, _) = weight_frequency.iter().find(|(_w, f)| **f > 1).unwrap();
                let (invalid_weight, _) = weight_frequency.iter().find(|(_w, f)| **f == 1).unwrap();
                let mut unbalanced_node = children[0];
                for child in &children {
                    let child_weight: u32 = self.graph[*child].1;
                    if child_weight == *invalid_weight {
                        unbalanced_node = *child;
                    }
                }
                Some((unbalanced_node, *invalid_weight, *expected_weight))
            }
        }
    }

    fn update_node_value(&mut self, node: NodeIndex, weight: u32) {
        let mut node = self.graph.node_weight_mut(node).unwrap();
        node.1 = weight;
    }

    fn update_weights(&mut self, current_nodes: Vec<NodeIndex>) {
        let mut parents_to_update: HashSet<NodeIndex> = HashSet::new();
        for current_node in current_nodes {
            let children_weight: u32 = self.children_weight(current_node);
            let current_node_weight: u32 = self.graph[current_node].1;
            self.update_node_value(current_node, current_node_weight + children_weight);

            let parent = self.parent(current_node);
            if parent.is_some() {
                parents_to_update.insert(parent.unwrap());
            }
        }

        if !parents_to_update.is_empty() {
            let parents_to_update: Vec<NodeIndex> = parents_to_update.into_iter().collect();
            self.update_weights(parents_to_update);
        }
    }

    fn find_unbalanced_node(&self, current_nodes: Vec<NodeIndex>) -> Option<(NodeIndex, u32, u32)> {
        let mut parents_to_check: HashSet<NodeIndex> = HashSet::new();

        for current_node in current_nodes {
            if self.children_weigh_unbalanced(current_node).is_some() {
                println!("exit");
                return self.children_weigh_unbalanced(current_node);
            } else {
                let parent = self.parent(current_node);
                if parent.is_some() {
                    parents_to_check.insert(parent.unwrap());
                }
            }
        }

// println!("parents_to_check {:?}",parents_to_check);

        if !parents_to_check.is_empty() {
            let parents_to_check: Vec<NodeIndex> = parents_to_check.into_iter().collect();
            self.find_unbalanced_node(parents_to_check)
        } else {
            None
        }
    }
}

pub fn answer1(input: &str) -> String {
    let lines = parse_input(&input);
    let mut graph = Graph::<String, u32>::new();

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
                graph.update_edge(nodes[&line.name], nodes[child], 0); //0: no edge weight
            }
        }
    }

    let source_node = graph.externals(Direction::Incoming).next();
    graph[source_node.unwrap()].clone()
}

pub fn answer2(input: &str) -> u32 {
    let lines = parse_input(&input);
    let mut graph = Graph::<(String, u32), u32>::new();

    // add all the nodes
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut weights: HashMap<NodeIndex, u32> = HashMap::new();
    for line in &lines {
        let name = &line.name;
        let node = graph.add_node((name.clone(), line.weight));
        nodes.insert(name.clone(), node);
        weights.insert(node, line.weight);
    }

    // add all the edges
    for line in &lines {
        if line.children.is_some() {
            for child in &line.children.clone().unwrap() {
                graph.update_edge(nodes[&line.name], nodes[child], 0); //0: no edge weight
            }
        }
    }

    let mut gfr = GraphForRecursion { graph };

    // starting for the leaf nodes of the graph, update all the weights
    // a node weight will be it's weight + the sum of it's childrens
    gfr.update_weights(gfr.leafs());
    println!("graph weight updated {:?}", &gfr.graph);

    // for all nodes of the graph starting from the leaf,
    // find the first node for which the children don't have the same weight
    let unbalanced = gfr.find_unbalanced_node(gfr.leafs()).unwrap();
    println!("unbalanced {:?}", unbalanced);
    let (unbalanced_node, invalid_weight, expected_weight) =unbalanced;
    let original_weight = weights[&unbalanced_node];

    //println!("{:?}", gfr.find_unbalanced_node(gfr.leafs()));
    println!("original_weight{:?}", original_weight);

    if invalid_weight > expected_weight {
        original_weight - (invalid_weight - expected_weight)
    } else {
        original_weight + (expected_weight - invalid_weight)
    }
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

#[test]
fn test_answer2_too_heavy() {
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

    assert_eq!(answer2(&input), 60);
}

#[test]
fn test_answer2_too_light() {
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
ugml (52) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#,
    );

    assert_eq!(answer2(&input), 60);
}
