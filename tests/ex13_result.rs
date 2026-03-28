// ============================================================
// Exercice 13 — Result<T, E>
// Niveau 5: Gestion des Résultats
// ============================================================
//
// Le type Result<T, E> représente une opération qui peut réussir (Ok(T))
// ou échouer (Err(E)). C'est la base de la gestion d'erreurs en Rust.
// On utilise match, if let, unwrap(), unwrap_or(), unwrap_or_else(), etc.
//
// 💡 Indices:
//   - `Result<T, E>` : Ok(valeur) ou Err(erreur)
//   - `match result { Ok(v) => ..., Err(e) => ... }` pour pattern matching
//   - `result.unwrap()` : retourne T ou panic! si Err
//   - `result.unwrap_or(default)` : retourne T ou default si Err
//   - `result.unwrap_or_else(|e| ...)` : retourne T ou applique une closure
//   - `result.is_ok()` / `result.is_err()` : vérifications booléennes
//   - `result.map()` / `result.and_then()` : transformations enchaînées
//
// Pour lancer : cargo test --test ex13_result
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Prenez un String et divisez-le en deux moitiés.
/// Retournez (première moitié, deuxième moitié) en Ok,
/// ou une String d'erreur si la longueur est impaire.
fn split_in_half(s: &str) -> Result<(&str, &str), String> {
    todo!("Divisez la chaîne en deux moitiés ou retournez Err")
}

/// Convertissez un String en i32.
/// Utilisez Result pour capturer les erreurs de parsing.
/// Si vide, retournez Err("string vide").
fn parse_number(s: &str) -> Result<i32, String> {
    todo!("Parsez le string en i32 ou retournez une erreur")
}

/// Prenez un Result<i32, String>.
/// Utilisez unwrap_or_else pour retourner la valeur ou calculer une alternative
/// basée sur le message d'erreur (longueur du message par défaut).
fn recover_with_default(result: Result<i32, String>) -> i32 {
    todo!("Utilisez unwrap_or_else(|e| e.len() as i32)")
}

/// Enchaînez deux opérations :
/// 1. parse_number(s1) pour obtenir n1
/// 2. parse_number(s2) pour obtenir n2
/// 3. Si les deux réussissent, retournez Ok(n1 + n2)
/// Utilisez and_then pour l'enchaînement.
fn add_two_numbers(s1: &str, s2: &str) -> Result<i32, String> {
    todo!("Enchaînez deux parse_number et additionnez les résultats")
}

/// Écrivez une fonction qui retourne Err("index invalide") si l'index
/// est hors limites, sinon Ok(&arr[i]).
fn safe_index<T>(arr: &[T], i: usize) -> Result<&T, String> {
    todo!("Vérifiez l'index et retournez Ok ou Err")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_in_half_success() {
        let result = split_in_half("hello");
        assert!(result.is_ok());
        let (left, right) = result.unwrap();
        assert_eq!(left, "he");
        assert_eq!(right, "llo");
    }

    #[test]
    fn test_split_in_half_odd_length() {
        let result = split_in_half("odd");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Longueur impaire");
    }

    #[test]
    fn test_parse_number_valid() {
        let result = parse_number("42");
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_parse_number_invalid() {
        let result = parse_number("not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_number_empty() {
        let result = parse_number("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "string vide");
    }

    #[test]
    fn test_recover_with_default_ok() {
        let result = Ok(100);
        assert_eq!(recover_with_default(result), 100);
    }

    #[test]
    fn test_recover_with_default_err() {
        let result: Result<i32, String> = Err("error msg".to_string());
        assert_eq!(recover_with_default(result), 9); // "error msg" a 9 caractères
    }

    #[test]
    fn test_add_two_numbers_success() {
        let result = add_two_numbers("10", "20");
        assert_eq!(result.unwrap(), 30);
    }

    #[test]
    fn test_add_two_numbers_first_invalid() {
        let result = add_two_numbers("abc", "20");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_two_numbers_second_invalid() {
        let result = add_two_numbers("10", "xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_index_valid() {
        let arr = [1, 2, 3, 4, 5];
        let result = safe_index(&arr, 2);
        assert_eq!(*result.unwrap(), 3);
    }

    #[test]
    fn test_safe_index_out_of_bounds() {
        let arr = [1, 2, 3];
        let result = safe_index(&arr, 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "index invalide");
    }
}
