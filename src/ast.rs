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

    // ✅ Ajout de la déclaration de variable
    VariableDeclaration {
        name: String,
        value: String, // Peut être un nombre ou une chaîne
    },

    // ✅ Ajout du support pour les conditions
    IfStatement {
        condition: String,
        body: Vec<TypeScriptNode>, // Liste d'instructions à exécuter
    },

    ForLoop {
        initialization: Option<Box<TypeScriptNode>>,
        condition: String,
        increment: Option<Box<TypeScriptNode>>, // ✅ Correction ici
        body: Vec<TypeScriptNode>,
    },

    WhileLoop {
        initialization: Option<Box<TypeScriptNode>>, // `let i = 0;`
        condition: String,                           // `i < 10`
        body: Vec<TypeScriptNode>,                   // Contenu de la boucle
        increment: Option<Box<TypeScriptNode>>,      // `i++`
    },
}
