// ============================================================
// Exercice 03 — Fonctions
// Niveau 1: Concepts Fondamentaux
// ============================================================
//
// Les fonctions sont définies avec `fn`. Elles peuvent avoir des paramètres
// et un type de retour. En Rust, la dernière expression d'une fonction
// (sans `;`) est sa valeur de retour. Les statements se terminent par `;`,
// les expressions non.
//
// 💡 Indices:
//   - `fn add(a: i32, b: i32) -> i32 { a + b }` retourne la somme
//   - Les paramètres doivent avoir un type explicite
//   - `fn greet() -> &'static str { "Hello!" }` retourne une chaîne statique
//   - Les accolades `{}` créent un scope
//
// Pour lancer : cargo test --test ex03_fonctions
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une fonction qui prend deux i32 et retourne leur somme.
fn add(a: i32, b: i32) -> i32 {
    todo!("Retourne a + b")
}

/// Écrivez une fonction qui prend un &str et retourne sa longueur.
fn string_length(s: &str) -> usize {
    todo!("Retourne la longueur de s")
}

/// Écrivez une fonction qui prend un nombre et retourne true si c'est pair,
/// false sinon.
fn is_even(n: i32) -> bool {
    todo!("Retourne true si n est pair")
}

/// Écrivez une fonction qui prend trois nombres et retourne leur moyenne (arrondie).
fn average(a: f64, b: f64, c: f64) -> f64 {
    todo!("Retourne la moyenne de a, b, c")
}

/// Écrivez une fonction sans paramètres qui retourne la constante π (approximation 3.14159).
fn pi() -> f64 {
    todo!("Retourne π")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(-2, 7), 5);
    }

    #[test]
    fn test_string_length() {
        assert_eq!(string_length("Rust"), 4);
        assert_eq!(string_length(""), 0);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
    }

    #[test]
    fn test_average() {
        let result = average(2.0, 4.0, 6.0);
        assert!((result - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_pi() {
        let result = pi();
        assert!((result - 3.14159).abs() < 0.001);
    }
}
