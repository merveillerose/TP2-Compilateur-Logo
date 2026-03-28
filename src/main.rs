mod lexer;
mod parser;
mod ast;

use crate::lexer::lexer_rules;
use crate::parser::grammar;
use crate::ast::eval;


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

    let input = "forward 100";
    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
    //println!("{:#?}", lexemes);

    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    //println!("{:#?}", parse_trees);
    //println!("{:?}", parse_trees.as_abstract_syntax_tree());
    
    let ast = parse_trees.as_abstract_syntax_tree();
    eval(&ast)

}