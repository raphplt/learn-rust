// ============================================================
// Exercice 14 — Gestion d'Erreurs
// Niveau 5: Opérateur ?, Traits From, et Types d'Erreurs Personnalisés
// ============================================================
//
// L'opérateur `?` simplifie la propagation d'erreurs dans une fonction
// retournant Result. `From` permet la conversion automatique d'erreurs.
// Les types d'erreurs personnalisés et Box<dyn Error> offrent flexibilité.
//
// 💡 Indices:
//   - `x?` : equivalent à `match x { Ok(v) => v, Err(e) => return Err(e.into()) }`
//   - `impl From<TypeA> for TypeB { fn from(a: TypeA) -> Self { ... } }`
//   - `Box<dyn Error>` : type d'erreur générique et flexible
//   - `anyhow::Result` : alternative pour rapidement, mais ici on fait maison
//   - `std::num::ParseIntError` : erreur du parsing d'entiers
//   - Les custom error types peuvent implémenter std::error::Error
//
// Pour lancer : cargo test --test ex14_erreurs
// ============================================================

#[allow(dead_code)]

use std::fmt;

// ---- Types d'erreurs personnalisés ----

/// Énumération des erreurs possibles dans nos opérations.
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeValue,
    InvalidInput(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division par zéro"),
            MathError::NegativeValue => write!(f, "Valeur négative non autorisée"),
            MathError::InvalidInput(msg) => write!(f, "Entrée invalide: {}", msg),
        }
    }
}

impl std::error::Error for MathError {}

/// Type d'erreur pour les opérations de chaînes.
#[derive(Debug, Clone, PartialEq)]
pub enum StringError {
    EmptyString,
    TooLong(usize),
    InvalidCharacter(char),
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StringError::EmptyString => write!(f, "Chaîne vide"),
            StringError::TooLong(len) => write!(f, "Chaîne trop longue: {} caractères", len),
            StringError::InvalidCharacter(c) => write!(f, "Caractère invalide: {}", c),
        }
    }
}

impl std::error::Error for StringError {}

// ---- Conversions From ----

/// Implémentez la conversion de ParseIntError en MathError.
impl From<std::num::ParseIntError> for MathError {
    fn from(err: std::num::ParseIntError) -> Self {
        todo!("Retournez MathError::InvalidInput avec un message")
    }
}

/// Implémentez la conversion de StringError en MathError.
impl From<StringError> for MathError {
    fn from(err: StringError) -> Self {
        todo!("Convertissez StringError en MathError::InvalidInput")
    }
}

// ---- Fonctions à compléter ----

/// Divisez deux nombres. Retournez Err si le diviseur est zéro.
/// Utilisez le type MathError.
fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    todo!("Vérifiez b == 0 et retournez Ok(a / b) ou Err")
}

/// Prenez un nombre positif. Retournez Err si négatif.
fn ensure_positive(n: i32) -> Result<i32, MathError> {
    todo!("Vérifiez n >= 0")
}

/// Calculez 1 / (a - b). Enchaînez les opérations avec `?`.
/// Les erreurs sont automatiquement propagées.
fn safe_division_chain(a: i32, b: i32) -> Result<i32, MathError> {
    todo!("Calculez (a - b), vérifiez que ce n'est pas zéro, retournez 1 / résultat")
}

/// Parsez deux nombres depuis des chaînes et divisez-les.
/// Utilisez `?` pour propager les erreurs de parsing.
/// Les ParseIntError sont automatiquement convertis en MathError via From.
fn divide_parsed(s1: &str, s2: &str) -> Result<i32, MathError> {
    todo!("Parsez s1 et s2 avec `?`, puis divisez")
}

/// Validez une chaîne : ne doit pas être vide et < 100 caractères.
/// Retournez Err(StringError) si non valide.
/// Enchaînez avec ensure_positive(len) en convertissant avec `?`.
fn validate_and_ensure_positive(s: &str) -> Result<i32, MathError> {
    todo!("Validez s, puis enchaînez avec ensure_positive de sa longueur")
}

/// Prendre un slice de i32, parsez chaque élément, et retournez le produit.
/// Utilisez `?` pour arrêter à la première erreur.
fn product_of_parsed(strs: &[&str]) -> Result<i32, MathError> {
    todo!("Parsez tous les strs et retournez leur produit, ou la première erreur")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2).unwrap(), 5);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MathError::DivisionByZero);
    }

    #[test]
    fn test_ensure_positive_ok() {
        assert_eq!(ensure_positive(5).unwrap(), 5);
        assert_eq!(ensure_positive(0).unwrap(), 0);
    }

    #[test]
    fn test_ensure_positive_err() {
        let result = ensure_positive(-1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MathError::NegativeValue);
    }

    #[test]
    fn test_safe_division_chain_success() {
        // 1 / (10 - 5) = 1 / 5 = 0
        let result = safe_division_chain(10, 5);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_safe_division_chain_zero_divisor() {
        // 1 / (5 - 5) : division par zéro
        let result = safe_division_chain(5, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_divide_parsed_success() {
        let result = divide_parsed("20", "4");
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_divide_parsed_invalid_first() {
        let result = divide_parsed("abc", "4");
        assert!(result.is_err());
    }

    #[test]
    fn test_divide_parsed_zero_divisor() {
        let result = divide_parsed("10", "0");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_and_ensure_positive_ok() {
        let result = validate_and_ensure_positive("hello");
        assert_eq!(result.unwrap(), 5); // "hello" a 5 caractères
    }

    #[test]
    fn test_validate_and_ensure_positive_empty() {
        let result = validate_and_ensure_positive("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_and_ensure_positive_too_long() {
        let s = "a".repeat(100);
        let result = validate_and_ensure_positive(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_product_of_parsed_success() {
        let result = product_of_parsed(&["2", "3", "5"]);
        assert_eq!(result.unwrap(), 30);
    }

    #[test]
    fn test_product_of_parsed_with_error() {
        let result = product_of_parsed(&["2", "abc", "5"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_product_of_parsed_empty() {
        let result = product_of_parsed(&[]);
        assert_eq!(result.unwrap(), 1); // produit vide = 1
    }

    #[test]
    fn test_error_display() {
        assert_eq!(MathError::DivisionByZero.to_string(), "Division par zéro");
        assert_eq!(
            MathError::InvalidInput("test".to_string()).to_string(),
            "Entrée invalide: test"
        );
    }
}
