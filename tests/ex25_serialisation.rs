// ============================================================
// Exercice 25 — Sérialisation & Fichiers
// Niveau 10: Serde, JSON & Gestion de Données
// ============================================================
//
// La sérialisation est un concept clé dans les bases de données:
// transformer des structures de données internes en format échangeable (JSON, binaire, etc.)
// et vice-versa. C'est nécessaire pour:
//   - Persister les données sur disque
//   - Communiquer entre processus
//   - Exporter/importer des données
//
// Cet exercice utilise serde_json pour sérialiser/désérialiser des Record.
// Dans qoredb, ce pattern est utilisé pour stocker les enregistrements et les index.
//
// 💡 Indices:
//   - `#[derive(Serialize, Deserialize)]` : dérive auto les traits serde
//   - `serde_json::to_string(obj)` : sérialise en JSON string
//   - `serde_json::from_str::<T>(json)` : désérialise depuis JSON string
//   - `fmt::Display` : implémente l'affichage personnalisé (println!, format!)
//   - Closure et iterator: `.iter().filter().collect()`
//   - `.zip()` : combine deux itérateurs
//   - Merge de vecteurs triés: algorithme classique
//
// Pour lancer : cargo test --test ex25_serialisation
// ============================================================

#[allow(dead_code)]

use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

// ---- Struct Record ----

/// Représente un enregistrement (record) dans une base de données.
/// Champ clé: timestamp pour le contrôle de concurrence optimiste (MVCC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub key: String,
    pub value: String,
    pub timestamp: u64,
}

// ---- Impl Display pour Record ----

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implémenter Display pour afficher le record de manière lisible")
    }
}

// ---- Fonctions à compléter ----

/// Sérialise une liste de Record en JSON.
/// Doit retourner une chaîne JSON valide, pas un Result.
/// Si la sérialisation échoue, panicz (il y a un bug).
///
/// Exemple de format attendu:
/// ```json
/// [
///   {"id":1,"key":"user","value":"alice","timestamp":1000},
///   {"id":2,"key":"role","value":"admin","timestamp":2000}
/// ]
/// ```
pub fn serialize_records(records: &[Record]) -> String {
    todo!("Sérialiser les records en JSON avec serde_json::to_string_pretty ou to_string")
}

/// Désérialise une chaîne JSON en vecteur de Record.
/// Retourne Result pour gérer les erreurs de parsing.
/// En cas d'erreur, retournez Err(String) avec un message explicite.
pub fn deserialize_records(json: &str) -> Result<Vec<Record>, String> {
    todo!("Désérialiser le JSON en Vec<Record> avec gestion d'erreur")
}

/// Filtre les records par préfixe de clé.
/// Retourne un vecteur de références aux records matchés.
/// Important: maintenir l'ordre original.
///
/// Exemple:
/// Records: [("user_1", "alice"), ("user_2", "bob"), ("admin", "root")]
/// Filtre: "user" → [("user_1", "alice"), ("user_2", "bob")]
pub fn filter_records<'a>(records: &'a [Record], prefix: &str) -> Vec<&'a Record> {
    todo!("Filtrer les records dont la clé commence par prefix")
}

