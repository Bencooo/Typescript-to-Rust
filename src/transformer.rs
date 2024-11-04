// 5 - transformation de l'AST Typescript en AST rust

// Le transformateur convertit l’AST TypeScript en un AST Rust. 
// Cette étape inclut la conversion des types 
// (par exemple, string en String) et la réorganisation des 
// structures pour qu’elles soient compatibles avec Rust.

// src/transformer.rs

use crate::ast::TypeScriptNode;

#[derive(Debug, Clone)]
pub enum RustNode {
    Println(String),  // Représente println!("...")
}

pub fn transform(node: TypeScriptNode) -> RustNode {
    match node {
        TypeScriptNode::ConsoleLog(text) => RustNode::Println(text),
    }
}
