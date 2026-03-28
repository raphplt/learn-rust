// ============================================================
// Exercice 17 — Closures
// Niveau 6: Fn, FnMut, FnOnce, Capture de Variables
// ============================================================
//
// Les closures sont des fonctions anonymes qui peuvent capturer leur environnement.
// - Fn : capture par référence (&), peut être appelée plusieurs fois sans mutation
// - FnMut : capture par référence mutable (&mut), modifie son environnement
// - FnOnce : capture par valeur (move), ne peut être appelée qu'une fois
//
// 💡 Indices:
//   - `let f = |x| x * 2;` : closure simple
//   - `|x| { x * 2 }` : corps multi-ligne avec bloc
//   - `move |x| x + y` : capture y par valeur (move)
//   - `fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }`
//   - `fn counter<F: FnMut()>(mut f: F)` : FnMut doit être mut
//   - Les closures implémentent Fn/FnMut/FnOnce automatiquement selon leur capture
//
// Pour lancer : cargo test --test ex17_closures
// ============================================================

#[allow(dead_code)]

// ---- Fonctions à compléter ----

/// Écrivez une closure qui double son argument et passez-la à apply.
fn apply_double(x: i32) -> i32 {
    todo!("Créez une closure |x| x * 2 et appelez-la avec apply")
}

/// Fonction générique qui applique une closure à un nombre.
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

/// Écrivez une closure qui capture une variable externe (multiplier).
/// Retournez f(5) où f capture multiplier.
fn closure_with_capture(multiplier: i32) -> i32 {
    todo!("Créez une closure qui capture multiplier et retournez f(5)")
}

/// Écrivez une closure FnMut qui incrémente un compteur.
/// Appelez-la trois fois et retournez la valeur finale du compteur.
fn mutating_closure() -> i32 {
    todo!("Créez une closure mut qui incrémente un compteur, appelez-la 3 fois")
}

/// Écrivez une fonction qui accepte une FnMut et l'appelle trois fois
/// pour accumuler la somme de ses résultats.
fn call_three_times<F: FnMut() -> i32>(mut f: F) -> i32 {
    todo!("Appelez f() trois fois et retournez la somme")
}

/// Écrivez une closure qui prend la propriété (move) d'une String.
/// La closure doit retourner la longueur de la String capturée.
fn closure_with_move(s: String) -> Box<dyn Fn() -> usize> {
    todo!("Créez une closure move qui retourne la longueur de s")
}

/// Écrivez une fonction qui prend deux nombres et retourne une closure
/// qui additionne le premier nombre à son argument.
fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    todo!("Retournez une closure move qui additionne x à son argument")
}

/// Évaluez une liste de closures sur une valeur donnée.
/// Chaque closure accepte un i32 et retourne un i32.
fn evaluate_all(x: i32, closures: Vec<Box<dyn Fn(i32) -> i32>>) -> Vec<i32> {
    todo!("Appelez chaque closure avec x et collectez les résultats")
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_double() {
        assert_eq!(apply_double(5), 10);
        assert_eq!(apply_double(0), 0);
    }

    #[test]
    fn test_closure_with_capture() {
        assert_eq!(closure_with_capture(3), 15); // 3 * 5 = 15
        assert_eq!(closure_with_capture(2), 10); // 2 * 5 = 10
    }

    #[test]
    fn test_mutating_closure() {
        let result = mutating_closure();
        assert_eq!(result, 3); // Appelée 3 fois, donc compteur = 3
    }

    #[test]
    fn test_call_three_times() {
        let result = call_three_times(|| 10);
        assert_eq!(result, 30); // 10 + 10 + 10
    }

    #[test]
    fn test_call_three_times_incrementing() {
        let mut counter = 0;
        let f = || {
            counter += 1;
            counter
        };
        let result = call_three_times(f);
        assert_eq!(result, 6); // 1 + 2 + 3
    }

    #[test]
    fn test_closure_with_move() {
        let s = "hello".to_string();
        let len = s.len();
        let closure = closure_with_move(s);
        // La String a été movée dans la closure
        assert_eq!(closure(), len);
    }

    #[test]
    fn test_closure_with_move_returns_five() {
        let closure = closure_with_move("hello".to_string());
        assert_eq!(closure(), 5);
    }

    #[test]
    fn test_make_adder() {
        let add_5 = make_adder(5);
        assert_eq!(add_5(10), 15);
        assert_eq!(add_5(0), 5);
    }

    #[test]
    fn test_make_adder_negative() {
        let add_neg3 = make_adder(-3);
        assert_eq!(add_neg3(10), 7);
    }

    #[test]
    fn test_evaluate_all() {
        let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
            Box::new(|x| x * 2),
            Box::new(|x| x + 5),
            Box::new(|x| x * x),
        ];
        let results = evaluate_all(3, closures);
        assert_eq!(results, vec![6, 8, 9]); // 3*2=6, 3+5=8, 3^2=9
    }

    #[test]
    fn test_evaluate_all_empty() {
        let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![];
        let results = evaluate_all(3, closures);
        assert!(results.is_empty());
    }

    #[test]
    fn test_apply_with_custom_closure() {
        let f = |x: i32| x * x + 1;
        assert_eq!(apply(f, 4), 17); // 4^2 + 1 = 17
    }
}
