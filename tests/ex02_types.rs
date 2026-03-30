// ============================================================
// Exercice 02 — Types Primitifs
// Niveau 1: Concepts Fondamentaux
// ============================================================
//
// Rust a plusieurs types primitifs: les entiers (i32, i64, u32, etc.),
// les flottants (f32, f64), les booléens, les caractères, et les collections
// (tuples, arrays). Les chaînes peuvent être &str (slice) ou String (allouée).
// Le type doit souvent être explicite ou déductible du contexte.
//
// 💡 Indices:
//   - `let x: i32 = 42;` déclare un entier signé 32-bit
//   - `let t: (i32, &str) = (5, "hello");` crée un tuple
//   - `let arr: [i32; 3] = [1, 2, 3];` crée un array de taille fixe
//   - `let s = "hello".to_string();` convertit &str en String
//
// Pour lancer : cargo test --test ex02_types
// ============================================================

#[allow(dead_code)]
// ---- Fonctions à compléter ----

/// Créez un tuple contenant un entier, un booléen et une chaîne.
/// Retournez la longueur de la chaîne du tuple.
fn tuple_demo() -> usize {
    let t: (i32, bool, &str) = (42, true, "Rusta");
    t.2.len()
}

/// Créez un array de 5 entiers [10, 20, 30, 40, 50].
/// Retournez la somme des éléments.
fn array_sum() -> i32 {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    a.iter().sum()
}

/// Convertissez la chaîne "Rust" en String, puis retournez sa longueur en octets.
fn string_conversion() -> usize {
    "Rust".to_string().len()
}

/// Créez deux nombres flottants 3.5 et 2.0, retournez leur multiplication.
fn float_multiplication() -> f64 {
    let float_1: f64 = 3.5;
    let float_2: f64 = 2.0;
    float_1 * float_2
}

/// Créez un array de caractères ['a', 'b', 'c', 'd'].
/// Retournez le premier et le dernier comme un tuple.
fn char_array() -> (char, char) {
    let array: [char; 4] = ['a', 'b', 'c', 'd'];
    (array[0], array[3])
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_demo() {
        assert_eq!(tuple_demo(), 5);
    }

    #[test]
    fn test_array_sum() {
        assert_eq!(array_sum(), 150);
    }

    #[test]
    fn test_string_conversion() {
        assert_eq!(string_conversion(), 4);
    }

    #[test]
    fn test_float_multiplication() {
        assert!((float_multiplication() - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_char_array() {
        let (first, last) = char_array();
        assert_eq!(first, 'a');
        assert_eq!(last, 'd');
    }
}
