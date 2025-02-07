use crate::ast::TypeScriptNode;
use crate::token::Token;
use crate::utils::ValueType;

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

            // Si aucun cas ne correspond, on passe au token suivant
            _ => {
                i += 1;
                continue;
            }

            Token::Keyword(kw) if kw == "for" => {
                println!("✅ DEBUG: Parser - Détection du `for` à l'index {}", i);
                i += 1;
            
                if let Token::Symbol('(') = tokens[i] {
                    println!("✅ DEBUG: Parser - Détection de `(` à l'index {}", i);
                    i += 1;
            
                    // ✅ Capture l'initialisation (ex: `let i = 0;`)
                    let end_index = tokens[i..].iter().position(|t| *t == Token::Symbol(';')).unwrap_or(tokens.len());

                    let initialization = if let Some(init) = parse(tokens[i..i + end_index].to_vec()).first().cloned() {
                        println!("✅ DEBUG: Parser - Initialisation détectée `{:?}`", init);
                        i += end_index;
                        Some(Box::new(init))
                    } else {
                        None
                    };


            
                    if let Token::Symbol(';') = tokens[i] {
                        i += 1;
                    }
            
                    // ✅ Capture la condition (ex: `i < 5`)
                    let mut condition = "true".to_string();
                    if let Some(Token::Identifier(var)) = tokens.get(i) {
                        if let Some(Token::Operator(op)) = tokens.get(i + 1) {
                            if let Some(Token::Number(value)) = tokens.get(i + 2) {
                                condition = format!("{} {} {}", var, op, value);
                                println!("✅ DEBUG: Parser - Condition détectée `{}`", condition);
                                i += 3; // On avance après la condition
                            }
                        }
                    }
                
            
                    if let Token::Symbol(';') = tokens[i] {
                        i += 1;
                    }
            
                    // ✅ Capture l'incrémentation (ex: `i++`)
                    let increment = if let Some(Token::Identifier(var)) = tokens.get(i) {
                        if let Some(Token::Operator(op)) = tokens.get(i + 1) {
                            if op == "++" {
                                println!("✅ DEBUG: Parser - Incrémentation détectée `{}`", var);
                                i += 2;
                                Some(Box::new(TypeScriptNode::VariableDeclaration {
                                    name: var.clone(),
                                    value: format!("{} + 1", var),
                                }))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
            
                    if let Token::Symbol(')') = tokens[i] {
                        i += 1;
                    }
            
                    if let Token::Symbol('{') = tokens[i] {
                        println!("✅ DEBUG: Parser - Détection d'**ACCOLADE OUVRANTE** à l'index {}", i);
                        i += 1;
                        let mut body = Vec::new();
                    
                        // Trouver l’index de `}`
                        let end_index = tokens[i..]
                            .iter()
                            .position(|t| *t == Token::Symbol('}'))
                            .unwrap_or(tokens.len());
                    
                        if end_index == tokens.len() {
                            println!("⚠️ WARNING: `for` sans **ACCOLADE FERMANTE** !");
                        }
                    
                        // Récupérer tous les tokens entre `{` et `}`
                        let sub_tokens = tokens[i..i + end_index].to_vec();
                        println!(
                            "✅ DEBUG: Extraction du corps du `for` entre **ACCOLADES** → {:?}",
                            sub_tokens
                        );
                    
                        let parsed_nodes = parse(sub_tokens);
                        println!(
                            "✅ DEBUG: Résultat `parse()` du `for` → {:?}",
                            parsed_nodes
                        );
                    
                        body.extend(parsed_nodes);
                        i += end_index; // ✅ Avancer après le `}`
                    
                        if i < tokens.len() && tokens[i] == Token::Symbol('}') {
                            println!("✅ DEBUG: Détection d'**ACCOLADE FERMANTE** à l'index {}", i);
                            i += 1;
                        }
                    
                        println!(
                            "✅ DEBUG: Parser - Ajout `ForLoop` avec init `{:?}`, condition `{}`, incr `{:?}` et body `{:?}`",
                            initialization, condition, increment, body
                        );
                    
                        nodes.push(TypeScriptNode::ForLoop {
                            initialization,
                            condition,
                            increment,
                            body,
                        });
                    }
                    
                }
            }
            
            
            _ => i += 1, // ✅ Continue à parcourir les tokens
        }
    }

    nodes
}
