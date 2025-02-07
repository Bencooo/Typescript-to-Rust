// 4 - Analyseur syntaxique pour le code TypeScript
// Contient le parser, qui analyse la séquence de tokens fournie
// par le lexeur et génère un AST TypeScript.

// 4 - Analyseur syntaxique pour le code TypeScript
// Contient le parser, qui analyse la séquence de tokens fournie
// par le lexeur et génère un AST TypeScript.

use crate::ast::TypeScriptNode;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<TypeScriptNode> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            // ✅ Détection de `console.log(...)`
            Token::Keyword(kw) if kw == "console" => {
                println!("✅ DEBUG: Détection de `console` à l'index {}", i);
                i += 1;

                if let Token::Identifier(id) = &tokens[i] {
                    if id == "log" {
                        println!(
                            "✅ DEBUG: Détection de `log` après `console` à l'index {}",
                            i
                        );
                        i += 1;

                        if let Token::Symbol('(') = tokens[i] {
                            println!(
                                "✅ DEBUG: Détection de `(` après `console.log` à l'index {}",
                                i
                            );
                            i += 1;

                            // ✅ Capture `console.log("texte")`
                            if let Token::Literal(text) = &tokens[i] {
                                println!(
                                    "✅ DEBUG: Détection de CHAÎNE `{:?}` à l'index {}",
                                    text, i
                                );
                                i += 1;
                                if let Token::Symbol(')') = tokens[i] {
                                    i += 1;
                                }
                                if let Token::Symbol(';') = tokens[i] {
                                    i += 1;
                                }

                                println!("✅ DEBUG: Ajout ConsoleLog CHAÎNE {:?}", text);
                                nodes.push(TypeScriptNode::ConsoleLog(format!("\"{}\"", text)));
                            // ✅ Stocke comme chaîne avec guillemets
                            }
                            // ✅ Capture `console.log(variable)`
                            else if let Token::Identifier(var_name) = &tokens[i] {
                                println!(
                                    "✅ DEBUG: Détection de VARIABLE `{}` à l'index {}",
                                    var_name, i
                                );
                                i += 1;
                                if let Token::Symbol(')') = tokens[i] {
                                    i += 1;
                                }
                                if let Token::Symbol(';') = tokens[i] {
                                    i += 1;
                                }

                                println!("✅ DEBUG: Ajout ConsoleLog VARIABLE {:?}", var_name);
                                nodes.push(TypeScriptNode::ConsoleLog(var_name.clone()));
                                // ✅ Stocke comme variable
                            }
                        }
                    }
                }
            }

            // ✅ Détection des déclarations de variables (`const` et `let`)
            Token::Keyword(kw) if kw == "const" || kw == "let" => {
                println!("✅ DEBUG: Parser - Détection de déclaration `{}` à l'index {}", kw, i);
                i += 1;
            
                if let Token::Identifier(var_name) = &tokens[i] {
                    println!("✅ DEBUG: Nom de variable détecté `{}`", var_name);
                    i += 1;
            
                    if let Token::Operator(op) = &tokens[i] {
                        if op == "=" {  // ✅ Vérifie bien que c'est un `=`
                            i += 1;
                            
                            if let Token::Number(value) = &tokens[i] {
                                println!("✅ DEBUG: Valeur numérique détectée `{}`", value);
                                nodes.push(TypeScriptNode::VariableDeclaration {
                                    name: var_name.clone(),
                                    value: value.to_string(),
                                });
                                i += 1;
                            } else if let Token::Literal(value) = &tokens[i] {
                                println!("✅ DEBUG: Valeur littérale détectée `{}`", value);
                                nodes.push(TypeScriptNode::VariableDeclaration {
                                    name: var_name.clone(),
                                    value: value.clone(),
                                });
                                i += 1;
                            } else if let Token::Boolean(value) = &tokens[i] {  // ✅ Ajout des booléens
                                println!("✅ DEBUG: Valeur booléenne détectée `{}`", value);
                                nodes.push(TypeScriptNode::VariableDeclaration {
                                    name: var_name.clone(),
                                    value: value.to_string(),
                                });
                                i += 1;
                            }
                        }
                    }
                }
            }
            
            // ✅ Détection des structures conditionnelles `if`
            Token::Keyword(kw) if kw == "if" => {
                println!("✅ DEBUG: Parser - Détection de `if` à l'index {}", i);
                i += 1;
            
                if let Token::Symbol('(') = tokens[i] {
                    i += 1;
            
                    // ✅ Capture `x < 10`, `true`, `false`
                    let mut condition = String::new();
                    if let Token::Identifier(var) = &tokens[i] {
                        condition.push_str(var);
                        i += 1;
                    } else if let Token::Keyword(kw) = &tokens[i] {
                        if kw == "true" || kw == "false" {
                            condition.push_str(kw);
                            i += 1;
                        }
                    } else if let Token::Number(value) = &tokens[i] {
                        condition.push_str(&value.to_string());
                        i += 1;
                    } else {
                        println!("⚠️ WARNING: Condition `if` inattendue : {:?}", tokens[i]);
                        i += 1; // ✅ Évite la boucle infinie
                    }
            
                    if let Token::Operator(op) = &tokens[i] {  // ✅ Gère `<`, `>`, `<=`, `>=`, `==`, `!=`
                        let operator = op.clone();
                        condition.push_str(&format!(" {}", operator));
                        i += 1;
            
                        if let Token::Number(value) = &tokens[i] {
                            condition.push_str(&format!(" {}", value));
                            i += 1;
                        }
                    }
            
                    if let Token::Symbol(')') = tokens[i] {
                        i += 1;
                        
                        if let Token::Symbol('{') = tokens[i] {
                            i += 1;
                            let mut body = Vec::new();
            
                            while i < tokens.len() && tokens[i] != Token::Symbol('}') {
                                println!("✅ DEBUG: Parser - Analyse du bloc `if` → Token {:?}", tokens[i]);
            
                                let sub_tokens = tokens[i..].to_vec();
                                let parsed_nodes = parse(sub_tokens);
                                let parsed_len = parsed_nodes.len();
            
                                if !parsed_nodes.is_empty() {
                                    body.extend(parsed_nodes);
                                }
            
                                i += parsed_len;
                                if parsed_len == 0 {
                                    i += 1; // ✅ Sécurité pour éviter boucle infinie
                                }
                            }
            
                            if i < tokens.len() && tokens[i] == Token::Symbol('}') {
                                i += 1; // ✅ Passe `}`
                            }
            
                            println!(
                                "✅ DEBUG: Parser - Ajout IfStatement avec condition `{}` et body `{:?}`",
                                condition, body
                            );
            
                            nodes.push(TypeScriptNode::IfStatement {
                                condition,
                                body,
                            });
                        }
                    }
                }
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

    println!("✅ DEBUG: AST généré : {:?}", nodes); // ✅ Debug pour voir l'AST généré
    nodes
}
