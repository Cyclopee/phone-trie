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
// TODO: insert
}
