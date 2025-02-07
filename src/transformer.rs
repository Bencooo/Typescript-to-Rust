
use crate::ast::RustNode;
use crate::ast::TypeScriptNode;

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

        // TypeScriptNode::ForLoop { initialization, condition, increment, body } => {
        //     println!(
        //         "✅ DEBUG: Transformer - Détection `for` avec init `{:?}`, condition `{}`, incr `{:?}` et body `{:?}`",
        //         initialization, condition, increment, body
        //     );
        
        //     let rust_initialization = initialization.map(|init| transform(*init));
        
        //     let rust_increment = increment.map(|incr| match *incr {
        //         TypeScriptNode::VariableDeclaration { name, value } => {
        //             // ✅ Correction : Transforme `i++` en `i += 1;`
        //             RustNode::Expression(format!("{} += 1;", name))
        //         }
        //         _ => transform(*incr),
        //     });
        
        //     let rust_body = body.into_iter().map(transform).collect::<Vec<RustNode>>();
        
        //     RustNode::WhileLoop {
        //         initialization: rust_initialization.map(Box::new),
        //         condition,
        //         increment: rust_increment.map(Box::new),
        //         body: rust_body,
        //     }
        // }
        

        // TypeScriptNode::WhileLoop { initialization, condition, increment, body } => {
        //     println!(
        //         "✅ DEBUG: Transformer - Détection `while` Rust avec init `{:?}`, condition `{}`, incr `{:?}` et body `{:?}`",
        //         initialization, condition, increment, body
        //     );
    
        //     let rust_initialization = initialization.map(|init| Box::new(transform(*init)));
        //     let rust_increment = increment.map(|incr| Box::new(transform(*incr)));
        //     let rust_body = body.into_iter().map(transform).collect::<Vec<RustNode>>();
    
        //     RustNode::WhileLoop {
        //         initialization: rust_initialization,
        //         condition,
        //         increment: rust_increment,
        //         body: rust_body,
        //     }
        // }
        
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
