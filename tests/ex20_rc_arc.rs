// ============================================================
// Exercice 20 — Rc, Arc & Interior Mutability
// Niveau 8: Propriété Partagée et Mutation Contrôlée
// ============================================================
//
// Rc<T> partage la propriété (single-threaded).
// Arc<T> partage la propriété (thread-safe).
// RefCell<T> permet la mutation intérieure (vérification à l'exécution).
// Rc<RefCell<T>> combine propriété partagée et mutation intérieure.
//
// 💡 Indices:
//   - `Rc::new(value)` : création
//   - `Rc::clone(&rc)` : duplique le pointeur (incrémente le refcount)
//   - `Arc::new(value)` : pour le multi-threading
//   - `RefCell::new(value)` : mutation intérieure
//   - `refcell.borrow()` : immutable borrow (&T)
//   - `refcell.borrow_mut()` : mutable borrow (&mut T)
//   - Rc<RefCell<T>> : propriété partagée + mutation intérieure
//   - `std::cell::Ref` / `RefMut` : smart pointers pour les borrows
//
// Pour lancer : cargo test --test ex20_rc_arc
// ============================================================

#[allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

// ---- Types et Fonctions à compléter ----

/// Structure simple avec un compteur de références.
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

/// Créez une liste chaînée avec Rc pour partager les nœuds.
/// Retournez [1, 2, 3] avec Rc.
fn create_rc_list() -> Rc<Node> {
    todo!("Créez une liste chaînée avec Rc partagé")
}

/// Clonez un Rc et vérifiez que le refcount augmente.
/// Utilisez Rc::clone et Rc::strong_count.
fn clone_rc_and_count() -> usize {
    todo!("Créez un Rc, clonez-le, et retournez le nombre de références")
}

/// Utilisez RefCell pour une mutation intérieure.
/// Modifiez la valeur et retournez-la.
fn refcell_mutation() -> i32 {
    todo!("Créez un RefCell avec 10, modifiez-le en 20, retournez la valeur")
}

/// Combinez Rc et RefCell pour partager une valeur mutable.
/// Modifiez la valeur via deux références partagées.
fn shared_mutable_value() -> i32 {
    todo!("Créez Rc<RefCell<i32>>, clonez-le, modifiez les deux, retournez la valeur")
}

/// Structure avec Rc<RefCell<>> pour des données partagées mutables.
struct Person {
    name: String,
    age: Rc<RefCell<u32>>,
}

impl Person {
    fn new(name: &str, initial_age: u32) -> Self {
        todo!("Créez une Person avec name et age enveloppé en Rc<RefCell>")
    }

    fn have_birthday(&self) {
        todo!("Incrémentez l'âge de 1 via borrow_mut()")
    }

    fn get_age(&self) -> u32 {
        todo!("Retournez l'âge courant via borrow()")
    }
}

/// Créez deux Person partageant le même Rc<RefCell<u32>> pour l'âge.
/// Modifiez l'âge et vérifiez que les deux Person le voient.
fn shared_age() -> (u32, u32) {
    todo!("Créez deux Person avec le même age Rc, modifiez-le, retournez les deux âges")
}

/// Arc pour thread-safety (même sans threads ici).
/// Créez un Arc et clonez-le.
fn arc_clone() -> usize {
    todo!("Créez un Arc, clonez-le, retournez Rc::strong_count (ou Arc::strong_count)")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rc_list() {
        let list = create_rc_list();
        assert_eq!(list.value, 1);
        // Vérifiez qu'il y a un next
        assert!(list.next.is_some());
    }

    #[test]
    fn test_clone_rc_and_count() {
        let count = clone_rc_and_count();
        assert_eq!(count, 2); // Original + clone
    }

    #[test]
    fn test_refcell_mutation() {
        let result = refcell_mutation();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_shared_mutable_value() {
        let result = shared_mutable_value();
        assert_eq!(result, 25); // 10 + 15 (les deux modifications)
    }

    #[test]
    fn test_person_new() {
        let person = Person::new("Alice", 25);
        assert_eq!(person.name, "Alice");
        assert_eq!(person.get_age(), 25);
    }

    #[test]
    fn test_person_have_birthday() {
        let person = Person::new("Bob", 30);
        person.have_birthday();
        assert_eq!(person.get_age(), 31);
    }

    #[test]
    fn test_person_multiple_birthdays() {
        let person = Person::new("Charlie", 20);
        person.have_birthday();
        person.have_birthday();
        person.have_birthday();
        assert_eq!(person.get_age(), 23);
    }

    #[test]
    fn test_shared_age() {
        let (age1, age2) = shared_age();
        // Les deux doivent avoir le même âge après modification
        assert_eq!(age1, age2);
    }

    #[test]
    fn test_arc_clone() {
        let count = arc_clone();
        assert_eq!(count, 2); // Original + clone
    }

    #[test]
    fn test_rc_refcell_combination() {
        let value = Rc::new(RefCell::new(100));
        let _clone = Rc::clone(&value);

        *value.borrow_mut() += 50;
        assert_eq!(*value.borrow(), 150);
    }

    #[test]
    fn test_multiple_persons_different_ages() {
        let alice = Person::new("Alice", 25);
        let bob = Person::new("Bob", 30);

        alice.have_birthday();
        bob.have_birthday();
        bob.have_birthday();

        assert_eq!(alice.get_age(), 26);
        assert_eq!(bob.get_age(), 32);
    }
}
