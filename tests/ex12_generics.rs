// ============================================================
// Exercice 12 — Génériques & Trait Bounds
// Niveau 4: Types Génériques & Contraintes
// ============================================================
//
// Les génériques permettent d'écrire du code pour plusieurs types.
// Les trait bounds contraignent les types génériques à implémenter certains traits.
// La syntaxe `<T: Trait>` signifie "T doit implémenter Trait".
// Cela permet de réutiliser du code tout en gardant la sécurité des types.
//
// 💡 Indices:
//   - `fn identity<T>(x: T) -> T { x }` est une fonction générique
//   - `fn max<T: PartialOrd>(a: T, b: T) -> T { ... }` contraint T
//   - `struct Box<T> { value: T }` est une struct générique
//   - `impl<T> Box<T> { fn new(value: T) -> Self { ... } }` impl générique
//   - `where` permet des contraintes complexes
//
// Pour lancer : cargo test --test ex12_generics
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une fonction générique identity<T>(x: T) -> T qui retourne x inchangé.
fn identity<T>(x: T) -> T {
    todo!("Retourne x inchangé")
}

/// Écrivez une fonction générique max<T: PartialOrd>(a: T, b: T) -> T
/// qui retourne le maximum.
fn max<T: PartialOrd>(a: T, b: T) -> T {
    todo!("Retourne le maximum de a et b")
}

/// La struct Container<T> est définie pour toi. Implémente les méthodes ci-dessous.
struct Container<T> {
    value: T,
}

/// Implémentez Container<T> avec une méthode new(value: T) -> Self.
impl<T> Container<T> {
    fn new(value: T) -> Self {
        todo!("Retourne un Container avec la valeur")
    }

    /// Implémentez une méthode get_ref(&self) -> &T.
    fn get_ref(&self) -> &T {
        todo!("Retourne une référence à la valeur")
    }

    /// Implémentez une méthode unwrap(self) -> T pour extraire la valeur.
    fn unwrap(self) -> T {
        todo!("Retourne la valeur")
    }
}

/// Écrivez une fonction générique first<T>(items: &[T]) -> Option<&T>
/// qui retourne une référence au premier élément.
fn first<T>(items: &[T]) -> Option<&T> {
    todo!("Retourne une référence au premier élément ou None")
}

/// Écrivez une fonction générique pair<T, U>(a: T, b: U) -> (T, U)
/// qui crée un tuple à partir de deux valeurs de types différents.
fn pair<T, U>(a: T, b: U) -> (T, U) {
    todo!("Retourne un tuple (a, b)")
}

/// La struct Pair<T, U> est définie pour toi. Implémente la méthode swap ci-dessous.
struct Pair<T, U> {
    first: T,
    second: U,
}

/// Implémentez Pair<T, U> avec une méthode swap(self) -> Pair<U, T>.
impl<T, U> Pair<T, U> {
    fn swap(self) -> Pair<U, T> {
        todo!("Retourne un Pair avec les éléments inversés")
    }
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(identity(42), 42);
        assert_eq!(identity("hello"), "hello");
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(3.5, 2.1), 3.5);
        assert_eq!(max("a", "z"), "z");
    }

    #[test]
    fn test_container_new() {
        let container = Container::new(42);
        assert_eq!(container.unwrap(), 42);
    }

    #[test]
    fn test_container_get_ref() {
        let container = Container::new("Rust");
        assert_eq!(container.get_ref(), &"Rust");
    }

    #[test]
    fn test_first() {
        let items = [1, 2, 3, 4, 5];
        assert_eq!(first(&items), Some(&1));
        assert_eq!(first::<i32>(&[]), None);
    }

    #[test]
    fn test_pair() {
        let p = pair(5, "hello");
        assert_eq!(p.0, 5);
        assert_eq!(p.1, "hello");
    }

    #[test]
    fn test_pair_swap() {
        let p = Pair { first: 5, second: "hello" };
        let swapped = p.swap();
        assert_eq!(swapped.first, "hello");
        assert_eq!(swapped.second, 5);
    }
}
