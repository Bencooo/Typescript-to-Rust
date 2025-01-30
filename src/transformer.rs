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
        // ✅ Gestion correcte de `console.log(...)`
        TypeScriptNode::ConsoleLog(text) => {
            println!("✅ DEBUG: Transformer - Début conversion ConsoleLog({:?})", text);

            if text.starts_with('"') && text.ends_with('"') {
                // ✅ C'est un `Literal`, on enlève les guillemets
                let cleaned_text = text.trim_matches('"').to_string();
                println!("✅ DEBUG: ConsoleLog CHAÎNE → Avant: {:?}, Après: {:?}", text, cleaned_text);
                RustNode::Println(format!("println!(\"{}\");", cleaned_text))
            } else {
                // ✅ C'est une `Variable`, on utilise `{}` dans println!
                println!("✅ DEBUG: ConsoleLog VARIABLE → {:?}", text);
                RustNode::Println(format!("println!(\"{{}}\", {});", text))
            }
        }

        // ✅ Transformation d'une variable TypeScript en variable Rust
        TypeScriptNode::VariableDeclaration { name, value } => {
            println!("✅ DEBUG: Transformer - Déclaration variable : {} = {}", name, value);
            
            // ✅ Ajoute `.to_string()` si c'est une chaîne
            let rust_value = if value.starts_with('"') && value.ends_with('"') {
                format!("{}.to_string()", value)
            } else {
                value
            };

            RustNode::VariableDeclaration { name, value: rust_value }
        }

        // ✅ Transformation d'un `if` TypeScript en `if` Rust
        TypeScriptNode::IfStatement { condition, body } => {
            println!("✅ DEBUG: Transformer - If Statement avec condition `{}`", condition);
            let transformed_body: Vec<RustNode> = body.into_iter().map(transform).collect();
            RustNode::IfStatement {
                condition,
                body: transformed_body,
            }
        }
    }
}





