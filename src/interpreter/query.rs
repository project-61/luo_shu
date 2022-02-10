use std::{collections::{HashMap, HashSet}, sync::RwLock};

use rayon::prelude::*;

use crate::ast::{Constant, Handle, Match, Symbol, Atom};

use super::database::*;


pub type ValueEnv = Handle<RwLock<HashMap<Symbol, Constant>>>;

type TempFact = Vec<(Symbol, Constant)>;

/*
impl Match {
    pub fn query(&self, db: Database, env: TempFact) -> HashSet<TempFact> {
        let key = Key(self.name.clone(), self.expr.len());
        let env: ValueEnv = Default::default();

        /*
        let values: Vec<Pattern> = self.expr.iter().map(|x| x.query(env.clone())).collect();
        db.rules.get_line(&key);
        */
        todo!()
    }
}
 */

impl Atom {
    pub fn eval(&self, env: &TempFact) -> Self {
        match self {
            Atom::Variable(name) => {
                env.par_iter().find_first(|(n, _)| n == name)
                    .map(|(_, c)| Atom::Const(c.clone()))
                    .unwrap_or_else(|| self.clone())
            }
            _ => self.clone(),
        }
    }

    pub fn unify(&self, value: &Constant) -> Result<Option<(Symbol, Constant)>, ()> {
        match (self, value) {
            (Atom::Variable(name), value) => Ok(Some((name.clone(), value.clone()))),
            (Atom::Const(c1), c2) => {
                if c1 == c2 {
                    Ok(None)
                } else {
                    Err(())
                }
            }
            _ => Ok(None),
        }
    }

    pub fn unify_fact(this: &[Self], table: &[Constant]) -> Result<TempFact, ()> {
        let mut env = Vec::new();
        for (i, atom) in this.iter().enumerate() {
            let value = table.get(i).ok_or_else(|| ())?;
            let res = atom.unify(value)?;
            if let Some((name, value)) = res {
                env.push((name, value));
            }
        }
        Ok(env)
    }
}

impl Database {
    pub fn query(&self, query: Match, env: &TempFact) -> Result<HashSet<TempFact>, ()> {

        todo!()
    }

    pub fn query_facts(&self, query: Match, env: &TempFact) -> HashSet<TempFact> {
        let key = Key(query.name.clone(), query.expr.len());
        let fact_list = self.facts
            .get_lines(&key);
        if fact_list.is_none() {
            return HashSet::new();
        }
        let fact_list = fact_list.unwrap();
        let fact_list = fact_list.read().unwrap();
        let match_table: Vec<Atom> = query.expr.par_iter().map(|x| x.eval(env)).collect();
        let match_table = &match_table as &[Atom];
        let r: HashSet<TempFact> = fact_list.par_iter().map(|Fact(t)| {
            let mut r = Atom::unify_fact(match_table, t)?;
            r.extend(env.clone());
            Ok(r)
        })
        .filter(|x: &Result<_, ()>| x.is_ok())
        .map(|x| x.unwrap()).collect();
        r
    }

    pub fn query_rules(&self, query: Match, env: TempFact) -> Result<HashSet<TempFact>, ()> {
        let key = Key(query.name.clone(), query.expr.len());

        todo!()
    }
}