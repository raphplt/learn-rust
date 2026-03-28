# Rust par la Pratique 🦀

Apprendre Rust par la pratique : **25 exercices** progressifs, des bases jusqu'aux projets systèmes.

## Prérequis

- [Rust](https://rustup.rs/) (stable)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Démarrage rapide

```bash
git clone <url-du-repo>
cd learn-rust

# Lancer un exercice
cargo test --test ex01_variables

# Voir ta progression
./check.sh
```

## Structure

```
tests/
├── ex01_variables.rs      Niveau 1 — Variables & Mutabilité
├── ex02_types.rs           Niveau 1 — Types Primitifs
├── ex03_fonctions.rs       Niveau 1 — Fonctions
├── ex04_controle.rs        Niveau 1 — Contrôle de Flux
├── ex05_ownership.rs       Niveau 2 — Ownership
├── ex06_references.rs      Niveau 2 — Références & Borrowing
├── ex07_slices.rs          Niveau 2 — Slices
├── ex08_structs.rs         Niveau 3 — Structs
├── ex09_enums.rs           Niveau 3 — Enums & Pattern Matching
├── ex10_option.rs          Niveau 3 — Option<T>
├── ex11_traits.rs          Niveau 4 — Traits
├── ex12_generics.rs        Niveau 4 — Génériques & Trait Bounds
├── ex13_result.rs          Niveau 5 — Result<T, E>
├── ex14_erreurs.rs         Niveau 5 — Gestion d'Erreurs
├── ex15_vecteurs.rs        Niveau 6 — Vec & HashMap
├── ex16_iterateurs.rs      Niveau 6 — Itérateurs
├── ex17_closures.rs        Niveau 6 — Closures
├── ex18_lifetimes.rs       Niveau 7 — Lifetimes
├── ex19_box.rs             Niveau 8 — Box & Smart Pointers
├── ex20_rc_arc.rs          Niveau 8 — Rc, Arc & RefCell
├── ex21_threads.rs         Niveau 9 — Threads
├── ex22_channels.rs        Niveau 9 — Channels & Mutex
├── ex23_kvstore.rs         Niveau 10 — Mini Key-Value Store
├── ex24_parser.rs          Niveau 10 — Parser de Commandes
└── ex25_serialisation.rs   Niveau 10 — Sérialisation
```

## Comment ça marche

Chaque fichier dans `tests/` contient des fonctions avec `todo!()` que tu dois implémenter. Les tests intégrés vérifient tes réponses.

```rust
// Exemple : tu trouves ça dans le fichier
fn add(a: i32, b: i32) -> i32 {
    todo!("Remplace-moi !")
}

// Tu remplaces par :
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Puis tu lances :

```bash
cargo test --test ex01_variables
```

## Guide de référence

Ouvre `guide.html` dans ton navigateur pour un guide complet avec la théorie essentielle, des exemples de code, les pièges courants, et un aide-mémoire.

## Progression

Le script `check.sh` affiche ta progression sur l'ensemble des exercices :

```bash
./check.sh
```

```
🦀 Rust par la Pratique — Vérification des exercices
═══════════════════════════════════════════════════════

── Niveau 1 ──
  ✅ ex01_variables — Variables & Mutabilité
  ⏳ ex02_types — Types Primitifs (pas encore commencé)
  ...
```

## Les 10 niveaux

| Niveau | Thème | Exercices |
|--------|-------|-----------|
| 1 | Variables, Types, Fonctions, Contrôle de flux | ex01 → ex04 |
| 2 | Ownership, Références, Slices | ex05 → ex07 |
| 3 | Structs, Enums, Option\<T\> | ex08 → ex10 |
| 4 | Traits, Génériques | ex11 → ex12 |
| 5 | Result\<T, E\>, Gestion d'erreurs | ex13 → ex14 |
| 6 | Collections, Itérateurs, Closures | ex15 → ex17 |
| 7 | Lifetimes | ex18 |
| 8 | Box, Rc, Arc, RefCell | ex19 → ex20 |
| 9 | Threads, Channels, Mutex | ex21 → ex22 |
| 10 | Mini KV Store, Parser, Sérialisation | ex23 → ex25 |

## Licence

MIT
