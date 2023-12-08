use std::collections::HashMap;
use num_integer::lcm;

fn main() {
    let input = include_str!("./input8.txt");
    let output = part2(input);
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

fn minimum_traversal(start_label: &str, instructions: &str, graph: &Graph) -> u64 {
    let mut steps: u64 = 0;
    let mut node_label = start_label.clone();
    'outer: loop {
        for i in instructions.chars() {
            let istr = i.to_string();
            node_label = graph.get_connected_node(node_label, &istr).unwrap();
            steps = steps + 1;
            if node_label.ends_with('Z') {
                break 'outer
            }
        }
    }
    return steps
}


fn part2(input: &str) -> u64 {
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

    let start_labels: Vec<_> = graph.nodes.keys().filter(|&s| s.ends_with('A')).collect();
    //let start_labels = &start_labels[..2].to_vec();
    let n_runs: usize = start_labels.len(); // number of simultaneous traversals

    let mut steps: u64 = 0;

    let mut node_labels: Vec<_> = start_labels.clone();
    let mut all_steps = vec![0u64; n_runs];
    for i in 0..n_runs {
        all_steps[i] = minimum_traversal(&node_labels[i], &instructions, &graph);
    }
    
    // Least common multiple from all the solutions
    let result = all_steps.iter().fold(1, |acc, &x| lcm(acc, x));
    return  result;
}


