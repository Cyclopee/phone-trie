use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
    pub name: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
            name: None,
        }
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;
        for ch in number.chars() {
            current = current.children.entry(ch).or_insert(TrieNode::new());
        }
        current.is_end = true;
        current.name = Some(name.to_string());
    }
}