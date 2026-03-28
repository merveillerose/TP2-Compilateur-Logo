#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Order(Order), 
    Number(i32),
    Repeat(i32, Box<AST>),  
    Block(Vec<AST>),        
    PenUp,
    PenDown,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Order{
    Forward,
    Backward,
    Left,
    Right,
}

#[allow(dead_code)]
pub fn eval(ast: &AST) -> () {
    match ast {
        AST::Program(commands) => {
            // On parcourt chaque commande du programme
            for cmd in commands{
                eval(cmd);
            }
            println!("Stop");  // fin du programme
        }
        AST::Command(parts) => {
            for part in parts {
                eval(part)
            }
        }
        AST::Order(order) => {
            match order {
                Order::Forward => print!("Avance de "),
                Order::Backward => print!("Recule de"),
                Order::Left => print!("Tourne à gauche de "),
                Order::Right => print!("Tourne à droite de "),
            }
        }
        AST::Number(n) => {
            println!("{} unités",n);
        }
        AST::Repeat(n, cmd) => {
            for _ in 0..*n {
                eval(cmd);
            }
        }
        AST::Block(commands) => {
            for cmd in commands {
                eval(cmd);
            }
        }
        AST::PenUp => println!("Stylo levé"),
        AST::PenDown => println!("Stylo baissé"),

        AST::None => {
        }
    }
}
