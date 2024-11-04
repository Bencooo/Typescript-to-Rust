// 6 - Générateur de code rust

// Le générateur de code Rust, qui prend l'AST Rust et produit 
// du code Rust lisible. Il assemble les fonctions, variables, 
// et autres structures en chaînes de caractères 
// correspondant à la syntaxe Rust.

// src/generator.rs

use crate::transformer::RustNode;

pub fn generate_code(node: RustNode) -> String {
    match node {
        RustNode::Println(text) => format!("println!(\"{}\");", text),
    }
}
