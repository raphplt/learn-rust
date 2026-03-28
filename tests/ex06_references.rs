// ============================================================
// Exercice 06 — Références & Borrowing
// Niveau 2: Références Immuables & Mutables
// ============================================================
//
// Les références permettent d'emprunter une valeur sans en prendre la propriété.
// Les références immuables (&T) ne peuvent pas modifier la valeur.
// Les références mutables (&mut T) le peuvent, mais une seule peut exister à la fois.
// C'est le système de borrowing qui prévient les data races.
//
// 💡 Indices:
//   - `let r = &x;` crée une référence immuable
//   - `let r = &mut x;` crée une référence mutable (x doit être mut)
//   - `*r` déréférence une référence
//   - Plusieurs refs immuables OK, mais une seule ref mutable à la fois
//
// Pour lancer : cargo test --test ex06_references
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une fonction qui prend une référence à un i32 et retourne sa valeur doublée.
fn double_by_reference(n: &i32) -> i32 {
    todo!("Retourne *n * 2")
}

/// Écrivez une fonction qui prend une référence mutable à un i32 et l'incrémente.
fn increment_by_reference(n: &mut i32) {
    todo!("Incrémente *n de 1")
}

/// Créez une variable, passez-la par référence immuable à deux fonctions différentes
/// (qui la lisent), puis retournez la somme de leurs résultats.
fn multiple_immutable_refs() -> i32 {
    fn read_and_add_5(n: &i32) -> i32 { *n + 5 }
    fn read_and_double(n: &i32) -> i32 { *n * 2 }
    let x = 10;
    todo!("Passe &x à deux fonctions et retourne la somme de leurs résultats")
}

/// Créez une variable mutable, modifiez-la via une référence mutable,
/// puis retournez la valeur finale.
fn mutable_reference_modify() -> i32 {
    todo!("Crée x mut = 0, le modifie via &mut x à 42, retourne 42")
}

/// Écrivez une fonction qui prend une chaîne par référence et retourne le nombre
/// de caractères en majuscule.
fn count_uppercase(s: &str) -> usize {
    todo!("Compte les caractères majuscules dans s")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_by_reference() {
        let n = 5;
        assert_eq!(double_by_reference(&n), 10);
    }

    #[test]
    fn test_increment_by_reference() {
        let mut n = 5;
        increment_by_reference(&mut n);
        assert_eq!(n, 6);
    }

    #[test]
    fn test_multiple_immutable_refs() {
        assert_eq!(multiple_immutable_refs(), 35); // 10+5 + 10*2 = 15 + 20
    }

    #[test]
    fn test_mutable_reference_modify() {
        assert_eq!(mutable_reference_modify(), 42);
    }

    #[test]
    fn test_count_uppercase() {
        assert_eq!(count_uppercase("RuSt"), 2);
        assert_eq!(count_uppercase("hello"), 0);
        assert_eq!(count_uppercase("HELLO"), 5);
    }
}
