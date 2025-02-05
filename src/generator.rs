use crate::ast::RustNode;
use crate::utils::{State, ValueType};

pub fn generate_code(nodes: Vec<RustNode>) -> String {
    let mut result = Vec::new(); // Stocke les lignes de code générées

    for node in nodes {
        let line = match node {
            // Gérer le nœud Println
            RustNode::Println(text) => text,

            // Gérer les déclarations de variables avec leur état
            RustNode::VariableDeclaration { name, value, state } => {
                match state {
                    State::Mutable => match value {
                        ValueType::String(s) => format!("let mut {} : String = {:?}", name, s),
                        ValueType::F64(f) => format!("let mut {} : f64 = {:?}", name, f),
                        ValueType::Bool(b) => format!("let mut {} : bool = {:?}", name, b),
                    },
                    State::Immutable => match value {
                        ValueType::String(s) => format!("let {} : String = {:?}", name, s),
                        ValueType::F64(f) => format!("let {} : f64 = {:?}", name, f),
                        ValueType::Bool(b) => format!("let {} : bool = {:?}", name, b),
                    },
                    _ => "".to_string(),
                }
            }

            // Gérer l'initialisation des variables avec un type générique
            RustNode::VariableInitialization { name, typevar, state } => {
                match state {
                    State::Mutable => format!("let mut {} : {}", name, typevar),
                    State::Immutable => format!("let {} : {}", name, typevar),
                    _ => "".to_string(),
                }
            }

            // Gérer les symboles
            RustNode::Symbol(symbol) => format!("{}", symbol),

            RustNode::Echap(symbol) => format!("{}\n", symbol),

        };

        result.push(line);
    }

    // Joindre toutes les lignes générées en une seule chaîne
    result.join("")
}
