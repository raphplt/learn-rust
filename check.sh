#!/bin/bash
# ============================================================
# Rust Exercices — Script de vérification
# Lance les exercices un par un et affiche ta progression
# ============================================================

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

EXERCISES=(
    "ex01_variables:Niveau 1:Variables & Mutabilité"
    "ex02_types:Niveau 1:Types Primitifs"
    "ex03_fonctions:Niveau 1:Fonctions"
    "ex04_controle:Niveau 1:Contrôle de Flux"
    "ex05_ownership:Niveau 2:Ownership"
    "ex06_references:Niveau 2:Références & Borrowing"
    "ex07_slices:Niveau 2:Slices"
    "ex08_structs:Niveau 3:Structs"
    "ex09_enums:Niveau 3:Enums & Pattern Matching"
    "ex10_option:Niveau 3:Option<T>"
    "ex11_traits:Niveau 4:Traits"
    "ex12_generics:Niveau 4:Génériques & Trait Bounds"
    "ex13_result:Niveau 5:Result<T, E>"
    "ex14_erreurs:Niveau 5:Gestion d'Erreurs"
    "ex15_vecteurs:Niveau 6:Vec & HashMap"
    "ex16_iterateurs:Niveau 6:Itérateurs"
    "ex17_closures:Niveau 6:Closures"
    "ex18_lifetimes:Niveau 7:Lifetimes"
    "ex19_box:Niveau 8:Box & Smart Pointers"
    "ex20_rc_arc:Niveau 8:Rc, Arc & RefCell"
    "ex21_threads:Niveau 9:Threads"
    "ex22_channels:Niveau 9:Channels & Mutex"
    "ex23_kvstore:Niveau 10:Mini Key-Value Store"
    "ex24_parser:Niveau 10:Parser de Commandes"
    "ex25_serialisation:Niveau 10:Sérialisation"
)

echo ""
echo -e "${CYAN}${BOLD}🦀 Rust par la Pratique — Vérification des exercices${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo ""

passed=0
failed=0
total=${#EXERCISES[@]}
current_level=""

for entry in "${EXERCISES[@]}"; do
    IFS=':' read -r name level title <<< "$entry"

    # Print level header if new level
    if [ "$level" != "$current_level" ]; then
        echo -e "\n${YELLOW}${BOLD}── $level ──${NC}"
        current_level="$level"
    fi

    # Run the test
    output=$(cargo test --test "$name" 2>&1)
    exit_code=$?

    if [ $exit_code -eq 0 ]; then
        echo -e "  ${GREEN}✅ $name${NC} — $title"
        ((passed++))
    else
        # Check if it's a todo!() (not yet started) or a real failure
        if echo "$output" | grep -q "not yet implemented"; then
            echo -e "  ${YELLOW}⏳ $name${NC} — $title ${YELLOW}(pas encore commencé)${NC}"
        else
            echo -e "  ${RED}❌ $name${NC} — $title ${RED}(erreurs dans ton code)${NC}"
        fi
        ((failed++))
    fi
done

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}Progression : ${GREEN}$passed${NC}/${BOLD}$total exercices réussis${NC}"

# Progress bar
bar_width=40
filled=$((passed * bar_width / total))
empty=$((bar_width - filled))
bar="${GREEN}"
for ((i=0; i<filled; i++)); do bar+="█"; done
bar+="${RED}"
for ((i=0; i<empty; i++)); do bar+="░"; done
bar+="${NC}"
echo -e "  [$bar]"

if [ $passed -eq $total ]; then
    echo ""
    echo -e "${GREEN}${BOLD}🎉 Bravo Raphaël ! Tu as terminé tous les exercices !${NC}"
    echo -e "${GREEN}Tu maîtrises maintenant les fondamentaux de Rust. 🦀${NC}"
fi
echo ""
