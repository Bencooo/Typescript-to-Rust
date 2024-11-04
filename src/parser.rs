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

    if let Token::Keyword(ref kw) = tokens[i] {
        if kw == "console" {
            i += 1;
            if let Token::Identifier(ref id) = tokens[i] {
                if id == "log" {
                    i += 1;
                    if let Token::Symbol('(') = tokens[i] {
                        i += 1;
                        if let Token::Literal(ref text) = tokens[i] {
                            return TypeScriptNode::ConsoleLog(text.clone());
                        }
                    }
                }
            }
        }
    }

    panic!("Unexpected token sequence");
}
