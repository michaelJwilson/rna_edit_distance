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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_node_creation() {
        let node: Node = Node::new("test".to_string());

        assert_eq!(node.name, "test");
        assert!(node.children.is_empty());
        assert!(node.states.is_empty());
    }

    #[test]
    fn test_add_child() {
        let mut parent: Node = Node::new("parent".to_string());
        let child: Node = Node::new("child".to_string());

        parent.add_child(child);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].name, "child");
    }

    #[test]
    fn test_set_state() {
        let mut node = Node::new("test".to_string());
        node.set_state("A".to_string(), 0);
        
        assert_eq!(node.states.get("A"), Some(&0));
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