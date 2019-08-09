#[derive(Debug)]
pub enum Primative {
    Boolean(bool),
    Atom(String),
    Integer(i32),
    String(String),
}

