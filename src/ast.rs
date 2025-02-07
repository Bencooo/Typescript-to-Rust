// Le fichier AST va contenir les structures de données pour les langages TypeScript et Rust

use crate::utils::ValueType;
use crate::utils::State;


#[derive(Debug,Clone)]
pub enum TypeScriptNode {

    ConsoleLog(String),
    Keyword(String),

    VariableDeclaration {
        name: String,
        value: ValueType,
        state: State,
    },

    VariableInitialization {
        name: String,
        typevar : String,
        state: State,
    },

    Operator(String),
    Symbol(char),
    Echap(char),
    Assign,

    // IfStatement {
    //     condition: Vec<TypeScriptNode>,
    //     body: Vec<TypeScriptNode>,
    // },
}

#[derive(Debug,Clone)]
pub enum RustNode {

    Println(String),
    Keyword(String),

    VariableDeclaration {
        name: String,
        value: ValueType,
        state: State,
    },

    VariableInitialization {
        name: String,
        typevar: String,
        state: State,
    },

    Operator(String),
    Symbol(char),
    Echap(char),

    // IfStatement {
    //     condition: Vec<RustNode>,
    //     body: Vec<RustNode>,
    // },

}