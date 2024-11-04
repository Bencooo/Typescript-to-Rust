// 2 - lexeur pour l'analyse lexicale

// Contient le lexeur, qui convertit le code source TypeScript 
// en une liste de Token. 
// Il gère le traitement des mots-clés, identifiants, 
// littéraux, symboles, et autres éléments syntaxiques. 
// C’est la première étape du pipeline de compilation.

// src/lexer.rs

use crate::token::Token;

pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = code.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            'c' if code[i..].starts_with("console") => {
                tokens.push(Token::Keyword("console".to_string()));
                i += "console".len();
            }
            'l' if code[i..].starts_with("log") => {
                tokens.push(Token::Identifier("log".to_string()));
                i += "log".len();
            }
            '"' => {
                let end_index = code[i + 1..].find('"').unwrap() + i + 1;
                let literal_value = &code[i + 1..end_index];
                tokens.push(Token::Literal(literal_value.to_string()));
                i = end_index + 1;
            }
            '(' | ')' | ';' => {
                tokens.push(Token::Symbol(chars[i]));
                i += 1;
            }
            _ => i += 1,
        }
    }

    tokens.push(Token::EOF); // Marque la fin de l'entrée
    tokens
}
