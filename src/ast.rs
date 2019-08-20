pub type UntypedExpr = Expr<()>;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Meta {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Pipe,
    And,
    Or,
    LtInt,
    LtEqInt,
    LtFloat,
    LtEqFloat,
    Eq,
    NotEq,
    GtEqInt,
    GtInt,
    GtEqFloat,
    GtFloat,
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MultInt,
    MultFloat,
    DivInt,
    DivFloat,
    ModuloInt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<T> {
    Int { value: i64, meta: Meta, typ: T },

    Float { value: f64, meta: Meta, typ: T },

    String { value: String, meta: Meta, typ: T },

    Atom { value: String, meta: Meta, typ: T },

    List {
        meta: Meta,
        typ: T,
        head: Box<Expr<T>>,
        tail: Box<Expr<T>>,
    },

    ListEnd {
        meta: Meta,
    },

    BinOp {
        meta: Meta,
        typ: T,
        name: BinOp,
        left: Box<Expr<T>>,
        right: Box<Expr<T>>,
    },
}


impl<T> Expr<T> {
    pub fn meta(&self) -> &Meta {
        match self {
            Expr::Int { meta, .. } => meta,
            Expr::List { meta, .. } => meta,
            Expr::ListEnd { meta, .. } => meta,
            Expr::Float { meta, .. } => meta,
            Expr::BinOp { meta, .. } => meta,
            Expr::String { meta, .. } => meta,
            Expr::Atom { meta, .. } => meta,
        }
    }
}
