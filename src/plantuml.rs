use crate::trie::TrieNode;
use std::fs;
pub fn generate(root: &TrieNode, output_path: &str) {
let mut content = String::from("@startmindmap\n* root\n");
write_node(root, 1, &mut content);
content.push_str("@endmindmap\n");
fs::write(output_path, content)
.expect("Impossible d'écrire le fichier PlantUML");
println!("Fichier PlantUML généré : {}", output_path);
}
fn write_node(node: &TrieNode, depth: usize, content: &mut String) {
let mut children: Vec<(&char, &TrieNode)> = node.children.iter().collect();
children.sort_by_key(|(ch, _)| *ch);
for (ch, child) in children {
let stars = "*".repeat(depth + 1);
if child.is_end {
let name = child.name.as_deref().unwrap_or("");
content.push_str(&format!("{} {} ({})\n", stars, ch, name));
} else {
content.push_str(&format!("{} {}\n", stars, ch));
}
write_node(child, depth + 1, content);
}
}