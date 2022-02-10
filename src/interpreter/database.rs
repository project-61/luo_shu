use std::{collections::HashMap, sync::RwLock};
// use rayon::prelude::*;

use crate::ast::{self, Constant, Handle, Symbol, Rule};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(pub Symbol, pub usize);


pub type Lines<T> = Handle<RwLock<Vec<T>>>;


#[derive(Debug, Clone, Default)]

pub struct Table<T> (pub Handle<RwLock<
    HashMap<
        Key,
        T
        >
    >>);

impl<T: Clone> Table<T> {
    pub fn get_lines(&self, key: &Key) -> Option<T> {
        let r = self.0
            .read()
            .unwrap();
        r.get(key).cloned()
    }
}


pub type FactDB = Table<Lines<Fact>>;

pub type RuleDB = Table<Lines<Rule>>;


#[derive(Debug, Clone, Default)]
pub struct Fact(pub Vec<Constant>);


#[derive(Debug, Clone, Default)]
pub struct Database {
    pub facts: FactDB,
    pub rules: RuleDB,
}

impl Database {
    pub fn new() -> Self {
        Database {
            facts: Default::default(),
            rules: Default::default(),
        }
    }

    pub fn load_fact(&self, fact: ast::FactDef) {
        let key = Key(fact.name.clone(), fact.values.len());
        let v = Fact(fact.values);
        if let Some(l) = self.facts.get_lines(&key) {
            l.write().unwrap().push(v);
        } else {
            let v = Handle::new(RwLock::new(vec![v]));
            self.facts.0.write().unwrap().insert(key, v);
        }
    }
    pub fn load_rule(&self, v: Rule) {
        let key = Key(v.name.clone(), v.args.len());
        if let Some(l) = self.rules.get_lines(&key) {
            l.write().unwrap().push(v);
        } else {
            let v = Handle::new(RwLock::new(vec![v]));
            self.rules.0.write().unwrap().insert(key, v);
        }
    }
}
