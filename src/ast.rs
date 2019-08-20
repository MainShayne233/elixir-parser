pub type UntypedExpr = Expr;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Meta {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int { value: i64, meta: Meta },

    Float { value: f64, meta: Meta },

    String { value: String, meta: Meta },

    Atom { value: String, meta: Meta },
}
