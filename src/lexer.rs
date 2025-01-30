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
            // ✅ Détection de `log`
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
            // ✅ Détection des identifiants (variables, fonctions)
            // ✅ Détection des identifiants (variables, fonctions)
            _ if chars[i].is_alphabetic() => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let mut ident = code[start..i].to_string();

                // ✅ Forcer la suppression des guillemets s'ils sont présents
                if ident.starts_with('"') && ident.ends_with('"') {
                    ident = ident[1..ident.len() - 1].to_string();
                }

                //println!("✅ DEBUG: Lexer - Ajout Identifier `{}`", ident); // Debug clair
                println!("✅ DEBUG: Lexer - Ajout Identifier {:?}", ident);
                tokens.push(Token::Identifier(ident));
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
                if let Some(end_offset) = code[start..].find('"') {
                    let end_index = start + end_offset;
                    let literal_value = &code[start..end_index];

                    println!("✅ DEBUG: Lexer Détection de chaîne → {}", literal_value); // 🛠️ Debug
                    tokens.push(Token::Literal(literal_value.to_string())); // ✅ Ajoute comme Literal

                    i = end_index + 1;
                } else {
                    println!("❌ ERREUR: Chaîne non fermée !");
                }
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
    println!("✅ DEBUG: Tokens générés : {:?}", tokens);

    tokens.push(Token::EOF); // Marque la fin du fichier
    tokens
}
