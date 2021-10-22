use std::{collections::HashMap, sync::RwLock};
use rayon::prelude::*;

use crate::{ast::{self, Constant, Expr, Handle, InferEnv, MatchExpr, Pattern, Query, Symbol}, types::Type};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(pub Symbol, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeKey(pub Vec<Option<Type>>);

pub type Lines<T> = Handle<RwLock<Vec<T>>>;


#[derive(Debug, Clone, Default)]

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

impl Default for Table<Fun> {
    fn default() -> Self {
        Self(Default::default())
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub facts: FactDB,
    pub rules: RuleDB,
    pub funs: FnDB,
}

impl Database {
    pub fn load_fact(&self, fact: ast::Fact) {
        let key = Key(fact.name.clone(), fact.values.len());
        let types = TypeKey(fact.values.par_iter().map(|x| Some(x.infer_type())).collect());
        let v = Fact(fact.values);
        if let Some(l) = self.facts.get_line(&key, &types) {
            l.write().unwrap().push(v);
        } else {
            let v = Handle::new(RwLock::new(vec![v]));
            let mut hm = HashMap::new();
            hm.insert(types, v);
            let hm = Handle::new(RwLock::new(hm));
            self.facts.0.write().unwrap().insert(key,hm);
        }
    }
    pub fn load_rule(&self, mp: ast::MatchProcedure) {
        let key = Key(mp.name.clone(), mp.args.len());
        let env: InferEnv = Default::default();
        let types = TypeKey(mp.args.par_iter().map(|x| x.infer_type(env.clone())).collect());
        let v = Rule {
            args: mp.args,
            body: mp.body,
        };
        if let Some(l) = self.rules.get_line(&key, &types) {
            l.write().unwrap().push(v);
        } else {
            let v = Handle::new(RwLock::new(vec![v]));
            let mut hm = HashMap::new();
            hm.insert(types, v);
            let hm = Handle::new(RwLock::new(hm));
            self.rules.0.write().unwrap().insert(key,hm);
        }
    }
    pub fn load_fn(&self, mp: ast::Function) {
        let key = Key(mp.name.clone(), mp.args.len());
        let env: InferEnv = Default::default();
        let types = TypeKey(mp.args.par_iter().map(|x| x.infer_type(env.clone())).collect());
        // let types = TypeKey(mp.args.par_iter().map(|x| x.infer_type(env.clone())).collect());
        let v = UserFn {
            args: mp.args,
            body: mp.body,
        };
        let v = Fun::User(v);
        if let Some(_l) = self.funs.get_line(&key, &types) {
            // log
            panic!("Function already defined");
        } else {
            let mut hm = HashMap::new();
            hm.insert(types, v);
            let hm = Handle::new(RwLock::new(hm));
            self.funs.0.write().unwrap().insert(key,hm);
        }
    }
    pub fn load_native_fn(&self, key: Key, types: TypeKey, v: NativeFn) {
        let v = Fun::Native(v);
        if let Some(_l) = self.funs.get_line(&key, &types) {
            // log
            panic!("Function already defined");
        } else {
            let mut hm = HashMap::new();
            hm.insert(types, v);
            let hm = Handle::new(RwLock::new(hm));
            self.funs.0.write().unwrap().insert(key,hm);
        }
    }
    pub fn query(&self, query: Query) -> HashMap<Symbol, Constant> {
        // let mut env = InferEnv::new();
        // let namespace = Default::default();

        todo!()
    }
}