/// Fusionne deux slices triées par id en un seul vecteur trié.
/// Règle de fusion: si un id apparaît dans les deux, garder le record avec le timestamp le plus élevé.
/// C'est un pattern courant dans les systèmes avec versioning (MVCC, Git, etc.).
///
/// Exemple:
/// a = [(id=1, ts=100), (id=3, ts=300), (id=5, ts=500)]
/// b = [(id=2, ts=200), (id=3, ts=400), (id=4, ts=400)]
/// Résultat = [(id=1, ts=100), (id=2, ts=200), (id=3, ts=400), (id=4, ts=400), (id=5, ts=500)]
/// Note: id=3 a ts=400 > ts=300, donc on garde la version de b.
pub fn merge_records(a: &[Record], b: &[Record]) -> Vec<Record> {
    todo!("Fusionner deux slices triées par id")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: créer un record
    fn make_record(id: u64, key: &str, value: &str, timestamp: u64) -> Record {
        Record {
            id,
            key: key.to_string(),
            value: value.to_string(),
            timestamp,
        }
    }

    // Tests Display
    #[test]
    fn test_record_display() {
        let record = make_record(1, "user", "alice", 1000);
        let displayed = format!("{}", record);
        // La sortie doit contenir les infos clés
        assert!(displayed.contains("user") || displayed.contains("alice"));
    }

    #[test]
    fn test_record_display_all_fields() {
        let record = make_record(42, "key", "value", 5000);
        let displayed = format!("{}", record);
        // Vérifier que les champs importants sont présents
        assert!(displayed.contains("42") || displayed.contains("key") || displayed.contains("value"));
    }

    // Tests serialize_records
    #[test]
    fn test_serialize_single_record() {
        let records = vec![make_record(1, "user", "alice", 1000)];
        let json = serialize_records(&records);

        // Le JSON doit contenir des informations valides
        assert!(json.contains("user"));
        assert!(json.contains("alice"));
    }

    #[test]
    fn test_serialize_multiple_records() {
        let records = vec![
            make_record(1, "user", "alice", 1000),
            make_record(2, "role", "admin", 2000),
        ];
        let json = serialize_records(&records);

        // Vérifier la structure
        assert!(json.contains("user"));
        assert!(json.contains("alice"));
        assert!(json.contains("role"));
        assert!(json.contains("admin"));
    }

    #[test]
    fn test_serialize_empty_records() {
        let records: Vec<Record> = vec![];
        let json = serialize_records(&records);

        // Le JSON doit être un array valide (vide)
        assert!(json.contains("[]") || json.contains("[ ]"));
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = vec![
            make_record(1, "key1", "value1", 100),
            make_record(2, "key2", "value2", 200),
        ];

        let json = serialize_records(&original);
        let deserialized = deserialize_records(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Tests deserialize_records
    #[test]
    fn test_deserialize_valid_json() {
        let json = r#"[{"id":1,"key":"user","value":"alice","timestamp":1000}]"#;
        let result = deserialize_records(json);

        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, 1);
        assert_eq!(records[0].key, "user");
    }

    #[test]
    fn test_deserialize_multiple_records() {
        let json = r#"[
            {"id":1,"key":"user","value":"alice","timestamp":1000},
            {"id":2,"key":"role","value":"admin","timestamp":2000}
        ]"#;
        let result = deserialize_records(json);

        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_deserialize_empty_json() {
        let json = "[]";
        let result = deserialize_records(json);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let json = "invalid json";
        let result = deserialize_records(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_malformed_record() {
        let json = r#"[{"id":"not_a_number"}]"#;
        let result = deserialize_records(json);

        assert!(result.is_err());
    }

    // Tests filter_records
    #[test]
    fn test_filter_records_by_prefix() {
        let records = vec![
            make_record(1, "user_1", "alice", 100),
            make_record(2, "user_2", "bob", 200),
            make_record(3, "admin", "root", 300),
        ];

        let filtered = filter_records(&records, "user");
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].key, "user_1");
        assert_eq!(filtered[1].key, "user_2");
    }

    #[test]
    fn test_filter_records_no_match() {
        let records = vec![
            make_record(1, "user_1", "alice", 100),
            make_record(2, "user_2", "bob", 200),
        ];

        let filtered = filter_records(&records, "admin");
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_records_all_match() {
        let records = vec![
            make_record(1, "test_a", "value", 100),
            make_record(2, "test_b", "value", 200),
            make_record(3, "test_c", "value", 300),
        ];

        let filtered = filter_records(&records, "test");
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_filter_records_empty() {
        let records: Vec<Record> = vec![];
        let filtered = filter_records(&records, "any");

        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_records_preserves_order() {
        let records = vec![
            make_record(5, "key_5", "v5", 500),
            make_record(1, "key_1", "v1", 100),
            make_record(3, "key_3", "v3", 300),
        ];

        let filtered = filter_records(&records, "key");
        // L'ordre original doit être maintenu
        assert_eq!(filtered[0].id, 5);
        assert_eq!(filtered[1].id, 1);
        assert_eq!(filtered[2].id, 3);
    }

    // Tests merge_records
    #[test]
    fn test_merge_records_simple() {
        let a = vec![make_record(1, "a", "v1", 100)];
        let b = vec![make_record(2, "b", "v2", 200)];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].id, 1);
        assert_eq!(merged[1].id, 2);
    }

    #[test]
    fn test_merge_records_interleaved() {
        let a = vec![
            make_record(1, "a", "v1", 100),
            make_record(3, "c", "v3", 300),
        ];
        let b = vec![
            make_record(2, "b", "v2", 200),
            make_record(4, "d", "v4", 400),
        ];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 4);
        assert_eq!(merged[0].id, 1);
        assert_eq!(merged[1].id, 2);
        assert_eq!(merged[2].id, 3);
        assert_eq!(merged[3].id, 4);
    }

    #[test]
    fn test_merge_records_duplicate_id_higher_timestamp_wins() {
        let a = vec![make_record(1, "key", "v_old", 100)];
        let b = vec![make_record(1, "key", "v_new", 500)];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].value, "v_new");
        assert_eq!(merged[0].timestamp, 500);
    }

    #[test]
    fn test_merge_records_duplicate_id_lower_timestamp_loses() {
        let a = vec![make_record(1, "key", "v_new", 500)];
        let b = vec![make_record(1, "key", "v_old", 100)];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].value, "v_new");
        assert_eq!(merged[0].timestamp, 500);
    }

    #[test]
    fn test_merge_records_all_duplicates() {
        let a = vec![
            make_record(1, "a", "a_v1", 100),
            make_record(2, "b", "b_v1", 200),
        ];
        let b = vec![
            make_record(1, "a", "a_v2", 150),
            make_record(2, "b", "b_v2", 150),
        ];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].value, "a_v2"); // ts 150 > 100
        assert_eq!(merged[1].value, "b_v1"); // ts 200 > 150
    }

    #[test]
    fn test_merge_records_empty_a() {
        let a: Vec<Record> = vec![];
        let b = vec![
            make_record(1, "a", "v1", 100),
            make_record(2, "b", "v2", 200),
        ];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].id, 1);
        assert_eq!(merged[1].id, 2);
    }

    #[test]
    fn test_merge_records_empty_b() {
        let a = vec![
            make_record(1, "a", "v1", 100),
            make_record(2, "b", "v2", 200),
        ];
        let b: Vec<Record> = vec![];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].id, 1);
        assert_eq!(merged[1].id, 2);
    }

    #[test]
    fn test_merge_records_both_empty() {
        let a: Vec<Record> = vec![];
        let b: Vec<Record> = vec![];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 0);
    }

    #[test]
    fn test_merge_records_complex_scenario() {
        // Scénario réaliste: fusion de deux snapshots de base de données
        let a = vec![
            make_record(1, "user_1", "alice", 1000),
            make_record(3, "user_3", "charlie", 1200),
            make_record(5, "user_5", "eve", 1400),
        ];
        let b = vec![
            make_record(2, "user_2", "bob", 1100),
            make_record(3, "user_3", "charlie_updated", 1300), // plus récent
            make_record(4, "user_4", "david", 1350),
        ];

        let merged = merge_records(&a, &b);
        assert_eq!(merged.len(), 5);

        // Vérifier l'ordre
        assert_eq!(merged[0].id, 1);
        assert_eq!(merged[1].id, 2);
        assert_eq!(merged[2].id, 3);
        assert_eq!(merged[3].id, 4);
        assert_eq!(merged[4].id, 5);

        // Vérifier que le record de user_3 est à jour
        assert_eq!(merged[2].value, "charlie_updated");
        assert_eq!(merged[2].timestamp, 1300);
    }

    #[test]
    fn test_merge_results_in_sorted_output() {
        let a = vec![
            make_record(5, "a", "v5", 500),
            make_record(10, "c", "v10", 1000),
        ];
        let b = vec![
            make_record(3, "b", "v3", 300),
            make_record(8, "d", "v8", 800),
        ];

        let merged = merge_records(&a, &b);
        // Le résultat doit être trié par id
        for i in 1..merged.len() {
            assert!(merged[i - 1].id < merged[i].id);
        }
    }
}
