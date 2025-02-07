
mod token;
mod lexer;
mod parser;
mod ast;
mod transformer;
mod generator;
mod utils;

use lexer::lex;
use parser::parse;
use transformer::transform;
use generator::generate_code;
use ast::RustNode;
use std::fs::{File, read_to_string};
use std::io::Write;

fn main() {
    // Lire le code TypeScript du fichier input.ts
    let input_path = "examples/input.ts"; // Chemin vers le fichier TypeScript
    let ts_code = read_to_string(input_path).expect("Unable to read input.ts");
    
    // Étapes de la transformation
    let tokens = lex(&ts_code);
    let ast = parse(tokens);
    let rust_ast = ast.into_iter().map(transform).collect::<Vec<RustNode>>();
    let rust_code = generate_code(rust_ast);

    // Afficher le code Rust généré dans la console (optionnel)
    println!("Generated Rust code:\n{}", rust_code);
    
    let output_path = "examples/output.rs"; // Chemin vers le fichier de sortie
    let mut file = File::create(output_path).expect("Unable to create file");
    file.write_all(rust_code.as_bytes()).expect("Unable to write data");
    
    println!("Conversion terminée!");
}
