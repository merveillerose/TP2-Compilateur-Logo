use crate::ast::{AST, Order};

/// Interpréteur simple qui parcourt l'AST et exécute les commandes dans l'ordre.
/// Il affiche chaque action sur la sortie standard et accumule les segments
/// dessinés afin de pouvoir générer une trace SVG après l'interprétation.
#[derive(Debug)]
pub struct Interpreter {
    x: f32,
    y: f32,
    angle: f32,
    pen_down: bool,
    segments: Vec<((f32, f32), (f32, f32))>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            angle: 0.0,     // orienté vers la droite au départ
            pen_down: true, // le stylo est baissé par défaut
            segments: Vec::new(),
        }
    }

    /// Exécute le nœud AST reçu ainsi que tous ses enfants.
    pub fn run(&mut self, ast: &AST) {
        match ast {
            AST::Program(commands) => {
                // Parcourt et exécute chaque commande du programme
                for cmd in commands {
                    self.run(cmd);
                }
            }
            AST::Command(parts) => {
                // Extrait l'ordre (direction) et la valeur (distance/angle)
                // puis applique le mouvement correspondant
                let mut current_order: Option<Order> = None;
                let mut value: Option<i32> = None;
                for part in parts {
                    match part {
                        AST::Order(o) => current_order = Some(o.clone()),
                        AST::Number(n) => value = Some(*n),
                        other => self.run(other),
                    }
                }
                if let (Some(order), Some(n)) = (current_order, value) {
                    self.apply(order, n);
                }
            }
            AST::Repeat(n, command) => {
                // Répète la commande enfant n fois
                for i in 0..*n {
                    println!("répétition {}/{}", i + 1, n);
                    self.run(command);
                }
            }
            AST::Block(commands) => {
                // Exécute séquentiellement toutes les commandes du bloc
                for cmd in commands {
                    self.run(cmd);
                }
            }
            AST::PenUp => {
                // Lève le stylo : les déplacements suivants ne laisseront pas de trace
                self.pen_down = false;
                println!("stylo levé");
            }
            AST::PenDown => {
                // Baisse le stylo : les déplacements suivants seront tracés
                self.pen_down = true;
                println!("stylo baissé");
            }
            // Ces variantes sont gérées dans leurs contextes parents ; rien à faire ici
            AST::Order(_) | AST::Number(_) | AST::None => {}
        }
    }

    /// Applique un ordre de déplacement ou de rotation à la tortue.
    fn apply(&mut self, order: Order, value: i32) {
        match order {
            Order::Forward  => self.move_turtle(value as f32),
            Order::Backward => self.move_turtle(-(value as f32)),
            Order::Left => {
                // Rotation vers la gauche (sens anti-horaire)
                self.angle -= value as f32;
                println!("gauche {} -> nouveau cap {:.1}°", value, self.angle);
            }
            Order::Right => {
                // Rotation vers la droite (sens horaire)
                self.angle += value as f32;
                println!("droite {} -> nouveau cap {:.1}°", value, self.angle);
            }
        }
    }

    /// Déplace la tortue d'une distance donnée dans la direction actuelle.
    /// Si le stylo est baissé, le segment parcouru est enregistré.
    fn move_turtle(&mut self, distance: f32) {
        let rad = self.angle.to_radians();
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y + distance * rad.sin();

        println!(
            "déplacement {:+} de ({:.1}, {:.1}) vers ({:.1}, {:.1}){}",
            distance,
            self.x,
            self.y,
            new_x,
            new_y,
            if self.pen_down { " [tracé]" } else { "" }
        );

        // Enregistre le segment uniquement si le stylo est en contact
        if self.pen_down {
            self.segments.push(((self.x, self.y), (new_x, new_y)));
        }

        self.x = new_x;
        self.y = new_y;
    }

    /// Construit une représentation SVG de tout ce qui a été dessiné pendant l'exécution.
    pub fn to_svg(&self) -> String {
        let mut svg = String::from(
            r#"<?xml version="1.0" encoding="utf-8"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="500" height="500">
"#,
        );

        // Génère un élément <path> pour chaque segment tracé
        for ((x1, y1), (x2, y2)) in &self.segments {
            svg.push_str(&format!(
                r#"<path d="M {} {} L {} {}" stroke="blue"/>"#,
                x1, y1, x2, y2
            ));
            svg.push('\n');
        }

        svg.push_str("</svg>\n");
        svg
    }
}