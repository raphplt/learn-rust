// ============================================================
// Exercice 24 — Parser de Commandes
// Niveau 10: Enums, Pattern Matching & Parsing
// ============================================================
//
// Un parser est un composant critique dans les systèmes de bases de données.
// Il transforme une chaîne d'entrée (commande utilisateur) en une structure de données typée.
//
// Exemple: "SET mykey myvalue" → Command::Set("mykey".to_string(), "myvalue".to_string())
//
// Ce pattern est fondamental dans qoredb pour interpréter les requêtes de l'utilisateur
// et les transformer en opérations internes.
//
// Votre parser doit:
//   - Reconnaître les commandes: GET, SET, DEL, LIST, COUNT, QUIT
//   - Gérer l'indentation et la casse
//   - Extraire les arguments (clés, valeurs)
//   - Retourner Command::Unknown pour les commandes invalides
//
// 💡 Indices:
//   - `str.trim()` : supprime les espaces en début/fin
//   - `str.split_whitespace()` : itérateur sur les mots
//   - `to_uppercase()` : convertit en majuscules
//   - `.collect::<Vec<_>>()` : collecte un itérateur en vecteur
//   - Pattern matching: `match command_type { ... }`
//   - `String::from()` ou `.to_string()` : créer des String
//
// Pour lancer : cargo test --test ex24_parser
// ============================================================

#[allow(dead_code)]

// ---- Enum Command ----

/// Représente une commande parsée.
/// Les commandes sont l'interface entre l'utilisateur et le système.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    /// GET key : récupère la valeur associée à une clé
    Get(String),
    /// SET key value : définit une clé-valeur
    Set(String, String),
    /// DEL key : supprime une clé
    Delete(String),
    /// LIST : affiche toutes les clés
    List,
    /// COUNT : compte le nombre de paires clé-valeur
    Count,
    /// QUIT : quitter le programme
    Quit,
    /// Commande non reconnue
    Unknown(String),
}

// ---- Fonctions à compléter ----

/// Parsez une chaîne d'entrée et retournez la commande correspondante.
///
/// Règles:
///   - Les commandes sont case-insensitive (GET, get, Get sont équivalents)
///   - Les espaces supplémentaires sont ignorés (trim, split_whitespace)
///   - GET et DEL nécessitent exactement 1 argument (la clé)
///   - SET nécessite exactement 2 arguments (clé et valeur)
///   - LIST, COUNT, QUIT ne prennent aucun argument
///   - Si le nombre d'arguments est incorrect, retournez Unknown
///
/// Exemples:
///   - "GET mykey" → Command::Get("mykey")
///   - "SET user alice" → Command::Set("user", "alice")
///   - "  DEL  unused  " → Command::Delete("unused")
///   - "LIST" → Command::List
///   - "INVALID arg" → Command::Unknown("INVALID arg")
pub fn parse_command(input: &str) -> Command {
    todo!("Implémenter le parser de commandes")
}

