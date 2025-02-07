use crate::token::Token;
use crate::utils::State;
use crate::utils::ValueType;

pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    let mut state : State = State::NoneState;
    let mut name = "";
    let mut echap = false;

    while i < chars.len() {
        match chars[i] {
            'c' if code[i..].starts_with("console.log") => {
                tokens.push(Token::Keyword("console.log".to_string()));
                i += "console.log".len(); // Avancer l'indice après "console.log"
                echap = true;
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
                } else {
                    if chars[i] == 'f' {
                        tokens.push(Token::Identifier("false".to_string()));
                        i += "false".len();
                    }else {
                        tokens.push(Token::Identifier("true".to_string()));
                        i += "true".len();
                    }
                }
                name = "";
                state = State::NoneState;
                echap = true;
            }

            'i' => {
                if code[i..].starts_with("if"){
                    tokens.push(Token::Keyword("if".to_string()));
                    echap = true;
                    i+= "if".len();
                }
            }

            '(' | ')'  => {
                tokens.push(Token::Symbol(chars[i]));
                i += 1;
            }

            '{' | '}' | ';' => {
                if echap || chars[i] == '}' {
                    tokens.push(Token::Echap(chars[i]));
                    echap = false;
                }else{
                    tokens.push(Token::Symbol(chars[i]));
                }
                i+=1;
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
                println!("{} ",i);
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Operator("==".to_string()));
                    i += 2;  // Incrémenter de 2 pour passer le "=="
                } else {
                    if state == State::NoneState{
                        println!("ASSIGN");
                        tokens.push(Token::Assign);
                        echap = true;
                    }
                    i+=1;
                }
            }

            // Détection des nombres (entiers et flottants)
            _ if chars[i].is_digit(10) => {
                let start = i;
                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                    i += 1;
                }
                let number = &code[start..i];
                if let Ok(n) = number.parse::<f64>() {
                    if name != "" && state != State::NoneState {
                        // Si le nom est défini et que l'état est défini (différent de NoneState)
                        tokens.push(Token::Variable { name: name.to_string(), value: ValueType::F64(number.parse().unwrap()), state: state });
                        echap = true;
                    } else if name == "" && state == State::NoneState {
                        // Si le nom est vide et l'état est NoneState
                        tokens.push(Token::Number(number.parse().unwrap()));
                    } else {
                        // Cas où le nombre est bien formé mais ne correspond pas aux autres conditions
                        println!("❌ ERREUR: Nombre mal formé {}", number);
                        break; // Sortir si le nombre est mal formé
                    }
                } else {
                    // Si la conversion du nombre échoue
                    println!("❌ ERREUR: Nombre mal formé {}", number);
                    break;
                }
                
                name = "";
                state = State::NoneState;
            }

            '"' => {
                let start = i + 1;
                if let Some(end_offset) = code[start..].find('"') {
                    let end_index = start + end_offset;
                    let literal_value = &code[start..end_index];

                    println!("✅ DEBUG: Lexer Détection de chaîne → {}", literal_value); // 🛠️ Debug
                    if name != "" && state != State::NoneState {
                        tokens.push(Token::Variable{name:name.to_string(),value:ValueType::String(literal_value.to_string()),state:state});
                        echap = true;
                    }else{
                        tokens.push(Token::Literal(literal_value.to_string()));
                    }

                    i = end_index+1;
                } else {
                    name = "";
                    state = State::NoneState;
                    println!("❌ ERREUR: Chaîne non fermée !");
                    break; // Sortie en cas d'erreur de chaîne non fermée
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
                if state != State::NoneState && name == ""{
                    name = ident;
                }else if state != State::NoneState && name != ""{
                    tokens.push(Token::Initialize{name:name.to_string(), typevar: ident.to_string(),state: state});
                    name = "";
                    state = State::NoneState;
                    echap = true;
                }else{
                    tokens.push(Token::Identifier(ident.to_string()));
                }
                i+= ident.len()-1;
                
            }

            // Autres caractères, avancer l'indice
            _ => {
                i += 1;
            }
        }
    }

    tokens.push(Token::EOF); // Marque la fin du fichier
    println!("{:?}",tokens);
    tokens

}
