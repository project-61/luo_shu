use std::{collections::HashMap, sync::RwLock};

use crate::{ast::{Constant, Expr, Handle, MatchExpr, Pattern, Symbol}, types::Type};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(pub Symbol, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeKey(pub Vec<Option<Type>>);

pub type Lines<T> = Handle<RwLock<Vec<T>>>;


#[derive(Debug, Clone)]

pub struct Table<T> (pub Handle<RwLock<HashMap<
    Key,
    Handle<RwLock<HashMap<
        TypeKey,
        T>>>>>>);

impl<T: Clone> Table<T> {
    fn get_line(&self, key: &Key, type_key: &TypeKey) -> Option<T> {
        self.0
            .read()
            .unwrap()
            .get(key)
            .and_then(|x|
                x.read()
                .unwrap()
                .get(type_key).cloned())
    }
}


pub type FactDB = Table<Lines<Fact>>;

pub type RuleDB = Table<Lines<Rule>>;

pub type FnDB = Table<Fun>;


#[derive(Debug, Clone)]
pub struct Fact(pub Vec<Constant>);

#[derive(Debug, Clone)]
pub struct Rule {
    pub args: Vec<Pattern>,
    pub body: Vec<MatchExpr>,
}

#[derive(Debug, Clone)]
pub enum Fun {
    User(UserFn),
    Native(NativeFn),
}

#[derive(Debug, Clone)]
pub struct UserFn {
    pub args: Vec<Pattern>,
    pub body: Vec<Expr>,
}

pub type NativeFn = extern "C" fn(Vec<Constant>) -> Constant;

#[derive(Debug, Clone)]
pub struct Database {
    pub facts: FactDB,
    pub rules: RuleDB,
    pub funs: FnDB,
}
