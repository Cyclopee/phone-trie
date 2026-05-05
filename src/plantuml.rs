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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::Trie;

    #[test]
    fn build_empty_trie() {
        let trie = Trie::new();
        let output = build(&trie.root);
        assert_eq!(output, "@startmindmap\n@endmindmap\n");
    }

    #[test]
    fn build_short_number() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");

        let expected = "\
@startmindmap
* 1
** 5
*** SAMU
@endmindmap
";
        assert_eq!(build(&trie.root), expected);
    }

    #[test]
    fn build_two_different_roots() {
        let mut trie = Trie::new();
        trie.insert("12", "Urgences");
        trie.insert("15", "SAMU");

        let expected = "\
@startmindmap
* 1
** 2
*** Urgences
** 5
*** SAMU
@endmindmap
";
        assert_eq!(build(&trie.root), expected);
    }

    #[test]
    fn build_is_deterministic() {
        let mut a = Trie::new();
        a.insert("12", "X");
        a.insert("15", "Y");

        let mut b = Trie::new();
        b.insert("15", "Y");
        b.insert("12", "X");

        assert_eq!(build(&a.root), build(&b.root));
    }
}