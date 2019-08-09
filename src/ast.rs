pub type UntypedExpr = Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int { value: i64 },

    Float { value: f64 },

    String { value: String },

    Atom { value: String },
}
