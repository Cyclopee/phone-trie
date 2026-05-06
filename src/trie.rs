use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct TrieNode {
pub children: HashMap<char, TrieNode>,
pub is_end: bool,
pub name: Option<String>,
}
impl TrieNode {
pub fn new() -> Self {
TrieNode::default()
}
}
#[derive(Debug, Default)]
pub struct Trie {
pub root: TrieNode,
}
impl Trie {
pub fn new() -> Self {
Trie::default()
}
pub fn insert(&mut self, number: &str, name: &str) {
let mut current = &mut self.root;
for ch in number.chars() {
current = current.children.entry(ch).or_default();
}
current.is_end = true;
current.name = Some(name.to_string());
}
}
#[cfg(test)]
mod tests {
use super::*;
#[test]
fn new_trie_is_empty() {
let trie = Trie::new();
assert!(trie.root.children.is_empty());
assert!(!trie.root.is_end);
}
#[test]
fn insert_one_number() {
let mut trie = Trie::new();
trie.insert("15", "SAMU");
let n1 = trie.root.children.get(&'1').unwrap();
let n5 = n1.children.get(&'5').unwrap();
assert!(n5.is_end);
assert_eq!(n5.name.as_deref(), Some("SAMU"));
}
#[test]
fn insert_shares_common_prefix() {
let mut trie = Trie::new();
trie.insert("0412", "Alice");
trie.insert("0468", "Bob");
let n0 = trie.root.children.get(&'0').unwrap();
let n4 = n0.children.get(&'4').unwrap();
assert_eq!(n4.children.len(), 2);
}
#[test]
fn insert_number_prefix_of_another() {
let mut trie = Trie::new();
trie.insert("0123456789", "Alice");
trie.insert("0123", "Bob");
let n3 = trie.root
.children.get(&'0').unwrap()
.children.get(&'1').unwrap()
.children.get(&'2').unwrap()
.children.get(&'3').unwrap();
assert!(n3.is_end);
assert_eq!(n3.name.as_deref(), Some("Bob"));
assert!(!n3.children.is_empty());
}
#[test]
fn insert_overwrites_existing_name() {
let mut trie = Trie::new();
trie.insert("15", "SAMU");
trie.insert("15", "Pompiers");
let n5 = trie.root
.children.get(&'1').unwrap()
.children.get(&'5').unwrap();
assert_eq!(n5.name.as_deref(), Some("Pompiers"));
}
}