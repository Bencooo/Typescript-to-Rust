// 1 - définition des tokens utilisés par le lexeur 

// Définit les différents types de tokens utilisés par le lexeur. 
// Chaque token représente une unité lexicale, comme les mots-clés 
// (function, let), les symboles (+, =, {, }), 
// ou les littéraux ("hello", 42).

use crate::utils::State;
use crate::utils::ValueType;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),         // Exemple : 
    Variable{
        name:String,
        value: ValueType,
        state: State,
    },
    Initialize{
        name:String,
        typevar: String,
        state:State,
    },
    Literal(String),
    Identifier(String),
    Function(String),
    Symbol(char),            // Symboles comme '(', ')', ';'
    Echap(char),
    Assign,                  // Symbole '='
    Operator(String),  
    EOF,                     // Fin de fichier
}