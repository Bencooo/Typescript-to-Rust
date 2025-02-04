// 6 - Générateur de code rust

// Le générateur de code Rust, qui prend l'AST Rust et produit 
// du code Rust lisible. Il assemble les fonctions, variables, 
// et autres structures en chaînes de caractères 
// correspondant à la syntaxe Rust.

// src/generator.rs

use crate::transformer::RustNode;

pub fn generate_code(nodes: Vec<RustNode>) -> String {
    nodes
        .into_iter()
        .map(|node| match node {
            RustNode::Println(text) => text,

            RustNode::VariableDeclaration { name, value } => {
                if value == "true" || value == "false" {
                    format!("let {} = {};", name, value) // ✅ Si c'est un booléen, on le garde sans guillemets
                } else if value.parse::<f64>().is_ok() {
                    format!("let {} = {};", name, value) // ✅ Si c'est un nombre, on garde tel quel
                } else {
                    format!("let {} = \"{}\".to_string();", name, value) // ✅ Si c'est une chaîne, on met `.to_string()`
                }
            }

            RustNode::IfStatement { condition, body } => {
                let body_code: String = body
                    .into_iter()
                    .map(|stmt| generate_code(vec![stmt])) // ✅ Générer chaque ligne du `if`
                    .collect::<Vec<String>>()
                    .join("\n    ");
                format!("if {} {{\n    {}\n}}", condition, body_code)
            }
        })
        .collect::<Vec<String>>()
        .join("\n") // ✅ Ajouter un saut de ligne entre chaque instruction
}

