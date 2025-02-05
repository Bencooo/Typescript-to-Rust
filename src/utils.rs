// Définir l'énumération `ValueType` qui peut contenir différents types
#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    String(String),
    F64(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Mutable,
    Immutable,
    NoneState,
}