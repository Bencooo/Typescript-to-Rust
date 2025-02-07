
use crate::ast::TypeScriptNode;
use crate::ast::RustNode;
use crate::token::Token;

pub fn transform(node: TypeScriptNode) -> RustNode {
    match node {
        TypeScriptNode::Keyword(kw) => {
            RustNode::Keyword(kw)
        }
        // ✅ Gestion correcte de `console.log(...)`
        TypeScriptNode::ConsoleLog(text) => {
            println!("✅ DEBUG: Transformer - Début conversion ConsoleLog({:?})", text);

            if text.starts_with('"') && text.ends_with('"') {
                // ✅ C'est un `Literal`, on enlève les guillemets
                let cleaned_text = text.trim_matches('"').to_string();
                println!("✅ DEBUG: ConsoleLog CHAÎNE → Avant: {:?}, Après: {:?}", text, cleaned_text);
                RustNode::Println(format!("println!({})", text))
            } else {
                // ✅ C'est une `Variable`, on crée un Token pour la variable
                println!("✅ DEBUG: ConsoleLog VARIABLE → {:?}", text);
                RustNode::Println(format!("println!(\"{{}}\", {})", text))
            }
        }

        // ✅ Transformation d'une variable TypeScript en variable Rust
        TypeScriptNode::VariableDeclaration { name, value, state } => {
            RustNode::VariableDeclaration { name, value,state  }
        }

        TypeScriptNode::VariableInitialization {name, typevar, state} => {
            RustNode::VariableInitialization {name, typevar, state}
        }

        TypeScriptNode::Symbol(symbol) => {
            RustNode::Symbol(symbol)
        }

        TypeScriptNode::Echap(symbol) => {
            RustNode::Echap(symbol)
        }

        TypeScriptNode::Operator(op) => {
            RustNode::Operator(op)
        }

        TypeScriptNode::Assign => {
            RustNode::Symbol('=')
        }


    }

}