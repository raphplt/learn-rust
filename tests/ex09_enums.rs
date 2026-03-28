// ============================================================
// Exercice 09 — Enums & Pattern Matching
// Niveau 3: Énumérations & Déstructuration
// ============================================================
//
// Les enums définissent un type avec plusieurs variantes possibles.
// Chaque variante peut avoir des données associées.
// Le pattern matching avec `match` permet d'utiliser chaque variante.
// `if let` est un sucre syntaxique pour match avec une seule branche.
//
// 💡 Indices:
//   - `enum Color { Red, Green, Blue }` définit un enum simple
//   - `enum Result<T, E> { Ok(T), Err(E) }` a des variantes avec données
//   - `match value { Pattern::Variant => action, _ => default }` déstructure
//   - `if let Pattern::Variant = value { ... }` pour une seule branche
//
// Pour lancer : cargo test --test ex09_enums
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Définissez un enum Direction avec quatre variantes: North, South, East, West.
enum Direction {
    todo_enum_variants: u32, // Remplace-moi
}

/// Écrivez une fonction qui prend un Direction et retourne son opposé.
fn opposite_direction(d: Direction) -> Direction {
    todo!("Retourne la direction opposée")
}

/// Définissez un enum Message avec variantes:
/// - Quit (pas de données)
/// - Move { x: i32, y: i32 }
/// - Write(String)
/// - ChangeColor(u8, u8, u8)
enum Message {
    todo_enum_variants: u32, // Remplace-moi
}

/// Écrivez une fonction qui prend un Message et retourne sa "description" sous forme de chaîne.
fn message_description(msg: Message) -> String {
    todo!("Retourne une description du Message selon sa variante")
}

/// Définissez un enum Result custom (ou utilisez std::result::Result si préféré):
/// Success(T) ou Error(String)
/// Écrivez une fonction qui prend Result<i32, String> et retourne la valeur ou 0 en cas d'erreur.
fn unwrap_or_default(result: Result<i32, String>) -> i32 {
    todo!("Retourne la valeur ou 0 en cas d'erreur")
}

/// Définissez un enum Status: Active(String), Inactive, Suspended { reason: String }.
/// Écrivez une fonction qui retourne true si le status est Active.
enum Status {
    todo_enum_variants: u32, // Remplace-moi
}

fn is_active(status: Status) -> bool {
    todo!("Retourne true si le status est Active")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite_direction() {
        let north = Direction::North;
        let opposite = opposite_direction(north);
        match opposite {
            Direction::South => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_message_description() {
        let msg = Message::Write("Hello".to_string());
        let desc = message_description(msg);
        assert!(desc.contains("Hello"));

        let msg2 = Message::Move { x: 5, y: 10 };
        let desc2 = message_description(msg2);
        assert!(desc2.contains("5") && desc2.contains("10"));
    }

    #[test]
    fn test_unwrap_or_default() {
        let success: Result<i32, String> = Ok(42);
        assert_eq!(unwrap_or_default(success), 42);

        let error: Result<i32, String> = Err("error".to_string());
        assert_eq!(unwrap_or_default(error), 0);
    }

    #[test]
    fn test_is_active() {
        let active = Status::Active("running".to_string());
        assert_eq!(is_active(active), true);

        let inactive = Status::Inactive;
        assert_eq!(is_active(inactive), false);

        let suspended = Status::Suspended { reason: "maintenance".to_string() };
        assert_eq!(is_active(suspended), false);
    }
}
