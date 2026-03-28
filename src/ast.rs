#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Order(Order), 
    Number(i32),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Order{
    Forward,
    Backward,
    Left,
    Right,
}

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
        AST::None => {
            // Règle vide, rien à afficher
        }
    }
}
