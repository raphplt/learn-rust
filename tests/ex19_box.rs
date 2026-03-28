// ============================================================
// Exercice 19 — Box & Smart Pointers
// Niveau 8: Allocation Dynamique et Pointeurs Intelligents
// ============================================================
//
// Box<T> alloue une valeur sur le heap et donne sa propriété.
// C'est utile pour les types récursifs, les trait objects, et l'allocation dynamique.
// Le trait Deref permet de traiter une Box<T> presque comme une &T.
// Le trait Drop permet du nettoyage personnalisé.
//
// 💡 Indices:
//   - `Box::new(value)` : alloue sur le heap
//   - `*boxed_value` : déréférence (fonctionne grâce au trait Deref)
//   - `enum List { Node(i32, Box<List>), Nil }` : type récursif
//   - `impl Deref for MyType { type Target = ...; fn deref(&self) -> &Target { ... } }`
//   - `impl Drop for MyType { fn drop(&mut self) { ... } }` : cleanup
//   - Les trait objects : `Box<dyn Trait>`
//   - Deref coercion : une Box peut être automatiquement convertie à &T
//
// Pour lancer : cargo test --test ex19_box
// ============================================================

#[allow(dead_code)]

use std::ops::Deref;

// ---- Types et Fonctions à compléter ----

/// Une simple liste chaînée récursive.
#[derive(Debug, Clone, PartialEq)]
enum List {
    Node(i32, Box<List>),
    Nil,
}

/// Créez une liste chaînée avec les éléments [1, 2, 3].
fn create_list() -> List {
    todo!("Créez List::Node(1, Box::new(List::Node(2, Box::new(List::Node(3, Box::new(List::Nil))))))")
}

/// Retournez la somme de tous les éléments d'une liste chaînée.
fn sum_list(list: &List) -> i32 {
    todo!("Récursivement additionnez les éléments de la liste")
}

/// Retournez la longueur d'une liste chaînée.
fn length_list(list: &List) -> usize {
    todo!("Comptez les nœuds récursivement")
}

/// Retournez le premier élément (Some(x)) ou None si la liste est vide.
fn first_element(list: &List) -> Option<i32> {
    todo!("Pattern matching pour retourner le premier élément")
}

/// Implémentez le trait Deref pour un wrapper simple.
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        todo!("Retournez MyBox { value }")
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!("Retournez &self.value")
    }
}

/// Structure pour tester le trait Drop.
#[derive(Debug)]
struct TrackedValue {
    name: String,
}

impl Drop for TrackedValue {
    fn drop(&mut self) {
        todo!("Afficherez un message de cleanup (ou ne rien faire pour le test)")
    }
}

/// Créez une TrackedValue et retournez sa Box.
fn create_tracked(name: &str) -> Box<TrackedValue> {
    todo!("Créez Box::new(TrackedValue { name: name.to_string() })")
}

/// Stockez trois trait objects dans un Vec.
fn create_trait_objects() -> Vec<Box<dyn std::fmt::Display>> {
    todo!("Retournez un Vec contenant Box<String>, Box<i32>, etc. (types implémentant Display)")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list = create_list();
        assert_eq!(sum_list(&list), 6); // 1 + 2 + 3
    }

    #[test]
    fn test_sum_list_single() {
        let list = List::Node(42, Box::new(List::Nil));
        assert_eq!(sum_list(&list), 42);
    }

    #[test]
    fn test_sum_list_empty() {
        let list = List::Nil;
        assert_eq!(sum_list(&list), 0);
    }

    #[test]
    fn test_length_list() {
        let list = create_list();
        assert_eq!(length_list(&list), 3);
    }

    #[test]
    fn test_length_list_empty() {
        let list: List = List::Nil;
        assert_eq!(length_list(&list), 0);
    }

    #[test]
    fn test_length_list_single() {
        let list = List::Node(99, Box::new(List::Nil));
        assert_eq!(length_list(&list), 1);
    }

    #[test]
    fn test_first_element_some() {
        let list = create_list();
        assert_eq!(first_element(&list), Some(1));
    }

    #[test]
    fn test_first_element_none() {
        let list: List = List::Nil;
        assert_eq!(first_element(&list), None);
    }

    #[test]
    fn test_mybox_new() {
        let b = MyBox::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_mybox_deref() {
        let b = MyBox::new("hello".to_string());
        // Deref coercion : b est automatiquement déréférencé
        assert_eq!(b.as_str(), "hello");
    }

    #[test]
    fn test_mybox_nested() {
        let b = MyBox::new(MyBox::new(100));
        // Deref coercion en cascade
        assert_eq!(**b, 100);
    }

    #[test]
    fn test_create_tracked() {
        let tracked = create_tracked("test_value");
        assert_eq!(tracked.name, "test_value");
    }

    #[test]
    fn test_create_trait_objects() {
        let objects = create_trait_objects();
        assert_eq!(objects.len(), 3);
    }

    #[test]
    fn test_list_equality() {
        let list1 = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
        let list2 = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
        assert_eq!(list1, list2);
    }

    #[test]
    fn test_empty_list_sum() {
        let list: List = List::Nil;
        assert_eq!(sum_list(&list), 0);
    }
}
