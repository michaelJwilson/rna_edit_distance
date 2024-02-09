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

    // NB no return syntax here.
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    // NB no return syntax here.
    fn set_state(&mut self, state: String, value: u32) {
        self.states.insert(state, value);
    }
}

fn main() {
    let root = Node::new("root".to_string());

    println!("Initialised root!");
    println!("{:?}", root);

    let mut states: HashMap<&str, Result<i32, &str>>= HashMap::new();

    states.insert("a",Ok(1));
    states.insert("b", Ok(2));

    let new_score: &Result<i32, &str> = states.get("b").unwrap();

    println!("{:?}", new_score);
}