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
            // ✅ Détection de `console`
            'c' if code[i..].starts_with("console") => {
                tokens.push(Token::Keyword("console".to_string()));
                i += "console".len();
            }
            // ✅ Détection de `log` (doit venir après `console`)
            'l' if code[i..].starts_with("log") => {
                tokens.push(Token::Identifier("log".to_string()));
                i += "log".len();
            }

            // ✅ Détection des autres mots-clés (`const`, `let`, etc.)
            'c' if code[i..].starts_with("const") => {
                tokens.push(Token::Keyword("const".to_string()));
                i += "const".len();
            }
            'l' if code[i..].starts_with("let") => {
                tokens.push(Token::Keyword("let".to_string()));
                i += "let".len();
            }
            'f' if code[i..].starts_with("function") => {
                tokens.push(Token::Keyword("function".to_string()));
                i += "function".len();
            }
            'i' if code[i..].starts_with("if") => {
                tokens.push(Token::Keyword("if".to_string()));
                i += "if".len();
            }
            'e' if code[i..].starts_with("else") => {
                tokens.push(Token::Keyword("else".to_string()));
                i += "else".len();
            }
            'r' if code[i..].starts_with("return") => {
                tokens.push(Token::Keyword("return".to_string()));
                i += "return".len();
            }

            // ✅ Détection des identifiants (variables, fonctions)
            _ if chars[i].is_alphabetic() => {
                let start = i;
                while i < chars.len() && chars[i].is_alphanumeric() {
                    i += 1;
                }
                let ident = &code[start..i];
                tokens.push(Token::Identifier(ident.to_string()));
            }

            // ✅ Détection des nombres (42, 3.14)
            _ if chars[i].is_digit(10) => {
                let start = i;
                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                    i += 1;
                }
                let number = &code[start..i];
                tokens.push(Token::Number(number.parse::<f64>().unwrap()));
            }

            // ✅ Détection des chaînes de caractères `"Hello"`
            '"' => {
                let start = i + 1;
                let end_index = code[start..].find('"').unwrap() + start;
                let literal_value = &code[start..end_index];
                tokens.push(Token::Literal(literal_value.to_string()));
                i = end_index + 1;
            }

            // ✅ Détection de l'affectation `=`
            '=' => {
                tokens.push(Token::Assign);
                i += 1;
            }

            // ✅ Détection des symboles `(`, `)`, `{`, `}`, `;`
            '(' | ')' | '{' | '}' | ';' => {
                tokens.push(Token::Symbol(chars[i]));
                i += 1;
            }

            // ✅ Ignorer les espaces et caractères inconnus
            _ => i += 1,
        }
    }

    tokens.push(Token::EOF); // Marque la fin du fichier
    tokens
}

