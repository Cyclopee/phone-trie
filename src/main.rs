use phone_manager::models::Contact;
use phone_manager::plantuml;
use phone_manager::trie::Trie;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --release -- <fichier.json>");
        process::exit(2);
    }

    let input_path = &args[1];
    let data = fs::read_to_string(input_path)?;
    let contacts: Vec<Contact> = json5::from_str(&data)?;
    let trie = build_trie(&contacts);

    let stem = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Nom de fichier invalide")?;

    let output_path = format!("graph/{}.puml", stem);
    plantuml::generate(&trie.root, &output_path)?;
    println!("Fichier PlantUML généré : {}", output_path);
    Ok(())
}

fn build_trie(contacts: &[Contact]) -> Trie {
    let mut trie = Trie::new();
    contacts.iter().for_each(|c| {
        trie.insert(&c.nb, &c.name);
        println!("Inséré : {} ({})", c.nb, c.name);
    });
    trie
}