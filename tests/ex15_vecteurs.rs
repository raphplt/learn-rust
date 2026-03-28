// ============================================================
// Exercice 15 — Vec & HashMap
// Niveau 6: Collections
// ============================================================
//
// Vec<T> est un vecteur dynamique (tableau redimensionnable).
// HashMap<K, V> stocke des paires clé-valeur.
// On utilise push, pop, iter, HashMap::new, insert, get, etc.
// L'entry API permet des opérations efficaces sur les entrées.
//
// 💡 Indices:
//   - `let mut v = Vec::new()` ou `vec![1, 2, 3]`
//   - `v.push(x)`, `v.pop()`, `v.len()`, `v.iter()`, `v.contains(&x)`
//   - `let mut map = HashMap::new()`, `map.insert(k, v)`, `map.get(&k)`
//   - `map.entry(k).or_insert(v)` : insérez si absent
//   - `map.iter()` : itérez sur les paires (clé, valeur)
//   - Pattern de comptage : `*counter.entry(x).or_insert(0) += 1`
//
// Pour lancer : cargo test --test ex15_vecteurs
// ============================================================

#[allow(dead_code)]

use std::collections::HashMap;

// ---- Fonctions à compléter ----

/// Retournez le nombre d'occurrences de chaque élément dans un vecteur.
fn count_occurrences(v: &[i32]) -> HashMap<i32, usize> {
    todo!("Comptez chaque élément avec entry API")
}

/// Retournez le double de chaque élément d'un vecteur dans un nouveau vecteur.
fn double_vec(v: &[i32]) -> Vec<i32> {
    todo!("Retournez un nouveau Vec avec chaque élément doublé")
}

/// Filtrez un vecteur et conservez seulement les nombres pairs.
fn keep_even(v: &[i32]) -> Vec<i32> {
    todo!("Retournez un Vec contenant seulement les nombres pairs")
}

/// Fusionnez deux vecteurs en un seul, sans doublons.
/// Utilisez un HashSet ou vérifiez contains().
fn merge_without_duplicates(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    todo!("Fusionnez v1 et v2 en gardant les éléments uniques")
}

/// Inversez l'ordre des éléments d'un vecteur.
fn reverse_vec(v: &[i32]) -> Vec<i32> {
    todo!("Retournez une copie inversée de v")
}

/// Créez une HashMap où les clés sont les mots et les valeurs sont leurs longueurs.
fn word_lengths<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    todo!("Retournez une map mot => longueur")
}

/// Comptez la fréquence de chaque caractère dans une chaîne.
fn char_frequency(s: &str) -> HashMap<char, usize> {
    todo!("Retournez une map char => fréquence")
}

/// Retournez la somme de tous les éléments d'un vecteur.
fn sum_vec(v: &[i32]) -> i32 {
    todo!("Retournez la somme de tous les éléments")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurrences() {
        let v = vec![1, 2, 1, 3, 2, 1];
        let result = count_occurrences(&v);
        assert_eq!(result[&1], 3);
        assert_eq!(result[&2], 2);
        assert_eq!(result[&3], 1);
    }

    #[test]
    fn test_count_occurrences_empty() {
        let v: Vec<i32> = vec![];
        let result = count_occurrences(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_double_vec() {
        let v = vec![1, 2, 3, 4];
        let result = double_vec(&v);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_double_vec_empty() {
        let v: Vec<i32> = vec![];
        let result = double_vec(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_keep_even() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result = keep_even(&v);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_keep_even_no_evens() {
        let v = vec![1, 3, 5];
        let result = keep_even(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_merge_without_duplicates() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let result = merge_without_duplicates(&v1, &v2);
        assert_eq!(result.len(), 4); // 1, 2, 3, 4
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
    }

    #[test]
    fn test_merge_without_duplicates_empty() {
        let v1: Vec<i32> = vec![];
        let v2 = vec![1, 2];
        let result = merge_without_duplicates(&v1, &v2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_reverse_vec() {
        let v = vec![1, 2, 3, 4];
        let result = reverse_vec(&v);
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_vec_empty() {
        let v: Vec<i32> = vec![];
        let result = reverse_vec(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_word_lengths() {
        let words = vec!["hello", "world", "rust"];
        let result = word_lengths(&words);
        assert_eq!(result["hello"], 5);
        assert_eq!(result["world"], 5);
        assert_eq!(result["rust"], 4);
    }

    #[test]
    fn test_word_lengths_empty() {
        let words: Vec<&str> = vec![];
        let result = word_lengths(&words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_char_frequency() {
        let result = char_frequency("aabbc");
        assert_eq!(result[&'a'], 2);
        assert_eq!(result[&'b'], 2);
        assert_eq!(result[&'c'], 1);
    }

    #[test]
    fn test_char_frequency_single() {
        let result = char_frequency("aaaa");
        assert_eq!(result[&'a'], 4);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_sum_vec() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_vec(&v), 15);
    }

    #[test]
    fn test_sum_vec_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(sum_vec(&v), 0);
    }

    #[test]
    fn test_sum_vec_negative() {
        let v = vec![10, -5, 3];
        assert_eq!(sum_vec(&v), 8);
    }
}
