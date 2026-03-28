fn main() {
    // Début du SVG
    let mut svg_content = String::new();
    svg_content.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
    svg_content.push_str("\n");
    svg_content.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="300" height="200">"#);
    svg_content.push_str("\n");

    // Ajouter des chemins pour un carré
    let paths = vec![
        (100, 100, 200, 100),
        (200, 100, 200, 200),
        (200, 200, 100, 200),
        (100, 200, 100, 100),
    ];

    for (x1, y1, x2, y2) in paths {
        svg_content.push_str(&format!(
            r#"  <path d="M {} {} L {} {}" stroke="red"/>"#,
            x1, y1, x2, y2
        ));
        svg_content.push_str("\n");
    }

    // Fin du SVG
    svg_content.push_str("</svg>\n");

    // Écrire dans un fichier
    std::fs::write("square.svg", svg_content).expect("Impossible d'écrire le fichier SVG");

    println!("Fichier square.svg généré !");
}
