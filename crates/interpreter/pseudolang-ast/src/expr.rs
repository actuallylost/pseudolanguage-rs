use std::fmt::Debug;

#[derive(Debug)]
pub enum Expr {
    Op(Box<Expr>, Opcode, Box<Expr>),
    Number(i64),
}

#[derive(Debug)]
pub enum Opcode {
    Pow,
    Mul,
    Div,
    Mod,
    IntDiv,
    Add,
    Sub,
}
