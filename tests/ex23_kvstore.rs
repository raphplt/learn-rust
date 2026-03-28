// ============================================================
// Exercice 23 — Mini Key-Value Store
// Niveau 10: Structures de Données & Collections
// ============================================================
//
// Un key-value store (KV store) est une structure fondamentale en systèmes.
// C'est le cœur de nombreuses bases de données (Redis, DynamoDB, etc.).
//
// Votre KvStore doit:
//   - Stocker des paires (String, String) dans une HashMap
//   - Supporter les opérations CRUD: new, set, get, delete
//   - Retourner des clés triées avec keys()
//   - Gérer les cas limites (clé inexistante, update vs insert, etc.)
//
// Ce pattern est utilisé dans qoredb pour la gestion d'index et d'index primaires.
//
// 💡 Indices:
//   - `use std::collections::HashMap;`
//   - `HashMap::new()` : crée une HashMap vide
//   - `map.insert(key, value)` : insert ou remplace, retourne Option<V>
//   - `map.get(key)` : retourne Option<&V>
//   - `map.remove(key)` : remove et retourne Option<V>
//   - `map.contains_key(key)` : vérification rapide
//   - `map.keys()` : itérateur sur les clés
//   - `map.len()` : nombre d'éléments
//   - `Option<T>` : Some(val) ou None
//
// Pour lancer : cargo test --test ex23_kvstore
// ============================================================

#[allow(dead_code)]

use std::collections::HashMap;

// ---- Struct à compléter ----

/// Un simple in-memory key-value store utilisant HashMap.
/// Pattern courant dans les systèmes d'index et de cache.
#[derive(Debug, Clone)]
pub struct KvStore {
    todo!("Ajouter un champ HashMap<String, String>")
}

impl KvStore {
    /// Créez une nouvelle instance vide du KvStore.
    pub fn new() -> Self {
        todo!("Retourner une instance avec une HashMap vide")
    }

    /// Ajoutez ou remplacez une clé-valeur.
    /// Retourne None si c'est une nouvelle insertion, Some(ancienne_valeur) sinon.
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        todo!("Insérer la paire (key, value) dans la map")
    }

    /// Récupérez la valeur associée à une clé.
    /// Retourne Option<&String> : Some(&value) si existe, None sinon.
    pub fn get(&self, key: &str) -> Option<&String> {
        todo!("Retourner une référence à la valeur, ou None")
    }

    /// Supprimez une clé et retournez sa valeur.
    /// Retourne true si la clé existait, false sinon.
    pub fn delete(&mut self, key: &str) -> bool {
        todo!("Supprimer la clé et retourner si elle existait")
    }

    /// Retournez un vecteur de toutes les clés, **triées alphabétiquement**.
    /// Important: les clés doivent être triées pour la cohérence.
    pub fn keys(&self) -> Vec<String> {
        todo!("Récupérer toutes les clés, les trier, et les retourner")
    }

    /// Retournez le nombre de paires clé-valeur.
    pub fn len(&self) -> usize {
        todo!("Retourner la taille de la map")
    }

    /// Vérifiez si le store est vide.
    pub fn is_empty(&self) -> bool {
        todo!("Retourner true si vide, false sinon")
    }

    /// Mettez à jour une clé existante ou insérez-la si elle n'existe pas.
    /// Retourne Some(ancienne_valeur) si mise à jour, None si insertion.
    /// C'est un pattern courant dans les bases de données (UPSERT).
    pub fn update_or_insert(&mut self, key: String, value: String) -> Option<String> {
        todo!("Utiliser insert et gérer le retour")
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_kvstore_is_empty() {
        let store = KvStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::new();
        store.set("user_1".to_string(), "Alice".to_string());

        let result = store.get("user_1");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "Alice");
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KvStore::new();
        assert!(store.get("nonexistent").is_none());
    }

    #[test]
    fn test_set_returns_old_value() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "value1".to_string());

        let old = store.set("key".to_string(), "value2".to_string());
        assert_eq!(old, Some("value1".to_string()));

        let current = store.get("key").unwrap();
        assert_eq!(current, "value2");
    }

    #[test]
    fn test_set_new_returns_none() {
        let mut store = KvStore::new();
        let result = store.set("newkey".to_string(), "newvalue".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "value".to_string());

        let deleted = store.delete("key");
        assert!(deleted);
        assert!(store.get("key").is_none());
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut store = KvStore::new();
        let deleted = store.delete("nonexistent");
        assert!(!deleted);
    }

    #[test]
    fn test_delete_updates_len() {
        let mut store = KvStore::new();
        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());
        assert_eq!(store.len(), 2);

        store.delete("a");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_keys_returns_sorted() {
        let mut store = KvStore::new();
        store.set("zebra".to_string(), "1".to_string());
        store.set("apple".to_string(), "2".to_string());
        store.set("mango".to_string(), "3".to_string());

        let keys = store.keys();
        assert_eq!(keys, vec!["apple", "mango", "zebra"]);
    }

    #[test]
    fn test_keys_empty_store() {
        let store = KvStore::new();
        assert_eq!(store.keys(), vec![] as Vec<String>);
    }

    #[test]
    fn test_update_or_insert_new_key() {
        let mut store = KvStore::new();
        let result = store.update_or_insert("new_key".to_string(), "new_value".to_string());

        assert!(result.is_none());
        assert_eq!(store.get("new_key").unwrap(), "new_value");
    }

    #[test]
    fn test_update_or_insert_existing_key() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "old_value".to_string());

        let result = store.update_or_insert("key".to_string(), "new_value".to_string());

        assert_eq!(result, Some("old_value".to_string()));
        assert_eq!(store.get("key").unwrap(), "new_value");
    }

    #[test]
    fn test_multiple_operations_sequence() {
        let mut store = KvStore::new();

        // Insert 3 items
        store.set("id_1".to_string(), "user_alice".to_string());
        store.set("id_2".to_string(), "user_bob".to_string());
        store.set("id_3".to_string(), "user_charlie".to_string());

        assert_eq!(store.len(), 3);
        assert!(!store.is_empty());

        // Update one
        let old = store.set("id_2".to_string(), "user_bob_updated".to_string());
        assert_eq!(old, Some("user_bob".to_string()));

        // Delete one
        assert!(store.delete("id_1"));
        assert_eq!(store.len(), 2);

        // Check sorted keys
        let keys = store.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys[0] < keys[1]); // Vérifier l'ordre alphabétique
    }

    #[test]
    fn test_kvstore_default() {
        let store = KvStore::default();
        assert!(store.is_empty());
    }

    #[test]
    fn test_kvstore_clone() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "value".to_string());

        let cloned = store.clone();
        assert_eq!(cloned.get("key").unwrap(), "value");
    }
}
