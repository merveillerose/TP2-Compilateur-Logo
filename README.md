# Compilateur et interpréteur Logo (Rust)

Mini chaîne d’outils Logo écrite en Rust pour le TP : lexeur + parseur + AST + compilateur SVG + interpréteur pas‑à‑pas.

## Organisation
- `src/lexer.rs` — règles Santiago pour transformer le texte Logo en lexèmes.
- `src/parser.rs` — grammaire BNF et construction de l’AST.
- `src/ast.rs` — définitions de l’AST et petite fonction `eval` de démonstration.
- `src/compiler.rs` — parcours l’AST pour générer des chemins SVG (compilateur).
- `src/interpreter.rs` — exécute l’AST pas à pas, affiche les actions et stocke les segments dessinés.
- `src/main.rs` — point d’entrée, lance l’interpréteur puis le compilateur sur un programme Logo d’exemple.
- `src/bin/svg_program.rs` — binaire de test qui dessine un carré statique en SVG.

## Prérequis
- Rust 1.75+ (édition 2024).
- Crates déjà listées dans `Cargo.toml` : `santiago`, `svg_fmt`.

## Lancer l’exemple
```bash
cargo run
```
Ce qui se passe :
1) Le programme Logo dans `src/main.rs` est lexé et parsé.
2) L’interpréteur affiche chaque action dans le terminal.
3) Un SVG de l’exécution est écrit dans `interpretation.svg`.
4) Un SVG issu de la compilation est écrit dans `carre.svg`.

Le programme par défaut trace un carré, quelques déplacements, puis deux petits traits perpendiculaires.

Pour tester un autre dessin, modifiez la chaîne `input` dans `src/main.rs` en respectant la grammaire étendue :
```
forward|backward <number>
left|right <angle>
penup | pendown
repeat <number> [ <program> ]
```

## Commandes utiles
- Vérifier la compilation : `cargo check`
- Démo du carré statique : `cargo run --bin svg_program`

## Notes d’implémentation
- Système de coordonnées : départ en (100, 100), cap 0 deg vers la droite, unités arbitraires.
- `compiler.rs` écrit directement des `<path>` SVG.  
  `interpreter.rs` produit les mêmes segments mais journalise chaque action (`pen up/down`, déplacement, rotation).
- Les deux SVG sont générés à la racine du projet pour ouverture rapide dans un navigateur.
