use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: HashMap<u8, Node>
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
    length: usize,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, text: &str) {
        let mut current_node = &mut self.root;

        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default()
        }
        current_node.at_end = true;
        self.length += 1;
    }

    fn contains(&self, text: &str) -> bool {
        let mut current_node = &self.root;

        for c in text.bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.at_end
    }

    fn len(&self) -> usize {
        self.length
    }
}

fn main() {
    let mut urls = Trie::new();
    urls.insert("https://www.amazon.com");
    urls.insert("https://www.reddit.com/r/rust");

    let contains_stub = urls.contains("https://www");
    println!("{contains_stub}");
    let len = urls.len();
    println!("{len}");
}
