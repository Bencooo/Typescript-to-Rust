// 1 - définition des tokens utilisés par le lexeur 

// Définit les différents types de tokens utilisés par le lexeur. 
// Chaque token représente une unité lexicale, comme les mots-clés 
// (function, let), les symboles (+, =, {, }), 
// ou les littéraux ("hello", 42).




#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),         // Exemple : "console" ou "log"
    Identifier(String),      // Nom de fonction ou variable, ici : "log"
    Literal(String),         // Chaîne de caractères, ici : "Hello, world!"
    Symbol(char),            // Symboles comme '(', ')', ';'
    EOF,                     // Fin de fichier
}
