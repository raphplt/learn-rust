// ============================================================
// Exercice 16 — Itérateurs
// Niveau 6: map, filter, collect, fold, chain, zip, enumerate
// ============================================================
//
// Les itérateurs en Rust sont lazy (paresseux) et combinent bien avec les fermetures.
// map transforme chaque élément, filter sélectionne selon un prédicat,
// collect rassemble en une collection, fold accumule une valeur.
// chain, zip, enumerate permettent des compositions puissantes.
//
// 💡 Indices:
//   - `v.iter()` : itérateur sur &T
//   - `v.into_iter()` : itérateur consommant v
//   - `.map(|x| x * 2)` : transformez chaque élément
//   - `.filter(|x| x % 2 == 0)` : gardez les éléments qui satisfont le prédicat
//   - `.collect::<Vec<_>>()` : rassemblez en Vec
//   - `.fold(init, |acc, x| acc + x)` : accumulation
//   - `.chain(autre_iter)` : fusionnez deux itérateurs
//   - `.zip(autre_iter)` : appairez éléments
//   - `.enumerate()` : obtenez (indice, élément)
//   - `.any(|x| ...)`, `.all(|x| ...)` : testez conditions
//
// Pour lancer : cargo test --test ex16_iterateurs
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Doublez chaque élément d'un vecteur en utilisant map.
fn double_with_map(v: &[i32]) -> Vec<i32> {
    todo!("Utilisez iter().map() et collect()")
}

/// Filtrez pour garder seulement les nombres > 10.
fn filter_greater_than_ten(v: &[i32]) -> Vec<i32> {
    todo!("Utilisez iter().filter() et collect()")
}

/// Retournez les carrés des nombres pairs.
fn squares_of_evens(v: &[i32]) -> Vec<i32> {
    todo!("Chaînez filter() et map() pour obtenir les carrés des pairs")
}

/// Additionnez tous les éléments en utilisant fold.
fn sum_with_fold(v: &[i32]) -> i32 {
    todo!("Utilisez fold pour accumuler la somme")
}

/// Chaînez deux vecteurs et collectez en un.
fn chain_vecs(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    todo!("Utilisez chain() pour fusionner v1 et v2")
}

/// Appairez les éléments de deux vecteurs avec zip.
/// Retournez une Vec de tuples (x, y).
fn zip_vecs(v1: &[i32], v2: &[i32]) -> Vec<(i32, i32)> {
    todo!("Utilisez zip() pour appairez les éléments")
}

/// Pour chaque élément, retournez (indice, valeur).
fn enumerate_vec<'a>(v: &[&'a str]) -> Vec<(usize, &'a str)> {
    todo!("Utilisez enumerate() et collect()")
}

/// Vérifiez si tous les nombres sont positifs.
fn all_positive(v: &[i32]) -> bool {
    todo!("Utilisez any() ou all()")
}

/// Vérifiez s'il existe au moins un nombre > 100.
fn any_greater_than_hundred(v: &[i32]) -> bool {
    todo!("Utilisez any() pour tester la condition")
}

/// Transformez une Vec<i32> en Vec<String> ("5" => "num: 5").
fn format_numbers(v: &[i32]) -> Vec<String> {
    todo!("Mappez chaque nombre en string formaté")
}

/// Calculez le produit de tous les éléments.
fn product_with_fold(v: &[i32]) -> i32 {
    todo!("Utilisez fold() avec la multiplication")
}

/// Retournez les mots avec plus de 3 caractères, triés par longueur (plus longs d'abord).
/// Utilisez filter, collect, puis sort_by_key.
fn long_words_sorted<'a>(words: &[&'a str]) -> Vec<&'a str> {
    todo!("Filtrez les mots > 3 chars, collectez, et triez par longueur décroissante")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_with_map() {
        let v = vec![1, 2, 3, 4];
        let result = double_with_map(&v);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_double_with_map_empty() {
        let v: Vec<i32> = vec![];
        let result = double_with_map(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_greater_than_ten() {
        let v = vec![5, 15, 8, 20, 3];
        let result = filter_greater_than_ten(&v);
        assert_eq!(result, vec![15, 20]);
    }

    #[test]
    fn test_filter_greater_than_ten_none() {
        let v = vec![1, 2, 3];
        let result = filter_greater_than_ten(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_squares_of_evens() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result = squares_of_evens(&v);
        assert_eq!(result, vec![4, 16, 36]); // 2^2, 4^2, 6^2
    }

    #[test]
    fn test_squares_of_evens_no_evens() {
        let v = vec![1, 3, 5];
        let result = squares_of_evens(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_sum_with_fold() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_with_fold(&v), 15);
    }

    #[test]
    fn test_sum_with_fold_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(sum_with_fold(&v), 0);
    }

    #[test]
    fn test_chain_vecs() {
        let v1 = vec![1, 2];
        let v2 = vec![3, 4];
        let result = chain_vecs(&v1, &v2);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chain_vecs_empty() {
        let v1: Vec<i32> = vec![];
        let v2 = vec![1, 2];
        let result = chain_vecs(&v1, &v2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_zip_vecs() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![10, 20, 30];
        let result = zip_vecs(&v1, &v2);
        assert_eq!(result, vec![(1, 10), (2, 20), (3, 30)]);
    }

    #[test]
    fn test_zip_vecs_different_lengths() {
        let v1 = vec![1, 2];
        let v2 = vec![10, 20, 30];
        let result = zip_vecs(&v1, &v2);
        assert_eq!(result, vec![(1, 10), (2, 20)]); // zip s'arrête à la plus courte
    }

    #[test]
    fn test_enumerate_vec() {
        let v = vec!["a", "b", "c"];
        let result = enumerate_vec(&v);
        assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    #[test]
    fn test_enumerate_vec_empty() {
        let v: Vec<&str> = vec![];
        let result = enumerate_vec(&v);
        assert!(result.is_empty());
    }

    #[test]
    fn test_all_positive_true() {
        let v = vec![1, 2, 3, 4];
        assert!(all_positive(&v));
    }

    #[test]
    fn test_all_positive_with_zero() {
        let v = vec![1, 0, 3];
        assert!(!all_positive(&v));
    }

    #[test]
    fn test_all_positive_with_negative() {
        let v = vec![1, -1, 3];
        assert!(!all_positive(&v));
    }

    #[test]
    fn test_any_greater_than_hundred_true() {
        let v = vec![50, 150, 75];
        assert!(any_greater_than_hundred(&v));
    }

    #[test]
    fn test_any_greater_than_hundred_false() {
        let v = vec![50, 75, 99];
        assert!(!any_greater_than_hundred(&v));
    }

    #[test]
    fn test_format_numbers() {
        let v = vec![1, 2, 3];
        let result = format_numbers(&v);
        assert_eq!(result, vec!["num: 1", "num: 2", "num: 3"]);
    }

    #[test]
    fn test_product_with_fold() {
        let v = vec![2, 3, 4];
        assert_eq!(product_with_fold(&v), 24);
    }

    #[test]
    fn test_product_with_fold_with_zero() {
        let v = vec![2, 0, 4];
        assert_eq!(product_with_fold(&v), 0);
    }

    #[test]
    fn test_long_words_sorted() {
        let words = vec!["a", "test", "hello", "ok", "world"];
        let result = long_words_sorted(&words);
        // Mots > 3 chars: "test" (4), "hello" (5), "world" (5)
        // Triés par longueur décroissante: "hello", "world" (5 chars), "test" (4 chars)
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[2], "test");
    }
}
