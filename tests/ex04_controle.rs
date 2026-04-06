// ============================================================
// Exercice 04 — Contrôle de Flux
// Niveau 1: Concepts Fondamentaux
// ============================================================
//
// Rust offre if/else pour les conditions, match pour le pattern matching,
// et plusieurs types de boucles: loop (infinie), while, for (avec ranges).
// Les ranges comme 1..5 vont de 1 à 4 (exclusif à droite).
// Les expressions if/match peuvent retourner des valeurs.
//
// 💡 Indices:
//   - `if x > 5 { ... } else { ... }` pour les conditions
//   - `match value { 1 => "un", 2 => "deux", _ => "autre" }` pour le matching
//   - `for i in 1..5 { ... }` itère sur le range 1..5
//   - `while condition { ... }` boucle tant que la condition est vraie
//
// Pour lancer : cargo test --test ex04_controle
// ============================================================

#[allow(dead_code)]
// ---- Fonctions à compléter ----

/// Retournez "grand" si n >= 100, "moyen" si n >= 50, sinon "petit".
fn categorize(n: i32) -> &'static str {
    if n >= 100 {
        return "grand";
    }
    if n >= 50 {
        return "moyen";
    } else {
        "petit"
    }
}

/// Utilisez match pour retourner le nom du mois (1 -> "Janvier", etc.).
/// Cas par défaut: "Invalide".
fn month_name(month: i32) -> &'static str {
    match month {
        1 => "Janvier",
        2 => "Février",
        3 => "Mars",
        4 => "Avril",
        5 => "Mai",
        6 => "Juin",
        7 => "Juillet",
        8 => "Aout",
        9 => "Septembre",
        10 => "Octobre",
        11 => "Novembre",
        12 => "Décembre",
        _ => "Invalide",
    }
}

/// Bouclez de 1 à 10 (inclus) et retournez la somme.
fn sum_to_10() -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < 11 {
        sum = sum + i;
        i += 1
    }
    sum
}

/// Retournez le nombre de nombres pairs dans le range 1 à 20.
fn count_even_to_20() -> i32 {
    let mut even = 0;
    for n in 1..21 {
        if n % 2 == 0 {
            even = even + 1
        }
    }
    even
}

/// Créez une boucle infinie qui s'arrête quand une variable >= 100.
/// Retournez le nombre d'itérations.
fn loop_until_100() -> i32 {
    let mut variable = 0;
    let mut count = 0;
    while variable < 100 {
        variable += 1;
        count += 1
    }
    count
}

// ---- Tests (ne pas modifier) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(150), "grand");
        assert_eq!(categorize(75), "moyen");
        assert_eq!(categorize(30), "petit");
    }

    #[test]
    fn test_month_name() {
        assert_eq!(month_name(1), "Janvier");
        assert_eq!(month_name(12), "Décembre");
        assert_eq!(month_name(13), "Invalide");
    }

    #[test]
    fn test_sum_to_10() {
        assert_eq!(sum_to_10(), 55); // 1+2+...+10
    }

    #[test]
    fn test_count_even_to_20() {
        assert_eq!(count_even_to_20(), 10);
    }

    #[test]
    fn test_loop_until_100() {
        assert_eq!(loop_until_100(), 100);
    }
}
