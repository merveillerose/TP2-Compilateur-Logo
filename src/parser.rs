use santiago::grammar::Grammar;

use crate::ast::{AST,Order};

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => |_| AST::None;

        "command" => rules "order" "number" => AST::Command;

        "order" => lexemes "FORWARD" => |_| AST::Order(Order::Forward);
        "order" => lexemes "BACKWARD" => |_| AST::Order(Order::Backward);
        "order" => lexemes "LEFT" => |_| AST::Order(Order::Left);
        "order" => lexemes "RIGHT" => |_| AST::Order(Order::Right);

        "number" => lexemes "NUMBER" => |lex| AST::Number(lex[0].raw.parse().unwrap());

    )
    
}



// pub fn grammar() -> Grammar<()> {
//     santiago::grammar!(
//         "program" => rules "command" "program";
//         "program" => empty;

//         "command" => rules "order" "number" ;

//         "number" => lexemes "NUMBER";

//         "order" => lexemes "FORWARD";
//         "order" => lexemes "BACKWARD";
//         "order" => lexemes "LEFT";
//         "order" => lexemes "RIGHT";
        

//     )
// }