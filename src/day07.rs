use nom::types::CompleteStr;
use nom::{
    char, delimited, do_parse, is_not, map, map_res, named, opt, separated_nonempty_list, tag,
};
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};
use std::collections::{HashMap, HashSet};

pub fn title() -> &'static str {
    "Day 07: Recursive Circus"
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

// (String, u32, u32) -> (name, weight, children weight)
#[derive(Debug, Clone, PartialEq)]
struct Node(String, u32, u32);

#[derive(Debug)]
struct Towers {
    graph: Graph<Node, u32>,
}

impl Towers {
    fn new(lines: &[Line]) -> Towers {
        let mut graph = Graph::<Node, u32>::new();

        // add all the nodes
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
        for line in lines {
            let name = &line.name;
            let node = graph.add_node(Node(name.clone(), line.weight, 0));
            nodes.insert(name.clone(), node);
        }

        // add all the edges
        for line in lines {
            if line.children.is_some() {
                for child in &line.children.clone().unwrap() {
                    graph.update_edge(nodes[&line.name], nodes[child], 0); //0: no edge weight
                }
            }
        }

        Towers { graph }
    }

    fn get(&self, idx: NodeIndex) -> Node {
        self.graph[idx].clone()
    }

    fn leafs(&self) -> Vec<NodeIndex> {
        self.graph.externals(Direction::Outgoing).collect()
    }

    fn source(&self) -> NodeIndex {
        self.graph.externals(Direction::Incoming).next().unwrap()
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
            .map(|idx| {
                let child = self.get(*idx);
                child.1 + child.2
            })
            .sum()
    }

    fn update_node_value(&mut self, node: NodeIndex, weight: u32) {
        let mut node = self.graph.node_weight_mut(node).unwrap();
        node.2 = weight;
    }

    fn compute_children_weights(&mut self, current_nodes: Vec<NodeIndex>) {
        let mut parents_to_update: HashSet<NodeIndex> = HashSet::new();
        for current_node in current_nodes {
            let children_weight: u32 = self.children_weight(current_node);
            self.update_node_value(current_node, children_weight);

            let parent = self.parent(current_node);
            if parent.is_some() {
                parents_to_update.insert(parent.unwrap());
            }
        }

        if !parents_to_update.is_empty() {
            let parents_to_update: Vec<NodeIndex> = parents_to_update.into_iter().collect();
            self.compute_children_weights(parents_to_update);
        }
    }

    fn find_unbalanced_node(&self, current_node: NodeIndex) -> Option<(NodeIndex, u32)> {
        // finding the unbalanced side
        let mut children_weight_frequency: HashMap<u32, usize> = HashMap::new();
        let mut children_weight_per_node: HashMap<u32, Vec<NodeIndex>> = HashMap::new();
        for child in &self.children(current_node) {
            let child_weight = self.get(*child);
            let child_weight = child_weight.1 + child_weight.2;
            children_weight_frequency
                .entry(child_weight)
                .and_modify(|f| *f += 1)
                .or_insert(1);
            children_weight_per_node
                .entry(child_weight)
                .and_modify(|l| l.push(*child))
                .or_insert_with(|| vec![*child]);
        }

        let unbalanced_side: u32 = children_weight_frequency
            .iter()
            .find(|(_, f)| **f == 1)
            .map(|(w, _)| *w)
            .unwrap();
        let correct_side: u32 = children_weight_frequency
            .iter()
            .find(|(_, f)| **f > 1)
            .map(|(w, _)| *w)
            .unwrap();
        let weight_diff: i32 = unbalanced_side as i32 - correct_side as i32;

        let unbalanced_side: NodeIndex = children_weight_per_node[&unbalanced_side][0];

        // looking at the children of the unbalanced side
        let mut children_weight_frequency: HashMap<u32, usize> = HashMap::new();
        for child in &self.children(unbalanced_side) {
            let child_weight = self.get(*child);
            let child_weight = child_weight.1 + child_weight.2;
            children_weight_frequency
                .entry(child_weight)
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
        if children_weight_frequency.len() == 1 {
            // children balanced, so the current node is the unbalanced one
            let new_weight = (self.get(unbalanced_side).1 as i32 - weight_diff) as u32;
            Some((unbalanced_side, new_weight))
        } else {
            self.find_unbalanced_node(unbalanced_side)
        }
    }
}

pub fn answer1(input: &str) -> String {
    let lines = parse_input(&input);
    let towers = Towers::new(&lines);

    towers.get(towers.source()).0
}

pub fn answer2(input: &str) -> u32 {
    let lines = parse_input(&input);
    let mut towers = Towers::new(&lines);

    // starting for the leaf nodes of the graph, update all the weights
    // a node weight will be it's weight + the sum of it's childrens
    towers.compute_children_weights(towers.leafs());

    // for all nodes of the graph starting from the leaf,
    // find the first node for which the children don't have the same weight
    towers.find_unbalanced_node(towers.source()).unwrap().1
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
fn test_compute_children_weight() {
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
    let lines = parse_input(&input);
    let mut towers = Towers::new(&lines);

    towers.compute_children_weights(towers.leafs());

    assert_eq!(
        towers.get(towers.leafs()[0]),
        Node("pbga".to_string(), 66, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[1]),
        Node("xhth".to_string(), 57, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[2]),
        Node("ebii".to_string(), 61, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[3]),
        Node("havc".to_string(), 66, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[4]),
        Node("ktlj".to_string(), 57, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[5]),
        Node("qoyq".to_string(), 66, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[6]),
        Node("jptl".to_string(), 61, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[7]),
        Node("gyxo".to_string(), 61, 0)
    );
    assert_eq!(
        towers.get(towers.leafs()[8]),
        Node("cntj".to_string(), 57, 0)
    );

    assert_eq!(
        towers.get(towers.source()),
        Node("tknk".to_string(), 41, 737)
    );
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
