use std::{collections::HashMap, io::Read, slice::SliceIndex, sync::{Arc, RwLock}};

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
    Expr(Handle<Expr>),
    TypeAssert(Handle<Pattern>, Type),
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
    BitNot(Box<Expr>),
    StringJoin(Vec<Expr>),
    Call(Symbol, Vec<Expr>),
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

impl Constant {
    pub fn is_none(&self) -> bool {
        match self {
            Constant::None => true,
            _ => false,
        }
    }
    pub fn infer_type(&self) -> Type {
        match self {
            Constant::None => Type::None,
            Constant::Bool(_) => Type::Bool,
            Constant::Int(_) => Type::Int,
            Constant::Uint(_) => Type::Uint,
            Constant::Float(_) => Type::Float,
            Constant::String(_, _) => Type::Str,
            // Constant::Bytes(_) => Type::Bytes,
        }
    }
}

pub type InferEnv = Arc<RwLock<HashMap<Symbol, Type>>>;

impl Pattern {
    pub fn infer_type(&self, env: InferEnv) -> Option<Type> {
        match self {
            Pattern::Wildcard => None,
            Pattern::Variable(x) => env.read().unwrap().get(x).cloned(),
            Pattern::Expr(e) => e.infer_type(env),
            Pattern::TypeAssert(e, t) =>
                e.infer_type(env).and_then(|ty| ty.simple_unify(t)),
        }
    }
}


impl Expr {
    pub fn infer_type(&self, env: InferEnv) -> Option<Type> {
        match self {
            Expr::Const(c) => Some(c.infer_type()),
            Expr::Symbol(s) => env.read().unwrap().get(s).cloned(),
            Expr::Add(e)    |
            Expr::Sub(e)    |
            Expr::Mul(e)    |
            Expr::Div(e)    |
            Expr::Mod(e)    |
            Expr::And(e)    |
            Expr::Or(e)     |
            Expr::Xor(e)    |
            Expr::BitAnd(e) |
            Expr::BitOr(e)  |
            Expr::BitXor(e) => e.iter()
                .map(|e| e.infer_type(env.clone()))
                .reduce(|e1, e2|
                    e1.and_then(|ty1| e2.and_then(|ty2| {
                        if ty1.type_assert(&ty2) {
                            Some(ty1)
                        } else {
                            // log
                            None
                        }}))).flatten(),
            Expr::Not(e) |
            Expr::BitNot(e) => e.infer_type(env),
            Expr::StringJoin(_e) => Some(Type::Str),
            Expr::Call(c, es) => todo!(),
        }
    }
}