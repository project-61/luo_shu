use ast::{FactDef, Symbol, Constant, Match, Atom};
use interpreter::database::Database;

mod ast;
mod frontend;
mod interpreter;

fn main() {
    test_query_fact();
    println!("successed!");
}

fn test_query_fact() {
    let db = Database::new();
    for i in 1..11u64 {
        db.load_fact(FactDef {
            name: Symbol::new("test1"),
            values: vec![Constant::from(i), Constant::from(i+1)],
        });
    }
    let query = Match {
        name: Symbol::new("test1"),
        expr: vec![Atom::Variable(Symbol::new("a")), Atom::Variable(Symbol::new("b"))],
    };
    let empty_env = Vec::new();
    println!("{:?}", db.query_facts(&query, &empty_env));
}