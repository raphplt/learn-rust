// ============================================================
// Exercice 01 — Variables & Mutabilité
// Niveau 1: Concepts Fondamentaux
// ============================================================
//
// Les variables en Rust sont immutables par défaut. Vous devez utiliser `mut`
// pour les rendre mutables. Le shadowing permet de redéfinir une variable avec
// une nouvelle valeur ou un nouveau type. Les constantes sont définies avec `const`
// et doivent avoir un type explicite.
//
// 💡 Indices:
//   - `let x = 5;` crée une variable immutable
//   - `let mut y = 5;` rend la variable mutable
//   - `let x = x + 1;` crée un shadowing (nouvelle variable)
//   - Les constantes utilisent UPPER_SNAKE_CASE
//
// Pour lancer : cargo test --test ex01_variables
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Déclarez une constante gravitationnelle G = 6.674e-11
/// et retournez sa valeur.
fn declare_constant() -> f64 {
    todo!("Déclare la constante G et retourne sa valeur")
}

/// Créez une variable immutable x = 10, puis un shadowing
/// pour en faire x = "10". Retournez la longueur de la chaîne.
fn shadowing_demo() -> usize {
    todo!("Crée x = 10, puis shadowing avec x = \"10\", retourne la longueur")
}

/// Créez une variable mutable temperature = 20.0,
/// modifiez-la à 25.0, puis retournez la différence avec la valeur initiale.
fn mutable_temperature() -> f64 {
    todo!("Crée temperature mutable, la modifie à 25.0, retourne la différence")
}

/// Déclarez trois variables immutables: a = 5, b = 10, c = 3.
/// Retournez a + b * c (respectez la priorité des opérations).
fn immutable_math() -> i32 {
    todo!("Déclare a, b, c et retourne a + b * c")
}

/// Créez une variable mutable counter = 0. Incrémentez-la 5 fois dans une boucle.
/// Retournez la valeur finale.
fn mutable_counter() -> i32 {
    todo!("Crée counter mutable, l'incrémente 5 fois, retourne sa valeur finale")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declare_constant() {
        let result = declare_constant();
        assert!((result - 6.674e-11).abs() < 1e-15);
    }

    #[test]
    fn test_shadowing_demo() {
        let result = shadowing_demo();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_mutable_temperature() {
        let result = mutable_temperature();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_immutable_math() {
        let result = immutable_math();
        assert_eq!(result, 35); // 5 + 10 * 3
    }

    #[test]
    fn test_mutable_counter() {
        let result = mutable_counter();
        assert_eq!(result, 5);
    }
}
