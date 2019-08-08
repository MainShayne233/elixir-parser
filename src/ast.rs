use std::fmt::{Debug, Error, Formatter};

pub enum AtomType {
    Colon,
}

pub enum Term {
    Atom(String, AtomType),
}

impl Debug for Term {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Term::*;
        match &*self {
            Atom(atom, AtomType::Colon) => write!(fmt, ":{}", atom),
        }
    }
}

// pub enum Expr {
//     Number(i32),
//     Op(Box<Expr>, Opcode, Box<Expr>),
// }
//
// pub enum Opcode {
//     Mul,
//     Div,
//     Add,
//     Sub,
// }
//
// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match &*self {
//             Number(x) => write!(fmt, "{:?}", x),
//             Op(lhs_expr, opcode, rhs_expr) => write!(fmt, "({:?} {:?} {:?})", lhs_expr, opcode, rhs_expr)
//         }
//     }
// }
//
// impl Debug for Opcode {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Opcode::*;
//         match *self {
//             Mul => write!(fmt, "*"),
//             Div => write!(fmt, "/"),
//             Add => write!(fmt, "+"),
//             Sub => write!(fmt, "-"),
//         }
//     }
// }
