#[derive(Debug)]
pub enum Literal {
    Atom(String),
    Integer(i32),
    String(String),
}

