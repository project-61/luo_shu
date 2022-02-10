use ast::{FactDef, Symbol, Constant, GlobalStr, Match, Atom};
use interpreter::database::{Database, Fact};

mod ast;
mod frontend;
mod interpreter;

fn main() {
    test_query_fact();
    println!("successed!");
}

fn test_query_fact() {
    let db = Database::new();
    for _ in 1..11451419u64 {
        db.load_fact(FactDef {
            name: Symbol::new("test1"),
            values: vec![Constant::from(1u64), Constant::from(1u64)],
        });
    }
    let query = Match {
        name: Symbol::new("test1"),
        expr: vec![Atom::Variable(Symbol::new("a")), Atom::Variable(Symbol::new("b"))],
    };
    let empty_env = Vec::new();
    println!("{:?}", db.query_facts(query, &empty_env));
}