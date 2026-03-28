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

    let _input = "forward 100";
    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &_logo_program).unwrap();
    //println!("{:#?}", lexemes);

    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    //println!("{:#?}", parse_trees);
    //println!("{:?}", parse_trees.as_abstract_syntax_tree());
    
    let ast = parse_trees.as_abstract_syntax_tree();
    //eval(&ast);

    // ========= SVG =============
    let mut logo = Logo::new();
    let svg = logo.compiler(&ast);

    std::fs::write("carre.svg", svg).expect("Impossible d'écrire le fichier SVG");
    
     println!("Fichier carre.svg généré !");
}