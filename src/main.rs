use phone_manager::models::Contact;
use phone_manager::trie::Trie;
use phone_manager::plantuml;
use std::fs;
use std::path::Path;
fn main() {
let args: Vec<String> = std::env::args().collect();
if args.len() < 2 {
eprintln!("Usage: cargo run -- <fichier.json>");
return;
}
let input_path = &args[1];
let data = fs::read_to_string(input_path)
.expect("Impossible de lire le fichier JSON");
let contacts: Vec<Contact> = json5::from_str(&data)
.expect("Erreur de désérialisation JSON");
let mut trie = Trie::new();
for contact in &contacts {
trie.insert(&contact.nb, &contact.name);
println!("Inséré : {} ({})", contact.nb, contact.name);
}
let stem = Path::new(input_path)
.file_stem()
.unwrap()
.to_str()
.unwrap();
let output_path = format!("graph/{}.puml", stem);
plantuml::generate(&trie.root, &output_path);
}