// ============================================================
// Exercice 05 — Ownership (Propriété)
// Niveau 2: Ownership & Move Semantics
// ============================================================
//
// L'ownership est le cœur de Rust. Chaque valeur a un propriétaire unique.
// Quand on assigne ou passe une variable, la propriété se transfère (move).
// Les types Copy (i32, f64, bool) ne se movent pas, ils se copient.
// Le clone() crée une copie explicite d'une valeur non-Copy.
//
// 💡 Indices:
//   - `let s = String::from("hello");` alloue mémoire sur le heap
//   - `let s2 = s;` transfère la propriété (s n'est plus valide)
//   - `let s2 = s.clone();` copie explicitement s
//   - Les i32, bool, f64 sont Copy et se copient automatiquement
//
// Pour lancer : cargo test --test ex05_ownership
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Créez un String, passez-le à une fonction qui le retourne,
/// puis retournez sa longueur.
fn take_and_return_string() -> usize {
    fn process_string(s: String) -> String {
        s
    }
    let s = String::from("Rust");
    let s = process_string(s);
    todo!("Retourne la longueur de s après process_string")
}

/// Créez deux variables: une String et un i32. Clonez la String et
/// vérifiez que les deux copies existent indépendamment.
/// Retournez 1 si clonage réussi, 0 sinon.
fn clone_string() -> i32 {
    todo!("Clone une String et vérifie que les deux copies existent")
}

/// Écrivez une fonction qui prend ownership d'une String,
/// ajoute " world" et la retourne.
fn append_world(mut s: String) -> String {
    todo!("Ajoute \" world\" à s et retourne-la")
}

/// Les i32 sont Copy. Vérifiez qu'un i32 peut être utilisé après
/// avoir été assigné. Retournez la somme de l'original et de la copie.
fn copy_behavior() -> i32 {
    todo!("Crée x: i32, assigne y = x, retourne x + y")
}

/// Créez un Vec<i32>, clonez-le, modifiez le clone,
/// et vérifiez que l'original est inchangé.
fn vec_clone_independence() -> bool {
    todo!("Clone un Vec, le modifie, et vérifie que l'original est inchangé")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_and_return_string() {
        assert_eq!(take_and_return_string(), 4);
    }

    #[test]
    fn test_clone_string() {
        assert_eq!(clone_string(), 1);
    }

    #[test]
    fn test_append_world() {
        let s = String::from("Hello");
        let result = append_world(s);
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_copy_behavior() {
        assert_eq!(copy_behavior(), 10); // 5 + 5
    }

    #[test]
    fn test_vec_clone_independence() {
        assert_eq!(vec_clone_independence(), true);
    }
}
