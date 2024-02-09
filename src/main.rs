use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
    states: HashMap<String, u32>,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name,
            children: Vec::new(),
            states: HashMap::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn set_state(&mut self, state: String, value: u32) {
        self.states.insert(state, value);
    }
}

fn main() {
    let root = Node::new("root".to_string());

    println!("Initialised root!");
    println!("{:?}", root);
}