/// Formatez une commande en une réponse lisible pour l'utilisateur.
/// C'est l'inverse du parsing : Command → String formatée.
///
/// Exemples:
///   - Command::Get("user") → "GET user"
///   - Command::Set("id", "123") → "SET id 123"
///   - Command::List → "LIST"
///   - Command::Unknown("foo") → "Unknown command: foo"
pub fn format_response(cmd: &Command) -> String {
    todo!("Formatter la commande en chaîne lisible")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    // Tests de parsing GET
    #[test]
    fn test_parse_get_simple() {
        let cmd = parse_command("GET mykey");
        assert_eq!(cmd, Command::Get("mykey".to_string()));
    }

    #[test]
    fn test_parse_get_with_whitespace() {
        let cmd = parse_command("  GET   somekey  ");
        assert_eq!(cmd, Command::Get("somekey".to_string()));
    }

    #[test]
    fn test_parse_get_case_insensitive() {
        assert_eq!(parse_command("get key"), Command::Get("key".to_string()));
        assert_eq!(parse_command("GeT key"), Command::Get("key".to_string()));
        assert_eq!(parse_command("GET key"), Command::Get("key".to_string()));
    }

    #[test]
    fn test_parse_get_no_arg_returns_unknown() {
        let cmd = parse_command("GET");
        assert_eq!(cmd, Command::Unknown("GET".to_string()));
    }

    #[test]
    fn test_parse_get_too_many_args_returns_unknown() {
        let cmd = parse_command("GET key extra");
        assert_eq!(cmd, Command::Unknown("GET key extra".to_string()));
    }

    // Tests de parsing SET
    #[test]
    fn test_parse_set_simple() {
        let cmd = parse_command("SET user alice");
        assert_eq!(cmd, Command::Set("user".to_string(), "alice".to_string()));
    }

    #[test]
    fn test_parse_set_case_insensitive() {
        assert_eq!(
            parse_command("set key value"),
            Command::Set("key".to_string(), "value".to_string())
        );
        assert_eq!(
            parse_command("SeT k v"),
            Command::Set("k".to_string(), "v".to_string())
        );
    }

    #[test]
    fn test_parse_set_with_whitespace() {
        let cmd = parse_command("  SET   id   12345  ");
        assert_eq!(cmd, Command::Set("id".to_string(), "12345".to_string()));
    }

    #[test]
    fn test_parse_set_missing_value_returns_unknown() {
        let cmd = parse_command("SET key");
        assert_eq!(cmd, Command::Unknown("SET key".to_string()));
    }

    #[test]
    fn test_parse_set_too_many_args_returns_unknown() {
        let cmd = parse_command("SET key value extra");
        assert_eq!(cmd, Command::Unknown("SET key value extra".to_string()));
    }

    // Tests de parsing DEL
    #[test]
    fn test_parse_delete_simple() {
        let cmd = parse_command("DEL mykey");
        assert_eq!(cmd, Command::Delete("mykey".to_string()));
    }

    #[test]
    fn test_parse_delete_case_insensitive() {
        assert_eq!(
            parse_command("del key"),
            Command::Delete("key".to_string())
        );
        assert_eq!(
            parse_command("DeL key"),
            Command::Delete("key".to_string())
        );
    }

    #[test]
    fn test_parse_delete_no_arg_returns_unknown() {
        let cmd = parse_command("DEL");
        assert_eq!(cmd, Command::Unknown("DEL".to_string()));
    }

    // Tests de parsing LIST, COUNT, QUIT
    #[test]
    fn test_parse_list() {
        let cmd = parse_command("LIST");
        assert_eq!(cmd, Command::List);
    }

    #[test]
    fn test_parse_list_case_insensitive() {
        assert_eq!(parse_command("list"), Command::List);
        assert_eq!(parse_command("LiSt"), Command::List);
    }

    #[test]
    fn test_parse_list_with_whitespace() {
        let cmd = parse_command("  LIST  ");
        assert_eq!(cmd, Command::List);
    }

    #[test]
    fn test_parse_count() {
        let cmd = parse_command("COUNT");
        assert_eq!(cmd, Command::Count);
    }

    #[test]
    fn test_parse_count_case_insensitive() {
        assert_eq!(parse_command("count"), Command::Count);
        assert_eq!(parse_command("CoUnT"), Command::Count);
    }

    #[test]
    fn test_parse_quit() {
        let cmd = parse_command("QUIT");
        assert_eq!(cmd, Command::Quit);
    }

    #[test]
    fn test_parse_quit_case_insensitive() {
        assert_eq!(parse_command("quit"), Command::Quit);
        assert_eq!(parse_command("QuIt"), Command::Quit);
    }

    // Tests de parsing invalides
    #[test]
    fn test_parse_unknown_command() {
        let cmd = parse_command("UNKNOWN");
        assert_eq!(cmd, Command::Unknown("UNKNOWN".to_string()));
    }

    #[test]
    fn test_parse_invalid_with_args() {
        let cmd = parse_command("INVALID arg1 arg2");
        assert_eq!(cmd, Command::Unknown("INVALID arg1 arg2".to_string()));
    }

    #[test]
    fn test_parse_empty_string() {
        let cmd = parse_command("");
        assert_eq!(cmd, Command::Unknown("".to_string()));
    }

    // Tests de format_response
    #[test]
    fn test_format_get() {
        let cmd = Command::Get("user".to_string());
        assert_eq!(format_response(&cmd), "GET user");
    }

    #[test]
    fn test_format_set() {
        let cmd = Command::Set("id".to_string(), "42".to_string());
        assert_eq!(format_response(&cmd), "SET id 42");
    }

    #[test]
    fn test_format_delete() {
        let cmd = Command::Delete("key".to_string());
        assert_eq!(format_response(&cmd), "DELETE key");
    }

    #[test]
    fn test_format_list() {
        let cmd = Command::List;
        assert_eq!(format_response(&cmd), "LIST");
    }

    #[test]
    fn test_format_count() {
        let cmd = Command::Count;
        assert_eq!(format_response(&cmd), "COUNT");
    }

    #[test]
    fn test_format_quit() {
        let cmd = Command::Quit;
        assert_eq!(format_response(&cmd), "QUIT");
    }

    #[test]
    fn test_format_unknown() {
        let cmd = Command::Unknown("BADCMD arg".to_string());
        let formatted = format_response(&cmd);
        assert!(formatted.contains("Unknown") || formatted.contains("BADCMD"));
    }

    // Tests d'intégration : parse → format
    #[test]
    fn test_parse_format_roundtrip_get() {
        let input = "GET mykey";
        let cmd = parse_command(input);
        let formatted = format_response(&cmd);
        assert_eq!(formatted, "GET mykey");
    }

    #[test]
    fn test_parse_format_roundtrip_set() {
        let cmd = parse_command("SET name john");
        let formatted = format_response(&cmd);
        assert_eq!(formatted, "SET name john");
    }

    #[test]
    fn test_multiple_command_parsing() {
        let commands = vec![
            "GET user_1",
            "SET config debug",
            "DEL temp",
            "LIST",
            "COUNT",
            "QUIT",
        ];

        let parsed: Vec<_> = commands.iter().map(|c| parse_command(c)).collect();

        assert_eq!(parsed[0], Command::Get("user_1".to_string()));
        assert_eq!(parsed[1], Command::Set("config".to_string(), "debug".to_string()));
        assert_eq!(parsed[2], Command::Delete("temp".to_string()));
        assert_eq!(parsed[3], Command::List);
        assert_eq!(parsed[4], Command::Count);
        assert_eq!(parsed[5], Command::Quit);
    }
}
