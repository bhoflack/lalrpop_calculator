use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(x) => write!(fmt, "{:?}", x),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error")
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
