// 5 - transformation de l'AST Typescript en AST rust

// Le transformateur convertit l’AST TypeScript en un AST Rust. 
// Cette étape inclut la conversion des types 
// (par exemple, string en String) et la réorganisation des 
// structures pour qu’elles soient compatibles avec Rust.

// src/transformer.rs

use crate::ast::TypeScriptNode;

#[derive(Debug, Clone)]
pub enum RustNode {
    Println(String),  // ✅ println!("...");

    // ✅ Ajout du support pour les déclarations de variables en Rust
    VariableDeclaration { name: String, value: String },

    // ✅ Ajout du support pour les structures conditionnelles `if`
    IfStatement { condition: String, body: Vec<RustNode> },
}

pub fn transform(node: TypeScriptNode) -> RustNode {
    match node {
        TypeScriptNode::ConsoleLog(text) => RustNode::Println(text),

        // ✅ Transformation d'une variable TypeScript en variable Rust
        TypeScriptNode::VariableDeclaration { name, value } => {
            RustNode::VariableDeclaration { name, value }
        }

        // ✅ Transformation d'un `if` TypeScript en `if` Rust
        TypeScriptNode::IfStatement { condition, body } => {
            let transformed_body: Vec<RustNode> = body.into_iter().map(transform).collect();
            RustNode::IfStatement {
                condition,
                body: transformed_body,
            }
        }
    }
}

