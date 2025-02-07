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
    Assign,
    Operator(String),
    Symbol(char),
    Echap(char),


    // IfStatement {
    //     condition: Vec<TypeScriptNode>,
    //     body: Vec<TypeScriptNode>,
    // },
    // ForLoop {
    //     initialization: Option<Box<TypeScriptNode>>,
    //     condition: String,
    //     increment: Option<Box<TypeScriptNode>>, // ✅ Correction ici
    //     body: Vec<TypeScriptNode>,
    // },

    // WhileLoop {
    //     initialization: Option<Box<TypeScriptNode>>, // `let i = 0;`
    //     condition: String,                           // `i < 10`
    //     body: Vec<TypeScriptNode>,                   // Contenu de la boucle
    //     increment: Option<Box<TypeScriptNode>>,      // `i++`
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

    // ForLoop {
    //     initialization: Option<Box<TypeScriptNode>>,
    //     condition: String,
    //     increment: Option<Box<TypeScriptNode>>, // ✅ Correction ici
    //     body: Vec<TypeScriptNode>,
    // },

    // WhileLoop {
    //     initialization: Option<Box<TypeScriptNode>>, // `let i = 0;`
    //     condition: String,                           // `i < 10`
    //     body: Vec<TypeScriptNode>,                   // Contenu de la boucle
    //     increment: Option<Box<TypeScriptNode>>,      // `i++`
    // },

    Operator(String),
    Symbol(char),
    Echap(char),
    //Expression(String),

    // IfStatement {
    //     condition: Vec<RustNode>,
    //     body: Vec<RustNode>,
    // },

}