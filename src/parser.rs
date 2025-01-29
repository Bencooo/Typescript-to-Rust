// 4 - analyseur syntaxique pour le code typescript

// Contient le parser, qui analyse la séquence de tokens fournie 
// par le lexeur et génère un AST TypeScript. 
// Il construit la structure syntaxique du programme, 
// en convertissant les tokens en une hiérarchie représentant 
// les déclarations de fonctions, les expressions, etc.

// src/parser.rs

use crate::token::Token;
use crate::ast::TypeScriptNode;

pub fn parse(tokens: Vec<Token>) -> TypeScriptNode {
    let mut i = 0;

    // Vérifier si le premier token est un mot-clé
    match &tokens[i] {
        Token::Keyword(kw) if kw == "console" => {
            i += 1;
            if let Token::Identifier(id) = &tokens[i] {
                if id == "log" {
                    i += 1;
                    if let Token::Symbol('(') = tokens[i] {
                        i += 1;
                        if let Token::Literal(text) = &tokens[i] {
                            return TypeScriptNode::ConsoleLog(text.clone());
                        }
                    }
                }
            }
        }
        
        Token::Keyword(kw) if kw == "const" || kw == "let" => {
            i += 1;
            if let Token::Identifier(var_name) = &tokens[i] {
                i += 1;
                if let Token::Assign = tokens[i] {
                    i += 1;
                    if let Token::Number(value) = &tokens[i] {
                        return TypeScriptNode::VariableDeclaration {
                            name: var_name.clone(),
                            value: value.to_string(),
                        };
                    } else if let Token::Literal(value) = &tokens[i] {
                        return TypeScriptNode::VariableDeclaration {
                            name: var_name.clone(),
                            value: value.to_string(),
                        };
                    }
                }
            }
        }

        Token::Keyword(kw) if kw == "if" => {
            i += 1;
            if let Token::Symbol('(') = tokens[i] {
                i += 1;
                if let Token::Identifier(var) = &tokens[i] {
                    i += 1;
                    if let Token::Symbol('>') = tokens[i] {
                        i += 1;
                        if let Token::Number(value) = &tokens[i] {
                            return TypeScriptNode::IfStatement {
                                condition: format!("{} > {}", var, value),
                                body: vec![], // On gérera le corps plus tard
                            };
                        }
                    }
                }
            }
        }

        _ => panic!("Unexpected token sequence"),
    }

    panic!("Unexpected token sequence");
}
