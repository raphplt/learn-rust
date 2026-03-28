// ============================================================
// Exercice 21 — Threads
// Niveau 9: Concurrence et Parallélisme
// ============================================================
//
// Les threads en Rust permettent l'exécution concurrente.
// thread::spawn lance un nouveau thread, JoinHandle permet de l'attendre.
// Les closures `move` transfèrent la propriété des variables au thread.
// Le compilateur force la thread-safety via les traits Send et Sync.
//
// 💡 Indices:
//   - `thread::spawn(|| { ... })` : lance un thread
//   - `let handle = thread::spawn(move || { ... })` : `move` capture les variables
//   - `handle.join().unwrap()` : attend que le thread finisse
//   - `thread::sleep(Duration::from_millis(100))` : pause
//   - Variables partagées entre threads nécessitent Arc<Mutex<T>>
//   - Les closures dans spawn doivent être `'static` (pas de références temporaires)
//   - `Send` : peut être envoyé à un autre thread
//   - `Sync` : peut être partagé entre threads
//
// Pour lancer : cargo test --test ex21_threads
// ============================================================

#[allow(dead_code)]

use std::thread;
use std::time::Duration;

// ---- Fonctions à compléter ----

/// Lancez un thread simple qui retourne un String.
/// Attendez le thread et retournez son résultat.
fn simple_thread_spawn() -> String {
    todo!("Lancez un thread qui retourne 'Hello from thread', attendez-le")
}

/// Lancez deux threads qui calculent des sommes en parallèle.
/// Attendez les deux et retournez la somme totale.
fn sum_in_threads() -> i32 {
    todo!("Lancez 2 threads, chacun calcule une somme partielle, retournez le total")
}

/// Lancez un thread qui duplique un nombre trois fois.
/// Le thread doit capturer le nombre avec `move`.
fn thread_with_move() -> Vec<i32> {
    todo!("Lancez un thread qui ajoute un nombre 3 fois à un Vec et le retourne")
}

/// Lancez plusieurs threads et collectez leurs résultats dans un Vec.
fn multiple_threads() -> Vec<String> {
    todo!("Lancez 4 threads numérotés, collectez leurs messages de String")
}

/// Lancez un thread qui dort, puis attendez-le avec un timeout.
fn thread_sleep_and_wait() -> bool {
    todo!("Lancez un thread qui dort 50ms, attendez-le, retournez true")
}

/// Lancez un thread pour chaque élément d'un vecteur et collectez les carrés.
fn map_with_threads(v: &[i32]) -> Vec<i32> {
    todo!("Lancez un thread par élément, retournez les carrés")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_thread_spawn() {
        let result = simple_thread_spawn();
        assert!(result.contains("Hello from thread"));
    }

    #[test]
    fn test_sum_in_threads() {
        let result = sum_in_threads();
        assert_eq!(result, 13); // (1+2+3) + (4+5+1) ou similaire
    }

    #[test]
    fn test_thread_with_move() {
        let result = thread_with_move();
        assert_eq!(result.len(), 3);
        // Vérifie que les trois éléments sont les mêmes
        assert_eq!(result[0], result[1]);
        assert_eq!(result[1], result[2]);
    }

    #[test]
    fn test_multiple_threads() {
        let results = multiple_threads();
        assert_eq!(results.len(), 4);
        // Chaque thread a retourné un message
        for msg in &results {
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn test_thread_sleep_and_wait() {
        let result = thread_sleep_and_wait();
        assert!(result);
    }

    #[test]
    fn test_map_with_threads() {
        let v = vec![1, 2, 3, 4];
        let results = map_with_threads(&v);
        assert_eq!(results.len(), 4);
        // Vérifie les carrés (pas d'ordre garanti)
        assert!(results.contains(&1));
        assert!(results.contains(&4));
        assert!(results.contains(&9));
        assert!(results.contains(&16));
    }

    #[test]
    fn test_map_with_threads_empty() {
        let v: Vec<i32> = vec![];
        let results = map_with_threads(&v);
        assert!(results.is_empty());
    }

    #[test]
    fn test_map_with_threads_negative() {
        let v = vec![-1, 2, -3];
        let results = map_with_threads(&v);
        assert_eq!(results.len(), 3);
        assert!(results.contains(&1));   // (-1)^2
        assert!(results.contains(&4));   // 2^2
        assert!(results.contains(&9));   // (-3)^2
    }

    #[test]
    fn test_thread_spawning_multiple() {
        // Test que les threads s'exécutent vraiment en parallèle
        let start = std::time::Instant::now();

        let handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(10));
                    i
                })
            })
            .collect();

        for handle in handles {
            let _ = handle.join();
        }

        let elapsed = start.elapsed();
        // Si vraiment parallèle, ~10ms. Si séquentiel, ~30ms.
        // On teste que ce n'est pas ridiculement long
        assert!(elapsed.as_millis() < 100);
    }
}
