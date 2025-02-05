use crate::ast::RustNode;
use crate::utils::{State, ValueType};

pub fn generate_code(nodes: Vec<RustNode>) -> String {
    nodes
        .into_iter()
        .map(|node| match node {
            RustNode::Println(text) => text,  // Gérer le nœud Println
            RustNode::VariableDeclaration { name, value, state } => {
                let var = match state {
                    State::Mutable => {
                        match value {
                            ValueType::String(s) => format!("let mut {} : String = {:?};", name, s),
                            ValueType::F64(f) => format!("let mut {} : f64 = {:?};", name, f),
                            ValueType::Bool(b) => format!("let mut {} : bool = {:?};", name, b),
                        }
                    }
                    State::Immutable => {
                        match value {
                            ValueType::String(s) => format!("let {} : String = {:?};", name, s),
                            ValueType::F64(f) => format!("let {} : f64 = {:?};", name, f),
                            ValueType::Bool(b) => format!("let {} : bool = {:?};", name, b),
                        }
                    }
                    _ =>{"".to_string()}
                };
                var  // Retourne la déclaration de la variable
            }
        })
        .collect::<Vec<String>>()
        .join("\n") // Ajouter un saut de ligne entre chaque instruction
}
