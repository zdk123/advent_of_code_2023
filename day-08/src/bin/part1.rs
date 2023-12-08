use std::collections::HashMap;

fn main() {
    let input = include_str!("./input8.txt");
    let output = part1(input);
    println!("{}", output);
}


// Define the node structure with a label
#[derive(Debug)]
struct Node {
    label: String,
}

// Define the edge structure with a label
#[derive(Debug)]
struct Edge {
    label: String,
}

// Define the graph structure that contains nodes and edges
#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
    edges: Vec<(String, String, Edge)>, // (source_node_label, target_node_label, edge)
}

impl Graph {
    // Create a new empty graph
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    // Add a node to the graph
    fn add_node(&mut self, label: &str) {
        let node = Node {
            label: label.to_string(),
        };
        self.nodes.insert(label.to_string(), node);
    }

    // Add an edge to the graph
    fn add_edge(&mut self, source_label: &str, target_label: &str, edge_label: &str) {
        let edge = Edge {
            label: edge_label.to_string(),
        };
        self.edges.push((source_label.to_string(), target_label.to_string(), edge));
    }

    // Traverse the graph to get the connected node label
    fn get_connected_node(&self, start_label: &str, edge_label: &str) -> Option<&String> {
        // Find the edge with the specified label starting from the given node
        if let Some((_, target_node_label, _)) = self
            .edges
            .iter()
            .find(|&&(ref source, _, ref edge)| source == start_label && edge.label == edge_label)
        {
            // Return the label of the connected node
            Some(target_node_label)
        } else {
            // No such edge found
            None
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut input_iter = input.lines();

    let instructions = input_iter.next().unwrap();

    let graph_lines: Vec<_> = input_iter.skip(1).collect();

    let mut graph = Graph::new();
    for graph_line in graph_lines {
        let (node, edges) = graph_line.split_once(" = ").unwrap();
        let sanitized_edge = edges.replace("(", "").replace(")", ""); 
        let (left_edge, right_edge) = sanitized_edge.split_once(", ").unwrap();

        graph.add_node(node);

        graph.add_node(left_edge);
        graph.add_edge(node, left_edge, "L");

        graph.add_node(right_edge);
        graph.add_edge(node, right_edge, "R");
    }

    // Traverse the graph and get the connected node label
    // println!("{:?}", instructions);
    let mut steps: u32 = 0;
    let mut node_label = "AAA";
    'outer: loop {
        for i in instructions.chars() {
            let istr = i.to_string();
            // println!("{}", istr);
            node_label = graph.get_connected_node(node_label, &istr).unwrap();
            steps = steps + 1;
            // println!("Connected Node: {}", node_label);
            if node_label == "ZZZ" {
                break 'outer
            }
        }
    
    }
    
    return steps;
}


