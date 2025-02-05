use crate::token::Token;
use crate::utils::State;
use crate::utils::ValueType;

pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    let mut state : State = State::NoneState;
    let mut name = "";

    while i < chars.len() {
        match chars[i] {
            'c' if code[i..].starts_with("console.log") => {
                tokens.push(Token::Keyword("console.log".to_string()));
                i += "console.log".len(); // Avancer l'indice après "console.log"
            }

            'c' if code[i..].starts_with("const") => {
                state = State::Immutable;
                i += "const".len();
            }

            'l' if code[i..].starts_with("let") => {
                state = State::Mutable;
                i += "let".len();
            }

            't' | 'f' => {
                if code[i..].starts_with("true") && state != State::NoneState && name != ""{
                    tokens.push(Token::Variable{name: name.to_string(),value:ValueType::Bool(true),state: state});
                    i += "true".len();
                } else if code[i..].starts_with("false") && state != State::NoneState && name != ""{
                    tokens.push(Token::Variable{name: name.to_string(),value:ValueType::Bool(false),state: state});
                    i += "false".len();
                }
                name = "";
                state = State::NoneState;
            }

            '(' | ')' | '{' | '}' | ';' => {
                tokens.push(Token::Symbol(chars[i]));
                i += 1;
            }

            '<' | '>' | '!' => {
                let mut op = chars[i].to_string();
                i += 1;

                if i < chars.len() && chars[i] == '=' {
                    op += "=";
                    i += 1;
                }

                println!("✅ DEBUG: Lexer - Détection de l'opérateur `{}`", op); // Debug
                tokens.push(Token::Operator(op));
            }

            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Operator("==".to_string()));
                    i += 2;  // Incrémenter de 2 pour passer le "=="
                } else {
                    tokens.push(Token::Assign);
                    i += 1;
                }
            }

            '"' => {
                let start = i + 1;
                if let Some(end_offset) = code[start..].find('"') {
                    let end_index = start + end_offset;
                    let literal_value = &code[start..end_index];

                    println!("✅ DEBUG: Lexer Détection de chaîne → {}", literal_value); // 🛠️ Debug
                    if name != "" && state != State::NoneState {
                        tokens.push(Token::Variable{name:name.to_string(),value:ValueType::String(literal_value.to_string()),state:state});
                    }else{
                        tokens.push(Token::Literal(literal_value.to_string()));
                    }

                    i = end_index + 1;
                } else {
                    name = "";
                    state = State::NoneState;
                    println!("❌ ERREUR: Chaîne non fermée !");
                    break; // Sortie en cas d'erreur de chaîne non fermée
                }
                name = "";
                state = State::NoneState;
            }
            
            // Détection des nombres (entiers et flottants)
            _ if chars[i].is_digit(10) => {
                let start = i;
                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                    i += 1;
                }
                let number = &code[start..i];
                if let Ok(n) = number.parse::<f64>() {
                    tokens.push(Token::Variable{name:name.to_string(),value:ValueType::F64(n),state:state});
                } else {
                    println!("❌ ERREUR: Nombre mal formé {}", number);
                    break;
                }
                name = "";
                state = State::NoneState;
            }

            // Détection des identifiants (variables, fonctions)
            _ if chars[i].is_alphabetic() => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident = &code[start..i];
                println!("✅ DEBUG: Lexer - Ajout Identifier {:?}", ident);
                if state != State::NoneState {
                    name = ident;
                }else{
                    tokens.push(Token::Identifier(ident.to_string()));
                }
                
            }

            // Autres caractères, avancer l'indice
            _ => {
                i += 1;
            }
        }
    }

    tokens.push(Token::EOF); // Marque la fin du fichier
    tokens
}
