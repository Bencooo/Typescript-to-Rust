// 3 - Définition de l'AST pour les deux anguage (ts et rust)

// Définit les structures de données pour les deux AST 
// (TypeScript et Rust). Ce fichier centralise la représentation 
// abstraite des éléments syntaxiques de chaque langage, 
// permettant au parser et au transformateur de partager des 
// structures communes.


// src/ast.rs

#[derive(Debug, Clone)]
pub enum TypeScriptNode {
    ConsoleLog(String),   // Ex : console.log("Hello, world!")
}
