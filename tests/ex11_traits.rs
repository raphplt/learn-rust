// ============================================================
// Exercice 11 — Traits
// Niveau 4: Définition & Implémentation de Traits
// ============================================================
//
// Les traits définissent une interface (ensemble de méthodes) que plusieurs types
// peuvent implémenter. On définit un trait avec `trait Nom { fn method(); }`.
// Les implémentations utilisent `impl Trait for Type { ... }`.
// Les traits standard comme Display, Debug, PartialEq facilitent la compatibilité.
//
// 💡 Indices:
//   - `trait Shape { fn area(&self) -> f64; }` définit un trait
//   - `impl Shape for Circle { fn area(&self) -> f64 { ... } }` l'implémente
//   - `impl Default for Point { fn default() -> Self { ... } }` trait associé
//   - `println!("{:?}", x);` utilise le trait Debug (dérivable avec #[derive])
//
// Pour lancer : cargo test --test ex11_traits
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Définissez un trait Shape avec méthode area() -> f64 et perimeter() -> f64.
trait Shape {
    todo_trait_methods: u32, // Remplace-moi
}

/// Définissez une struct Circle avec radius: f64.
/// Implémentez Shape pour Circle.
struct Circle {
    todo_struct_fields: u32, // Remplace-moi
}

impl Shape for Circle {
    todo!("Implémentez area et perimeter pour Circle")
}

/// Définissez une struct Rectangle avec width et height.
/// Implémentez Shape pour Rectangle.
struct Rectangle {
    todo_struct_fields: u32, // Remplace-moi
}

impl Shape for Rectangle {
    todo!("Implémentez area et perimeter pour Rectangle")
}

/// Définissez un trait Printable avec méthode print(&self) -> String.
trait Printable {
    todo_trait_methods: u32, // Remplace-moi
}

/// Définissez une struct Person avec name et age.
/// Implémentez Printable pour Person.
struct Person {
    todo_struct_fields: u32, // Remplace-moi
}

impl Printable for Person {
    todo!("Implémentez print pour Person")
}

/// Définissez un trait Calculator avec méthode calculate(a: i32, b: i32) -> i32.
/// Écrivez une struct Adder et une struct Multiplier qui l'implémentent.
trait Calculator {
    todo_trait_methods: u32, // Remplace-moi
}

struct Adder;
struct Multiplier;

impl Calculator for Adder {
    todo!("Implémentez calculate pour Adder (retourne a + b)")
}

impl Calculator for Multiplier {
    todo!("Implémentez calculate pour Multiplier (retourne a * b)")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 1.0 };
        assert!((circle.area() - PI).abs() < 0.01);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle { radius: 1.0 };
        assert!((circle.perimeter() - 2.0 * PI).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 4.0, height: 5.0 };
        assert!((rect.area() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle { width: 4.0, height: 5.0 };
        assert!((rect.perimeter() - 18.0).abs() < 0.01);
    }

    #[test]
    fn test_person_printable() {
        let person = Person {
            name: "Alice".to_string(),
            age: 25,
        };
        let output = person.print();
        assert!(output.contains("Alice"));
        assert!(output.contains("25"));
    }

    #[test]
    fn test_adder() {
        let adder = Adder;
        assert_eq!(adder.calculate(5, 3), 8);
    }

    #[test]
    fn test_multiplier() {
        let multiplier = Multiplier;
        assert_eq!(multiplier.calculate(5, 3), 15);
    }
}
