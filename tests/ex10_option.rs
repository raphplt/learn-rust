// ============================================================
// Exercice 10 — Option<T>
// Niveau 3: Gestion des Valeurs Optionnelles
// ============================================================
//
// Option<T> représente une valeur qui peut exister (Some(T)) ou non (None).
// C'est l'alternative sûre à null. Les méthodes .unwrap(), .unwrap_or(),
// .map(), .and_then(), et le pattern matching permettent de manipuler Option.
// Rust force à traiter le cas None explicitement.
//
// 💡 Indices:
//   - `Some(5)` crée une Option avec une valeur
//   - `None` crée une Option vide
//   - `.unwrap()` extrait la valeur (panic si None)
//   - `.unwrap_or(default)` retourne default si None
//   - `.map(f)` applique f si Some
//   - `.and_then(f)` enchaîne les opérations Option
//
// Pour lancer : cargo test --test ex10_option
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une fonction qui prend Option<i32> et retourne la valeur
/// ou 0 si None.
fn unwrap_or_zero(opt: Option<i32>) -> i32 {
    todo!("Retourne la valeur ou 0 si None")
}

/// Écrivez une fonction qui prend Option<&str> et retourne sa longueur
/// en majuscules (String), ou "EMPTY" si None.
fn length_or_empty(opt: Option<&str>) -> String {
    todo!("Retourne la longueur en majuscules ou \"EMPTY\"")
}

/// Écrivez une fonction qui prend deux Option<i32> et retourne
/// leur somme si les deux sont Some, sinon None.
fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!("Retourne Some(a+b) si les deux sont Some, sinon None")
}

/// Écrivez une fonction qui divise deux nombres (Option<f64>).
/// Retournez Some(a/b) si b != 0, sinon None.
fn safe_divide(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    todo!("Retourne Some(a/b) si b != 0 et les deux sont Some, sinon None")
}

/// Écrivez une fonction qui trouve le premier nombre pair dans une slice,
/// ou retourne None.
fn find_even(numbers: &[i32]) -> Option<i32> {
    todo!("Retourne Some du premier nombre pair ou None")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_or_zero() {
        assert_eq!(unwrap_or_zero(Some(42)), 42);
        assert_eq!(unwrap_or_zero(None), 0);
    }

    #[test]
    fn test_length_or_empty() {
        assert_eq!(length_or_empty(Some("Rust")), "4");
        assert_eq!(length_or_empty(None), "EMPTY");
    }

    #[test]
    fn test_add_options() {
        assert_eq!(add_options(Some(5), Some(3)), Some(8));
        assert_eq!(add_options(Some(5), None), None);
        assert_eq!(add_options(None, Some(3)), None);
        assert_eq!(add_options(None, None), None);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(Some(10.0), Some(2.0)), Some(5.0));
        assert_eq!(safe_divide(Some(10.0), Some(0.0)), None);
        assert_eq!(safe_divide(Some(10.0), None), None);
        assert_eq!(safe_divide(None, Some(2.0)), None);
    }

    #[test]
    fn test_find_even() {
        assert_eq!(find_even(&[1, 3, 4, 5, 6]), Some(4));
        assert_eq!(find_even(&[1, 3, 5]), None);
        assert_eq!(find_even(&[]), None);
    }
}
