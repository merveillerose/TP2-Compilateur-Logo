mod lexer;

use crate::lexer::lexer_rules;


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
    println!("{:#?}", lexemes);

}