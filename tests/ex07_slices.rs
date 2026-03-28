// ============================================================
// Exercice 07 — Slices
// Niveau 2: String & Array Slices
// ============================================================
//
// Une slice est une référence à une portion contiguë d'une collection.
// &str est une slice de String (references immuables).
// &[T] est une slice d'un array ou Vec.
// Les slices permettent de passer des sous-portions sans copier.
//
// 💡 Indices:
//   - `let slice = &s[0..3];` prend caractères 0, 1, 2 (3 exclu)
//   - `let slice = &arr[2..];` prend du 2 à la fin
//   - `let slice = &arr[..3];` prend du début à l'index 2
//   - `s.len()` retourne le nombre d'octets (pas de chars Unicode)
//
// Pour lancer : cargo test --test ex07_slices
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une fonction qui prend une slice &[i32] et retourne la somme.
fn sum_slice(slice: &[i32]) -> i32 {
    todo!("Retourne la somme des éléments de la slice")
}

/// Écrivez une fonction qui prend un &str et retourne les 3 premiers caractères.
/// Si la chaîne est plus courte, retournez la chaîne entière.
fn first_three_chars(s: &str) -> &str {
    todo!("Retourne les 3 premiers caractères ou la chaîne entière")
}

/// Écrivez une fonction qui prend une slice &[i32] et retourne le maximum.
/// Assume que la slice n'est pas vide.
fn max_in_slice(slice: &[i32]) -> i32 {
    todo!("Retourne le maximum de la slice")
}

/// Écrivez une fonction qui retourne une slice contenant les éléments
/// pairs d'un array. Créez un array de test et slicez-le.
fn slice_even_numbers() -> Vec<i32> {
    todo!("Crée un array, identifie les nombres pairs et retourne un Vec")
}

/// Écrivez une fonction qui prend un &str et retourne le nombre de mots
/// (séparés par des espaces).
fn count_words(s: &str) -> usize {
    todo!("Compte le nombre de mots dans s (séparés par espace)")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_slice() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_slice(&arr[..]), 15);
        assert_eq!(sum_slice(&arr[1..4]), 9); // 2+3+4
    }

    #[test]
    fn test_first_three_chars() {
        assert_eq!(first_three_chars("Rust"), "Rus");
        assert_eq!(first_three_chars("Go"), "Go");
    }

    #[test]
    fn test_max_in_slice() {
        let arr = [3, 7, 2, 9, 1];
        assert_eq!(max_in_slice(&arr[..]), 9);
    }

    #[test]
    fn test_slice_even_numbers() {
        let result = slice_even_numbers();
        assert!(result.iter().all(|&x| x % 2 == 0));
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello world Rust"), 3);
        assert_eq!(count_words("OneWord"), 1);
        assert_eq!(count_words(""), 0);
    }
}
