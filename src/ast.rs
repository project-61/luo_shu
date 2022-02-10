use std::{sync::Arc, fmt::{Display, Debug}};


pub type Handle<T> = Arc<T>;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol (pub GlobalStr);

impl Symbol {
    pub fn new(s: &str) -> Self {
        Symbol(GlobalStr::new(s))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct FactDef {
    pub name: Symbol,
    pub values: Vec<Constant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub name: Symbol,
    pub args: Vec<Atom>,
    pub matchs: Vec<Match>,
    pub not_matchs: Vec<NotMatch>,
    pub filter: Vec<Filter>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Variable(Symbol),
    // Wildcard, // _
    // Const(Constant),
    // TypeAssert(Handle<Pattern>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub name: Symbol,
    pub expr: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotMatch (pub Match);

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Eq(Atom, Atom),
    NotEq(Atom, Atom),
    Lt(Atom, Atom),
    Gt(Atom, Atom),
    LtEq(Atom, Atom),
    GtEq(Atom, Atom),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Wildcard,
    Const(Constant),
    Variable(Symbol),
    // And(Vec<Expr>),
    // Or(Vec<Expr>),
    // Not(Box<Expr>),
}

#[derive(Clone, Copy, Eq)]
pub struct GlobalStr (pub *const String);

impl GlobalStr {
    pub fn new(i: &str) -> GlobalStr {
        // todo: use interned string
        let s = Box::new(i.to_string());
        let s = Box::leak(s);
        GlobalStr(s)
    }
}

impl std::hash::Hash for GlobalStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe{ self.0.as_ref() }.unwrap().as_str().hash(state);
    }
}

impl Debug for GlobalStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", unsafe { self.0.as_ref().unwrap().as_str() })
    }
}

impl Display for GlobalStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { self.0.as_ref().unwrap().as_str() })
    }
}

impl PartialEq for GlobalStr {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            true
        } else {
            unsafe {
                self.0.as_ref() == other.0.as_ref()
            }
        }
    }
}

unsafe impl Send for GlobalStr {}
unsafe impl Sync for GlobalStr {}

#[derive(Clone, Copy)]
pub struct Constant {
    pub value: ConstValue,
    pub type_: ConstType,
}

impl From<()> for Constant {
    fn from(_: ()) -> Self {
        Constant {
            value: ConstValue { none: () },
            type_: ConstType::None,
        }
    }
}

impl From<bool> for Constant {
    fn from(v: bool) -> Self {
        Constant {
            value: ConstValue { bool: v },
            type_: ConstType::Bool,
        }
    }
}

impl From<i64> for Constant {
    fn from(v: i64) -> Self {
        Constant {
            value: ConstValue { int: v },
            type_: ConstType::Int,
        }
    }
}

impl From<u64> for Constant {
    fn from(v: u64) -> Self {
        Constant {
            value: ConstValue { uint: v },
            type_: ConstType::Uint,
        }
    }
}

impl From<f64> for Constant {
    fn from(v: f64) -> Self {
        Constant {
            value: ConstValue { float: v },
            type_: ConstType::Float,
        }
    }
}

impl From<GlobalStr> for Constant {
    fn from(v: GlobalStr) -> Self {
        Constant {
            value: ConstValue { string: v },
            type_: ConstType::String,
        }
    }
}

impl std::fmt::Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            match self.type_ {
                ConstType::None => write!(f, "None"),
                ConstType::Bool => write!(f, "{:?}", self.value.bool),
                ConstType::Int => write!(f, "{:?}", self.value.int),
                ConstType::Uint => write!(f, "{:?}", self.value.uint),
                ConstType::Float => write!(f, "{:?}", self.value.float),
                ConstType::String => write!(f, "{:?}", self.value.string),
            }
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            match self.type_ {
                ConstType::None   => write!(f, "None"),
                ConstType::Bool   => write!(f, "{}", self.value.bool),
                ConstType::Int    => write!(f, "{}", self.value.int),
                ConstType::Uint   => write!(f, "{}", self.value.uint),
                ConstType::Float  => write!(f, "{}", self.value.float),
                ConstType::String => write!(f, "{}", self.value.string),
            }
        }
    }
}

impl std::hash::Hash for Constant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_.hash(state);
        unsafe {
            match self.type_ {
                ConstType::None   => {},
                ConstType::Bool   => self.value.bool.hash(state),
                ConstType::Int    => self.value.int.hash(state),
                ConstType::Uint   => self.value.uint.hash(state),
                ConstType::Float  => (self.value.float as u64).hash(state),
                ConstType::String => self.value.string.hash(state),
            }
        }
    }
}

impl PartialEq for Constant {
    fn eq(&self, other: &Self) -> bool {
        if self.type_ != other.type_ {
            return false;
        }
        unsafe {
            match self.type_ {
                ConstType::None   => true,
                ConstType::Bool   => self.value.bool   == other.value.bool,
                ConstType::Int    => self.value.int    == other.value.int,
                ConstType::Uint   => self.value.uint   == other.value.uint,
                ConstType::Float  => self.value.float  == other.value.float,
                ConstType::String => self.value.string == other.value.string,
            }
        }
    }
}

impl Eq for Constant {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConstType {
    None = 0,
    Bool = 1,
    Int = 2,
    Uint = 3,
    Float = 4,
    String = 5,
}

#[derive(Clone, Copy)]
pub union ConstValue {
    pub none: (),
    pub bool: bool,
    pub int: i64,
    pub uint: u64,
    pub float: f64,
    pub string: GlobalStr,
}

unsafe impl Send for Constant {}
unsafe impl Sync for Constant {}
