// ============================================================
// Exercice 08 — Structs
// Niveau 3: Définition & Implémentation
// ============================================================
//
// Les structs regroupent des données. On les définit avec `struct Nom { field: Type }`.
// Les blocs `impl` contiennent les méthodes (prennent &self, &mut self, ou self).
// Les fonctions associées (sans self) peuvent être appelées avec `Struct::function()`.
//
// 💡 Indices:
//   - `struct Point { x: i32, y: i32 }` définit une struct
//   - `impl Point { fn distance(&self) -> f64 { ... } }` ajoute une méthode
//   - `Point { x: 1, y: 2 }` crée une instance
//   - `fn new() -> Self { ... }` est une fonction associée conventionnelle
//
// Pour lancer : cargo test --test ex08_structs
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// La struct Person est définie pour toi. Implémente les méthodes ci-dessous.
struct Person {
    name: String,
    age: u32,
}

/// Implémentez une méthode new pour Person.
impl Person {
    fn new(name: String, age: u32) -> Self {
        todo!("Retourne une nouvelle Person")
    }

    /// Implémentez une méthode is_adult qui retourne true si age >= 18.
    fn is_adult(&self) -> bool {
        todo!("Retourne true si age >= 18")
    }

    /// Implémentez une méthode birthday qui augmente age de 1.
    fn birthday(&mut self) {
        todo!("Augmente age de 1")
    }
}

/// La struct Rectangle est définie pour toi. Implémente les méthodes ci-dessous.
struct Rectangle {
    width: f64,
    height: f64,
}

/// Implémentez une méthode area qui retourne la surface.
impl Rectangle {
    fn area(&self) -> f64 {
        todo!("Retourne width * height")
    }

    /// Implémentez une méthode perimeter qui retourne le périmètre.
    fn perimeter(&self) -> f64 {
        todo!("Retourne 2 * (width + height)")
    }
}

/// La struct BankAccount est définie pour toi. Implémente les méthodes ci-dessous.
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    /// Implémentez une fonction associée new avec balance initiale.
    fn new(owner: String, initial_balance: f64) -> Self {
        todo!("Crée un nouveau compte avec owner et initial_balance")
    }

    /// Implémentez deposit qui ajoute un montant.
    fn deposit(&mut self, amount: f64) {
        todo!("Ajoute amount à balance")
    }

    /// Implémentez withdraw qui retire un montant (ne pas retirer si balance insuffisante).
    fn withdraw(&mut self, amount: f64) -> bool {
        todo!("Retire amount si balance >= amount, retourne true si succès")
    }
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_new() {
        let p = Person::new("Alice".to_string(), 25);
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 25);
    }

    #[test]
    fn test_person_is_adult() {
        let adult = Person::new("Bob".to_string(), 18);
        let minor = Person::new("Charlie".to_string(), 15);
        assert_eq!(adult.is_adult(), true);
        assert_eq!(minor.is_adult(), false);
    }

    #[test]
    fn test_person_birthday() {
        let mut p = Person::new("Diana".to_string(), 20);
        p.birthday();
        assert_eq!(p.age, 21);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 5.0, height: 3.0 };
        assert!((rect.area() - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle { width: 5.0, height: 3.0 };
        assert!((rect.perimeter() - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_bank_account_new() {
        let acc = BankAccount::new("Eve".to_string(), 1000.0);
        assert_eq!(acc.owner, "Eve");
        assert_eq!(acc.balance, 1000.0);
    }

    #[test]
    fn test_bank_account_deposit() {
        let mut acc = BankAccount::new("Frank".to_string(), 500.0);
        acc.deposit(200.0);
        assert_eq!(acc.balance, 700.0);
    }

    #[test]
    fn test_bank_account_withdraw() {
        let mut acc = BankAccount::new("Grace".to_string(), 500.0);
        assert_eq!(acc.withdraw(200.0), true);
        assert_eq!(acc.balance, 300.0);
        assert_eq!(acc.withdraw(400.0), false);
        assert_eq!(acc.balance, 300.0);
    }
}
