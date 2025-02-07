use crate::ast::TypeScriptNode;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<TypeScriptNode> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {

            // Détection de `console.log(...)`
            Token::Keyword(kw) => {
                if kw == "console.log" {
                    println!("✅ DEBUG: Détection de console.log");
                    i += 1;

                    // Vérification de la parenthèse ouvrante
                    if let Token::Symbol('(') = &tokens[i] {
                        i += 1;

                        // Capture `console.log("texte")`
                        if let Token::Literal(text) = &tokens[i] {
                            println!(
                                "✅ DEBUG: Détection de CHAÎNE `{:?}` à l'index {}",
                                text, i
                            );
                            i += 1;

                            if let Token::Symbol(')') = &tokens[i] {
                                i += 1;
                            }

                            println!("✅ DEBUG: Ajout ConsoleLog CHAÎNE {:?}", text);
                            nodes.push(TypeScriptNode::ConsoleLog(format!("\"{}\"", text)));
                        }
                        // Capture `console.log(variable)`
                        else if let Token::Identifier(var_name) = &tokens[i] {
                            println!(
                                "✅ DEBUG: Détection de VARIABLE `{}` à l'index {}",
                                var_name, i
                            );
                            i += 1;

                            if let Token::Symbol(')') = &tokens[i] {
                                i += 1;
                            }

                            println!("✅ DEBUG: Ajout ConsoleLog VARIABLE {:?}", var_name);
                            nodes.push(TypeScriptNode::ConsoleLog(var_name.clone()));
                        }
                        
                        
                    }
                    
                }else{
                    println!("ajout de {}",kw);
                    nodes.push(TypeScriptNode::Keyword(kw.clone()));
                    i+=1;
                }
            }
            // Traitement des déclarations de variables
            Token::Variable{ name, value, state } => {
                nodes.push(TypeScriptNode::VariableDeclaration{name: name.clone(),value: value.clone(),state: state.clone()});
                i+=1;
            }

            Token::Initialize{name,typevar,state} => {
                nodes.push(TypeScriptNode::VariableInitialization{name: name.clone(),typevar: typevar.clone(),state : state.clone()});
                i+=1;
            }

            Token::Identifier(ident) => {
                nodes.push(TypeScriptNode::Keyword(ident.clone()));
                i+=1;
            }

            Token::Echap(symbol) => {
                nodes.push(TypeScriptNode::Echap(symbol.clone()));
                i+=1;

            }

            Token::Symbol(symbol) => {
                nodes.push(TypeScriptNode::Symbol(symbol.clone()));
                i+=1;
            }

            Token::Operator(op) => {
                nodes.push(TypeScriptNode::Operator(op.clone()));
                i+=1;
            }

            Token::Assign => {
                nodes.push(TypeScriptNode::Assign);
                i+=1;
            }

            Token::Number(n) => {
                nodes.push(TypeScriptNode::Number(n.clone()));
                i+=1;
            }
            
            _ => i += 1, // ✅ Continue à parcourir les tokens
        }
    }

    nodes
}
