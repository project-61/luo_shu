use std::{collections::{HashMap, HashSet}, sync::RwLock};

use rayon::prelude::*;

use crate::ast::{Constant, Handle, Match, Symbol, Atom, Rule, NotMatch};

use super::database::*;


pub type ValueEnv = Handle<RwLock<HashMap<Symbol, Constant>>>;

type TempFact = Vec<(Symbol, Constant)>;


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

    pub fn unify_const(&self, value: &Constant) -> Result<Option<(Symbol, Constant)>, ()> {
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
        for (atom, value) in this.iter().zip(table.iter()) {
            let res = atom.unify_const(value)?;
            if let Some((name, value)) = res {
                if let Some(x) = env.par_iter()
                .find_first(|(k, _)| k == &name)
                .map(|(_, v)| *v) {
                    if x != value {
                        return Err(());
                    }
                } else {
                    env.push((name, value));
                }
            }
        }
        Ok(env)
    }

    pub fn get_var(&self, params: &Atom) -> Result<Option<(Symbol, Atom)>, ()> {
        match (self, params) {
            (Atom::Variable(v), Atom::Const(c)) =>
                Ok(Some((v.clone(), params.clone()))),
            (Atom::Variable(v), Atom::Variable(c)) =>
                Ok(Some((v.clone(), params.clone()))),
            (Atom::Variable(v), Atom::Wildcard) =>
                Err(()),
            _ => Ok(None),
        }
    }

    pub fn get_vars(this: &[Self], params: &[Self]) ->
        Result<Vec<(Symbol, Atom)>, ()> {
            let mut env = Vec::new();
            for (atom, param) in this.iter().zip(params.iter()) {
                let res = atom.get_var(param)?;
                if let Some((name, value)) = res {
                    if let Some(x) = env.par_iter()
                    .find_first(|(k, _)| k == &name)
                    .map(|(_, v)| *v) {
                        if x != value {
                            return Err(());
                        }
                    } else {
                        env.push((name, value));
                    }
                }
            }
            Ok(env)
    }

    pub fn unify_argument(&self, argument: &Self) ->
        Result<Option<(Symbol, Constant)>, ()> {
        match (self, argument) {
            (Atom::Variable(v), Atom::Const(c)) =>
                Ok(Some((v.clone(), c.clone()))),
            (Atom::Const(_), Atom::Variable(_)) |
            (Atom::Variable(_), Atom::Variable(_)) =>
                Ok(None),
            (Atom::Const(c1), Atom::Const(c2)) => {
                if c1 == c2 {
                    Ok(None)
                } else {
                    Err(())
                }
            },
            _ => Ok(None),
        }
    }

    pub fn unify_arguments(this: &[Self], arguments: &[Self]) ->
        Result<TempFact, ()> {
            let mut env = Vec::new();
            for (atom, argument) in this.iter().zip(arguments.iter()) {
                let res = atom.unify_argument(argument)?;
                if let Some((name, value)) = res {
                    if let Some(x) = env.par_iter()
                    .find_first(|(k, _)| k == &name)
                    .map(|(_, v)| *v) {
                        if x != value {
                            return Err(());
                        }
                    } else {
                        env.push((name, value));
                    }
                }
            }
            Ok(env)
    }
}

impl Database {
    pub fn query(&self, query: &Match, env: &TempFact) -> HashSet<TempFact> {
        let r = self.query_facts(query, env);
        if r.is_empty() {
            self.query_rules(query, env)
        } else {
            r
        }
    }

    pub fn not_query(&self, query: &NotMatch, env: &TempFact) -> bool {
        self.query_facts(&query.0, env).is_empty()
    }

    pub fn query_facts(&self, query: &Match, env: &TempFact) -> HashSet<TempFact> {
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

    pub fn query_rules(&self, query: &Match, env: &TempFact) -> HashSet<TempFact> {
        let key = Key(query.name.clone(), query.expr.len());
        let rule_list = self.rules
            .get_lines(&key);
        if rule_list.is_none() {
            return HashSet::new();
        }
        let rule_list = rule_list.unwrap();
        let rule_list = rule_list.read().unwrap();
        let match_table: Vec<Atom> = query.expr.par_iter().map(|x| x.eval(&env)).collect();
        let r: HashSet<TempFact> = rule_list.par_iter().flat_map(|r| {
            let mut env = Atom::unify_arguments(&match_table, &r.args).unwrap(); // todo
            let res: Vec<_> = r.matchs.par_iter()
                .fold(
                    || [env].into_par_iter().collect::<HashSet<_>>(),
                    |env, m| env.par_iter()
                        .flat_map(|env| self.query(m, &env))
                        .collect())
                .flatten()
                .filter(|env| r.not_matchs
                    .par_iter()
                    .map(|nm| self.not_query(nm, env))
                    .all(|x| x))
                // .filter(|env| r.filter
                    // .par_iter()
                    // .map(|filter|))
                .collect();
            res
        }).collect();
        r
    }
}