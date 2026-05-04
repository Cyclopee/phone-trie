use crate::trie::TrieNode;
use std::fs;

pub fn generate(root: &TrieNode, output_path: &str) {
    let content = build(root);

    fs::write(output_path, content)
        .expect("Impossible d'écrire le fichier PlantUML");

    println!("Fichier PlantUML généré : {}", output_path);
}


pub fn build(root: &TrieNode) -> String {
    let mut content = String::from("@startmindmap\n");
    write_children(root, 1, &mut content);
    content.push_str("@endmindmap\n");
    content
}

fn write_children(node: &TrieNode, depth: usize, content: &mut String) {
    let mut children: Vec<(&char, &TrieNode)> = node.children.iter().collect();
    children.sort_by_key(|(ch, _)| *ch);

    let stars = "*".repeat(depth);
    for (ch, child) in children {
        content.push_str(&format!("{} {}\n", stars, ch));
        write_children(child, depth + 1, content);

      
        if child.is_end {
            if let Some(name) = &child.name {
                let leaf_stars = "*".repeat(depth + 1);
                content.push_str(&format!("{} {}\n", leaf_stars, name));
            }
        }
    }
}