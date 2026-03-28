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
    const G: f64 = 6.674e-11;
    G
   
}

/// Créez une variable immutable x = 10, puis un shadowing
/// pour en faire x = "10". Retournez la longueur de la chaîne.
fn shadowing_demo() -> usize {
    let x = 10;
    let x = "10";
    x.len()
}

/// Créez une variable mutable temperature = 20.0,
/// modifiez-la à 25.0, puis retournez la différence avec la valeur initiale.
fn mutable_temperature() -> f64 {
   let mut temperature = 20.0;
    let temp     = temperature;
    temperature = 25.0;
    temperature - temp
}

/// Déclarez trois variables immutables: a = 5, b = 10, c = 3.
/// Retournez a + b * c (respectez la priorité des opérations).
fn immutable_math() -> i32 {
    let (a,b,c) = (5,10,3);
    a+b*c

}

/// Créez une variable mutable counter = 0. Incrémentez-la 5 fois dans une boucle.
/// Retournez la valeur finale.
fn mutable_counter() -> i32 {
    let mut counter = 0;
    for _n in 1..6 {
        counter+= 1;
    }
    counter
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
