// ============================================================
// Exercice 18 — Lifetimes
// Niveau 7: Annotations de Durée de Vie
// ============================================================
//
// Les lifetimes garantissent que les références ne survivent pas aux données
// qu'elles pointent. On les note avec 'a, 'b, etc. Le compilateur les élide
// souvent grâce à des règles (elision rules), mais parfois il faut les annoter.
//
// 💡 Indices:
//   - `fn borrow<'a>(x: &'a T) -> &'a T` : la référence retournée vit aussi longtemps que x
//   - `struct Pair<'a> { first: &'a str, second: &'a str }` : références dans une struct
//   - `'a: 'b` : 'a vit au moins aussi longtemps que 'b (lifetime bound)
//   - Règles d'élision : chaque paramètre emprunté reçoit un lifetime distinct,
//     et s'il n'y a qu'un lifetime en entrée, la sortie le reçoit
//   - `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str` : les deux doivent vivre aussi longtemps
//   - Les structs avec références doivent annoter les lifetimes de leurs références
//
// Pour lancer : cargo test --test ex18_lifetimes
// ============================================================

#[allow(dead_code)]

// ---- Types et Fonctions à compléter ----

/// Retournez la référence qui vit le plus longtemps entre deux.
/// Les deux références doivent vivre le même temps.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    todo!("Retournez s1 ou s2 selon lequel est le plus long")
}

/// Structure avec une référence. Le lifetime doit être annoté.
struct Borrowed<'a> {
    data: &'a str,
}

/// Implémentez une méthode new pour Borrowed.
impl<'a> Borrowed<'a> {
    fn new(data: &'a str) -> Self {
        todo!("Retournez Borrowed { data }")
    }
}

/// Implémentez une méthode get_data qui retourne &'a str.
impl<'a> Borrowed<'a> {
    fn get_data(&self) -> &'a str {
        todo!("Retournez self.data")
    }
}

/// Structure contenant deux références au même type.
/// Les deux références doivent avoir le même lifetime.
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

/// Implémentez une méthode new pour Pair.
impl<'a> Pair<'a> {
    fn new(first: &'a str, second: &'a str) -> Self {
        todo!("Retournez Pair { first, second }")
    }
}

/// Implémentez une méthode longest qui retourne la plus longue chaîne.
impl<'a> Pair<'a> {
    fn longest(&self) -> &'a str {
        todo!("Retournez self.first ou self.second, le plus long")
    }
}

/// Convertissez un &str en Vec<&str> en séparant par un séparateur.
/// Le lifetime des &str de sortie doit être lié au &str d'entrée.
fn split_borrowed<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    todo!("Splittez s par sep et retournez un Vec de &'a str")
}

/// Retournez un slice de deux chaînes concaténées (ceci ne compile pas comme espéré).
/// Cet exercice montre les limites : on ne peut pas retourner une référence
/// à une donnée temporaire. Retournez plutôt une String propriétaire.
fn concatenate_owned(s1: &str, s2: &str) -> String {
    todo!("Concaténez s1 et s2 en retournant une String")
}

/// Écrivez une fonction avec deux lifetimes différents.
/// x vit 'a, y vit 'b, et on retourne quelque chose avec 'a.
fn first_of_different_lifetimes<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    todo!("Retournez toujours x (qui vit 'a)")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_first_longer() {
        let s1 = "hello";
        let s2 = "hi";
        assert_eq!(longest(s1, s2), "hello");
    }

    #[test]
    fn test_longest_second_longer() {
        let s1 = "hi";
        let s2 = "goodbye";
        assert_eq!(longest(s1, s2), "goodbye");
    }

    #[test]
    fn test_longest_equal() {
        let s1 = "cat";
        let s2 = "dog";
        assert_eq!(longest(s1, s2), s1); // ou s2, peu importe
    }

    #[test]
    fn test_borrowed_new() {
        let s = "test";
        let b = Borrowed::new(s);
        assert_eq!(b.data, "test");
    }

    #[test]
    fn test_borrowed_get_data() {
        let s = "hello";
        let b = Borrowed::new(s);
        assert_eq!(b.get_data(), "hello");
    }

    #[test]
    fn test_pair_new() {
        let s1 = "rust";
        let s2 = "lang";
        let p = Pair::new(s1, s2);
        assert_eq!(p.first, "rust");
        assert_eq!(p.second, "lang");
    }

    #[test]
    fn test_pair_longest() {
        let s1 = "short";
        let s2 = "a very long string";
        let p = Pair::new(s1, s2);
        assert_eq!(p.longest(), "a very long string");
    }

    #[test]
    fn test_pair_longest_equal() {
        let s1 = "equal";
        let s2 = "equal";
        let p = Pair::new(s1, s2);
        assert_eq!(p.longest(), "equal");
    }

    #[test]
    fn test_split_borrowed() {
        let s = "a,b,c";
        let result = split_borrowed(s, ",");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "b");
        assert_eq!(result[2], "c");
    }

    #[test]
    fn test_split_borrowed_single() {
        let s = "hello";
        let result = split_borrowed(s, ",");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "hello");
    }

    #[test]
    fn test_concatenate_owned() {
        let s1 = "hello";
        let s2 = "world";
        let result = concatenate_owned(s1, s2);
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_concatenate_owned_with_space() {
        let s1 = "hello";
        let s2 = "world";
        // On peut utiliser la String résultante après sa création
        let result = concatenate_owned(s1, s2);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_first_of_different_lifetimes() {
        let short = "short";
        let long = "a very long string";
        let result = first_of_different_lifetimes(short, long);
        assert_eq!(result, "short");
    }

    #[test]
    fn test_lifetime_scope() {
        // Cet test vérifie que le lifetime est correctement géré.
        let data = String::from("owned data");
        let borrowed = Borrowed::new(&data);
        assert_eq!(borrowed.get_data(), "owned data");
        // La String propriétaire vit au moins aussi longtemps que le borrowed
    }
}
