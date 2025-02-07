use crate::ast::RustNode;
use crate::utils::{State, ValueType};

pub fn generate_code(nodes: Vec<RustNode>) -> String {
    let mut result = Vec::new(); // Stocke les lignes de code générées

    for node in nodes {
        let line = match node {

            RustNode::Keyword(kw) => format!("{}",kw),
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

            RustNode::ForLoop { initialization, condition, increment, body } => {
                let init_code = initialization
                    .map(|init| generate_code(vec![*init])) // ✅ Générer l'initialisation (ex: `let i = 0;`)
                    .unwrap_or_default();
            
                let incr_code = increment
                    .map(|incr| generate_code(vec![*incr])) // ✅ Générer l'incrémentation (ex: `i += 1;`)
                    .unwrap_or_default();
            
                let body_code: String = body
                    .into_iter()
                    .map(|stmt| generate_code(vec![stmt])) // ✅ Générer chaque ligne du `for`
                    .collect::<Vec<String>>()
                    .join("\n    ");
            
                format!(
                    "{}\nwhile {} {{\n    {}\n    {}\n}}",
                    init_code, condition, body_code, incr_code
                )
            }

            RustNode::WhileLoop {
                initialization,
                condition,
                increment,
                body,
            } => {
                let init_code = initialization
                    .map(|init| generate_code(vec![*init]))
                    .unwrap_or("".to_string());

                let body_code = body
                    .into_iter()
                    .map(|stmt| generate_code(vec![stmt]))
                    .collect::<Vec<String>>()
                    .join("\n    ");

                let incr_code = increment
                    .map(|incr| generate_code(vec![*incr]))
                    .unwrap_or("".to_string());

                format!(
                    "{}\nwhile {} {{\n    {}\n    {}\n}}",
                    init_code, condition, body_code, incr_code
                )
            }

            RustNode::Expression(expr) => expr,
            

            // Gérer les symboles
            RustNode::Symbol(symbol) => symbol.to_string(),

            RustNode::Echap(symbol) => format!("{}\n", symbol),

            RustNode::Operator(op) => format!("{}",op),

        };

        result.push(line);
    }

    // Joindre toutes les lignes générées en une seule chaîne
    result.join("")
}
