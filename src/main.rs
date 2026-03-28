mod lexer;
mod parser;
mod ast;
mod compiler;
mod interpreter;

use crate::lexer::lexer_rules;
use crate::parser::grammar;
use crate::compiler::Logo as CompilerLogo;
use crate::interpreter::Interpreter;

fn main() {
    let input = "
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
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    let ast = parse_trees.as_abstract_syntax_tree();

    // ----- interpreter (partie 5)
    let mut interpreter = Interpreter::new();
    interpreter.run(&ast);
    let svg_interpretation = interpreter.to_svg();
    std::fs::write("interpretation.svg", svg_interpretation)
        .expect("Impossible d'ecrire le fichier SVG d'interpretation");
    println!("SVG interprete ecrit dans interpretation.svg");

    // ----- compilation vers SVG 
    let mut compiler_logo = CompilerLogo::new();
    compiler_logo.compiler(&ast);
    let svg = compiler_logo.finish();
    std::fs::write("carre.svg", svg).expect("Impossible d'ecrire le fichier SVG");
    //println!("SVG compile genere dans carre.svg");
}
