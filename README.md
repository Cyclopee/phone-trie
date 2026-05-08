# Phone Trie
Gestionnaire de numéros de téléphone basé sur une structure de données
**Trie** (prefix tree), écrit en Rust.
## Description
Le programme prend en entrée un fichier JSON contenant des contacts (numéro +
nom),
construit un Trie en mémoire pour stocker efficacement les numéros (en
partageant
les préfixes communs), puis génère un fichier PlantUML au format **MindMap**
permettant de visualiser la structure.
## Prérequis
- [Rust](https://www.rust-lang.org/tools/install) (édition 2021)
## Lancer le programme
```bash
cargo run --release -- data/04_common_parts.json
```
Le fichier PlantUML sera généré dans le dossier `graph/`.
Vous pouvez tester avec n'importe lequel des fichiers fournis :
```bash
cargo run --release -- data/01_simple.json
cargo run --release -- data/02_different_roots.json
cargo run --release -- data/03_one_in_another.json
cargo run --release -- data/04_common_parts.json
```
## Lancer les tests
```bash
cargo test
```
## Vérifier la qualité du code
```bash
cargo clippy --all-targets -- -D warnings
```
## Visualiser le graphe PlantUML
1. Installer [Docker](https://docs.docker.com/get-docker/)
2. Lancer un serveur PlantUML local :
```bash
docker pull plantuml/plantuml-server:jetty
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```
3. Ouvrir <http://localhost:8080> dans un navigateur
4. Coller le contenu d'un fichier `.puml` (ex: `graph/04_common_parts.puml`)
## Structure du projet
```
phone-trie/
├── Cargo.toml
├── README.md
├── src/
│ ├── main.rs point d'entrée, gestion des erreurs avec Result
│ ├── lib.rs déclaration des modules + #![forbid(unsafe_code)]
│ ├── trie.rs implémentation from scratch du Trie
│ ├── models.rs struct Contact + désérialisation JSON
│ └── plantuml.rs génération du fichier PlantUML MindMap
├── data/ fichiers JSON de test (fournis, non modifiés)
└── graph/ fichiers PlantUML générés
```
## Choix techniques
- **Trie implémenté from scratch** avec `HashMap<char, TrieNode>`,
conformément
à la contrainte du sujet (interdiction d'utiliser un crate pour le Trie).
- **`#![forbid(unsafe_code)]`** dans `lib.rs` : aucun bloc `unsafe` n'est
autorisé.
- **Gestion des erreurs avec `Result`** : `main` retourne `Result<(), Box<dyn
Error>>`
et utilise l'opérateur `?` pour propaguer les erreurs proprement.
- **Iterators** : `contacts.iter().for_each(...)` utilisé dans `build_trie` à
la place
d'une boucle `for` classique.
- **`json5` au lieu de `serde_json`** : les fichiers de test contiennent des
virgules
finales que `serde_json` refuse mais que `json5` accepte.
- **Format PlantUML conforme au sujet** : pas de noeud `* root` artificiel,
le nom
est écrit comme un noeud enfant supplémentaire du dernier chiffre.
## Membres du groupe
- Adam BARDON
- Eliott RIVALETTO
- Lyvann NABOR
- Matteo CUTAIA