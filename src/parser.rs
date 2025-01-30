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
                        println!("✅ DEBUG: Détection de `log` après `console` à l'index {}", i);
                        i += 1;
            
                        if let Token::Symbol('(') = tokens[i] {
                            println!("✅ DEBUG: Détection de `(` après `console.log` à l'index {}", i);
                            i += 1;
            
                            // ✅ Capture `console.log("texte")`
                            if let Token::Literal(text) = &tokens[i] {
                                println!("✅ DEBUG: Détection de CHAÎNE `{:?}` à l'index {}", text, i);
                                i += 1;
                                if let Token::Symbol(')') = tokens[i] { i += 1; }
                                if let Token::Symbol(';') = tokens[i] { i += 1; }
                                
                                println!("✅ DEBUG: Ajout ConsoleLog CHAÎNE {:?}", text);
                                nodes.push(TypeScriptNode::ConsoleLog(format!("\"{}\"", text))); // ✅ Stocke comme chaîne avec guillemets
                            }
                            // ✅ Capture `console.log(variable)`
                            else if let Token::Identifier(var_name) = &tokens[i] {
                                println!("✅ DEBUG: Détection de VARIABLE `{}` à l'index {}", var_name, i);
                                i += 1;
                                if let Token::Symbol(')') = tokens[i] { i += 1; }
                                if let Token::Symbol(';') = tokens[i] { i += 1; }
            
                                println!("✅ DEBUG: Ajout ConsoleLog VARIABLE {:?}", var_name);
                                nodes.push(TypeScriptNode::ConsoleLog(var_name.clone())); // ✅ Stocke comme variable
                            }
                        }
                    }
                }
            }
            

            // ✅ Détection des déclarations de variables (`const` et `let`)
            Token::Keyword(kw) if kw == "const" || kw == "let" => {
                i += 1;
                if let Token::Identifier(var_name) = &tokens[i] {
                    i += 1;
                    if let Token::Assign = tokens[i] {
                        i += 1;
                        if let Token::Number(value) = &tokens[i] {
                            nodes.push(TypeScriptNode::VariableDeclaration {
                                name: var_name.clone(),
                                value: value.to_string(),
                            });
                            i += 1;
                        } else if let Token::Literal(value) = &tokens[i] {
                            nodes.push(TypeScriptNode::VariableDeclaration {
                                name: var_name.clone(),
                                value: value.clone(),
                            });
                            i += 1;
                        }
                    }
                }
            }

            // ✅ Détection des structures conditionnelles `if`
            Token::Keyword(kw) if kw == "if" => {
                i += 1;
                if let Token::Symbol('(') = tokens[i] {
                    i += 1;
                    if let Token::Identifier(var) = &tokens[i] {
                        i += 1;
                        if let Token::Symbol('>') = tokens[i] {
                            i += 1;
                            if let Token::Number(value) = &tokens[i] {
                                i += 1;
                                if let Token::Symbol(')') = tokens[i] {
                                    i += 1;
                                    if let Token::Symbol('{') = tokens[i] {
                                        i += 1;
                                        let mut body = Vec::new();

                                        // ✅ Correction de la gestion des accolades `{}` dans `if`
                                        while i < tokens.len() && tokens[i] != Token::Symbol('}') {
                                            if let Some(node) =
                                                parse(vec![tokens[i].clone()]).first().cloned()
                                            {
                                                body.push(node);
                                            }
                                            i += 1; // ✅ Avance correctement
                                        }

                                        if i < tokens.len() && tokens[i] == Token::Symbol('}') {
                                            i += 1; // ✅ Passe le `}`
                                        }

                                        println!(
                                            "✅ DEBUG: Ajout IfStatement avec condition `{}` et body `{:?}`",
                                            format!("{} > {}", var, value),
                                            body
                                        );

                                        nodes.push(TypeScriptNode::IfStatement {
                                            condition: format!("{} > {}", var, value),
                                            body,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => i += 1, // ✅ Continue à parcourir les tokens
        }
    }

    println!("✅ DEBUG: AST généré : {:?}", nodes); // ✅ Debug pour voir l'AST généré
    nodes
}
