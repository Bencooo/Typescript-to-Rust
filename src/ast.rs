// Le fichier AST va contenir les structures de données pour les langages TypeScript et Rust

use crate::utils::ValueType;
use crate::utils::State;


#[derive(Debug,Clone)]
pub enum TypeScriptNode {

    ConsoleLog(String),

    VariableDeclaration {
        name: String,
        value: ValueType,
        state: State,
    },

    // IfStatement {
    //     condition: String,
    //     body: Vec<TypeScriptNode>,
    // },
}

#[derive(Debug,Clone)]
pub enum RustNode {

    Println(String),

    VariableDeclaration {
        name: String,
        value: ValueType,
        state: State,
    },

    // IfStatement {
    //     condition: String,
    //     body: Vec<RustNode>,
    // },

}