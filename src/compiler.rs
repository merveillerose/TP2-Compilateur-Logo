use crate::ast::{AST,Order};

#[derive(Debug)]
pub struct Logo{
    x: f32,
    y: f32,
    angle: f32,
    pen_down: bool,
    svg_content: String,
}


impl Logo{
    pub fn new() -> Self {
        Logo{
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            pen_down: true,  // stylo baissé par défaut
            svg_content: String::from(
                r#"<?xml version="1.0" encoding="utf-8"?>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="500" height="500">
            "#,
            ),
        }
    }

    pub fn compiler(&mut self, ast: &AST) -> String {
        match ast {
            AST::Program(commands) => {
                for cmd in commands {
                    self.compiler(cmd);
                }
            }
            AST::Command(parts) => {
                // On stocke temporairement la commande pour le nombre
                let mut current_order: Option<&Order> = None;

                for part in parts {
                    match part {
                        AST::Order(order) => current_order = Some(order),
                        AST::Number(n) => {
                            if let Some(order) = current_order {
                                match order {
                                    Order::Forward => {
                                        let rad = self.angle.to_radians();
                                        let new_x = self.x + *n as f32 * rad.cos();
                                        let new_y = self.y + *n as f32 * rad.sin();
                                        if self.pen_down {
                                            self.svg_content.push_str(&format!(
                                                r#"<path d="M {} {} L {} {}" stroke="black"/>"#,
                                                self.x, self.y, new_x, new_y
                                            ));
                                            self.svg_content.push('\n');
                                        }
                                        self.x = new_x;
                                        self.y = new_y;
                                    }
                                    Order::Backward => {
                                        let rad = self.angle.to_radians();
                                        let new_x = self.x - *n as f32 * rad.cos();
                                        let new_y = self.y - *n as f32 * rad.sin();
                                        if self.pen_down {
                                            self.svg_content.push_str(&format!(
                                                r#"<path d="M {} {} L {} {}" stroke="black"/>"#,
                                                self.x, self.y, new_x, new_y
                                            ));
                                            self.svg_content.push('\n');
                                        }
                                        self.x = new_x;
                                        self.y = new_y;
                                    }
                                    Order::Left => {
                                        self.angle -= *n as f32;
                                    }
                                    Order::Right => {
                                        self.angle += *n as f32;
                                    }
                                }
                            }
                        }
                        AST::None => {}
                        AST::Program(_) | AST::Command(_) => {
                            // récursion si jamais il y a des sous-commandes
                            self.compiler(part);
                        }
                    }
                }
            }
            AST::Order(_) | AST::Number(_) | AST::None => {}
        }

        // À la fin du programme, fermer le SVG
        if let AST::Program(_) = ast {
            self.svg_content.push_str("</svg>\n");
        }

        self.svg_content.clone()
    }
}