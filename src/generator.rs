// 6 - Générateur de code rust

// Le générateur de code Rust, qui prend l'AST Rust et produit 
// du code Rust lisible. Il assemble les fonctions, variables, 
// et autres structures en chaînes de caractères 
// correspondant à la syntaxe Rust.

// src/generator.rs

use crate::transformer::RustNode;

pub fn generate_code(node: RustNode) -> String {
    match node {
        // ✅ Génération d'un println! en Rust
        RustNode::Println(text) => format!("println!(\"{}\");", text),

        // ✅ Génération d'une déclaration de variable Rust
        RustNode::VariableDeclaration { name, value } => {
            format!("let {} = {};", name, value)
        }

        // ✅ Génération d'une structure conditionnelle Rust
        RustNode::IfStatement { condition, body } => {
            let body_code: String = body.into_iter().map(generate_code).collect::<Vec<String>>().join("\n    ");
            format!("if {} {{\n    {}\n}}", condition, body_code)
        }
    }
}
