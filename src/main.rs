mod lexer;
mod parser;
mod ast;
mod compiler;

use crate::lexer::lexer_rules;
use crate::parser::grammar;
//use crate::ast::eval;
use crate::compiler::Logo;


fn main() {
    // programme qui respecte la grammaire pour construire un carré
    let _logo_program = "
        forward 100
        right 90
        forward 100
        right 90
        forward 100
        right 90
        forward 100
        right 90
    ";

    

    let _input = "
    penup
    forward 50
    pendown
    repeat 4 [
        forward 100
        right 90
    ]
    left 45
    backward 50
    repeat 2 [
        penup
        forward 30
        pendown
        left 90
        forward 20
    ]
";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &_input).unwrap();
    //println!("{:#?}", lexemes);

    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    //println!("{:#?}", parse_trees);
    //println!("{:?}", parse_trees.as_abstract_syntax_tree());
    
    let ast = parse_trees.as_abstract_syntax_tree();
    //eval(&ast);

    // ========= SVG =============
    let mut logo = Logo::new();
    logo.compiler(&ast); // compiler remplit le svg_content
    let svg = logo.finish(); // récupère le SVG complet
    std::fs::write("carre.svg", svg).expect("Impossible d'écrire le fichier SVG");
    println!("SVG généré dans carre.svg !");
}