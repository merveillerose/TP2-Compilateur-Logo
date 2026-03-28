use santiago::grammar::Grammar;

use crate::ast::{AST,Order};

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => |_| AST::None;

        "command" => rules "order" "number" => AST::Command;
        "command" => rules "loop" => |v| v[0].clone();
        "command" => rules "block" => |v| v[0].clone();
        "command" => rules "state" => |v| v[0].clone();

        "loop" => rules "repeat" "number" "command" => |v| {
                 if let AST::Number(n) = &v[1] {
                    AST::Repeat(*n, Box::new(v[2].clone()))
                } else {
                    AST::None
                }
            };
 
        "block" => rules "lbracket" "program" "rbracket"
            => |v| {
                if let AST::Program(cmds) = &v[1] {
                    AST::Block(cmds.clone())
                } else {
                    AST::None
                }
            };
    
        "order" => lexemes "FORWARD" => |_| AST::Order(Order::Forward);
        "order" => lexemes "BACKWARD" => |_| AST::Order(Order::Backward);
        "order" => lexemes "LEFT" => |_| AST::Order(Order::Left);
        "order" => lexemes "RIGHT" => |_| AST::Order(Order::Right);

        "state" => lexemes "PENUP" => |_| AST::PenUp;
        "state" => lexemes "PENDOWN" => |_| AST::PenDown;

        "number" => lexemes "NUMBER" => |lex| AST::Number(lex[0].raw.parse().unwrap());
        "lbracket" => lexemes "LBRACKET" => |_| AST::None;
        "rbracket" => lexemes "RBRACKET" => |_| AST::None;
        "repeat" => lexemes "REPEAT" => |_| AST::None;

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