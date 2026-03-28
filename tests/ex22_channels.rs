// ============================================================
// Exercice 22 — Channels & Mutex
// Niveau 9: Synchronisation entre Threads
// ============================================================
//
// Les channels (mpsc::channel) permettent la communication entre threads.
// Un Sender peut envoyer des messages, un Receiver les reçoit.
// Mutex<T> synchronise l'accès à une ressource partagée.
// Arc<Mutex<T>> combine propriété partagée thread-safe et synchronisation.
//
// 💡 Indices:
//   - `let (tx, rx) = mpsc::channel()` : création d'un channel
//   - `tx.send(value)` : envoie un message (consume le sender si dernière copie)
//   - `rx.recv()` : attend un message (blocking)
//   - `rx.try_recv()` : non-blocking, retourne Result
//   - `Mutex::new(value)` : création d'un mutex
//   - `mutex.lock().unwrap()` : verrouille (mutable borrow)
//   - `Arc<Mutex<T>>` : partagé entre threads + synchronisé
//   - `std::sync::mpsc::channel()` : message passing
//
// Pour lancer : cargo test --test ex22_channels
// ============================================================

#[allow(dead_code)]

use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;

// ---- Fonctions à compléter ----

/// Créez un channel, envoyez un nombre, et recevez-le.
fn simple_channel_send_recv() -> i32 {
    todo!("Créez un channel, envoyez 42, recevez et retournez")
}

/// Lancez un thread qui envoie trois nombres via un channel.
/// Dans le thread principal, collectez les nombres reçus.
fn channel_from_thread() -> Vec<i32> {
    todo!("Lancez un thread qui envoie 1, 2, 3. Recevez-les dans le main thread")
}

/// Lancez plusieurs threads (producteurs) qui envoient tous via un channel partagé.
/// Collectez tous les messages.
fn multiple_producers() -> Vec<i32> {
    todo!("Lancez 3 threads qui envoient chacun 2 nombres via le même channel")
}

/// Utilisez un Mutex pour protéger un compteur partagé.
/// Deux threads l'incrémentent chacun 50 fois.
fn mutex_shared_counter() -> i32 {
    todo!("Utilisez Arc<Mutex<i32>>, lancez 2 threads qui l'incrémentent")
}

/// Créez un channel, lancez un thread qui envoie 1, 2, 3 avec délais.
/// Dans le main, itérez sur le receiver.
fn channel_iterator_pattern() -> Vec<i32> {
    todo!("Itérez sur le receiver pour collecter les messages")
}

/// Utilisez Mutex et channel ensemble :
/// Un thread modifie une valeur protégée par Mutex et envoie un message après.
fn mutex_and_channel() -> i32 {
    todo!("Lancez un thread qui modifie un Mutex<i32> et envoie un signal")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_channel_send_recv() {
        let result = simple_channel_send_recv();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_channel_from_thread() {
        let results = channel_from_thread();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 2);
        assert_eq!(results[2], 3);
    }

    #[test]
    fn test_multiple_producers() {
        let results = multiple_producers();
        // 3 threads * 2 nombres chacun = 6 messages
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn test_multiple_producers_sum() {
        let results = multiple_producers();
        let sum: i32 = results.iter().sum();
        // Somme dépend de la numérotation (0+1)+(1+2)+(2+3) = 9 ou similaire
        assert!(sum > 0);
    }

    #[test]
    fn test_mutex_shared_counter() {
        let result = mutex_shared_counter();
        assert_eq!(result, 100); // 50 + 50
    }

    #[test]
    fn test_mutex_correctness() {
        let counter = Arc::new(Mutex::new(0));
        let c1 = Arc::clone(&counter);
        let c2 = Arc::clone(&counter);

        let h1 = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = c1.lock().unwrap();
                *num += 1;
            }
        });

        let h2 = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = c2.lock().unwrap();
                *num += 1;
            }
        });

        h1.join().unwrap();
        h2.join().unwrap();

        let result = *counter.lock().unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_channel_iterator_pattern() {
        let results = channel_iterator_pattern();
        assert_eq!(results.len(), 3);
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_mutex_and_channel() {
        let result = mutex_and_channel();
        // Vérifiez que la valeur a été modifiée
        assert!(result > 0);
    }

    #[test]
    fn test_channel_multiple_sends() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            for i in 1..=5 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        });

        let collected: Vec<i32> = rx.iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mutex_lock_and_modify() {
        let value = Arc::new(Mutex::new(10));
        let v_clone = Arc::clone(&value);

        let handle = thread::spawn(move || {
            let mut val = v_clone.lock().unwrap();
            *val += 5;
        });

        handle.join().unwrap();

        let result = *value.lock().unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_channel_try_recv_empty() {
        let (_tx, rx) = mpsc::channel::<i32>();
        let result = rx.try_recv();
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_try_recv_with_value() {
        let (tx, rx) = mpsc::channel();
        tx.send(42).unwrap();
        let result = rx.try_recv();
        assert_eq!(result.unwrap(), 42);
    }
}
