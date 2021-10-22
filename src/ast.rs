use std::sync::Arc;

use crate::types::Type;


pub type Handle<T> = Arc<T>;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol (pub Handle<String>);

impl Symbol {
    pub fn new(s: &str) -> Self {
        Symbol(Arc::new(s.to_string()))
    }

    pub fn register() {

    }
}

pub type StrProc = Symbol;


#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    pub name: Symbol,
    pub values: Vec<Constant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchProcedure {
    pub name: Symbol,
    pub args: Vec<Pattern>,
    pub body: Vec<MatchExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub defs: Vec<Symbol>,
    pub body: Vec<MatchExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard, // _
    Variable(Symbol),
    Expr(Expr),
    TypeAssert(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub name: Symbol,
    pub expr: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Symbol,
    pub args: Vec<Pattern>,
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(Box<Constant>),
    Symbol(Symbol),
    Add(Vec<Expr>),
    Sub(Vec<Expr>),
    Mul(Vec<Expr>),
    Div(Vec<Expr>),
    Mod(Vec<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
    Xor(Vec<Expr>),
    Not(Box<Expr>),
    BitAnd(Vec<Expr>),
    BitOr(Vec<Expr>),
    BitXor(Vec<Expr>),
    BitNot(Vec<Expr>),
    StringJoin(Vec<Expr>),
    Call(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(StrProc, String),
    // Bytes(Bytes),
}
