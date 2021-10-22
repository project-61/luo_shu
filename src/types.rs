use std::collections::HashSet;



/// 3 bits for type
///         3bit tag
/// ----------------
/// |       | type |
/// ----------------
/// |   5   |   3  |
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    None = 0,
    Bool = 1,
    Int = 2,
    Uint = 3,
    Float = 4,
    Str = 5,
    Vec = 6,
    Map = 7,
}

impl Type {
    pub fn type_assert(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::None, Type::None) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Int, Type::Int) => true,
            (Type::Uint, Type::Uint) => true,
            (Type::Float, Type::Float) => true,
            (Type::Str, Type::Str) => true,
            (Type::Vec, Type::Vec) => true,
            (Type::Map, Type::Map) => true,
            _ => false,
        }
    }
    pub fn simple_unify(&self, other: &Type) -> Option<Type> {
        if self.type_assert(other) {
            Some(self.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HintType {
    Typ(Type),
    Union(HashSet<Type>),
    Any,
}

impl From<Type> for HintType {
    fn from(i: Type) -> Self {
        HintType::Typ(i)
    }
}

impl HintType {
    pub fn type_assert(&self, other: &HintType) -> bool {
        match (self, other) {
            (HintType::Typ(t1), HintType::Typ(t2)) => t1.type_assert(t2),
            (HintType::Union(ts), HintType::Typ(t1)) => ts.contains(&t1),
            (HintType::Typ(t1), HintType::Union(ts)) =>
            if ts.contains(&t1) {
                if ts.len() != 1 {
                    log::warn!("Union contains more than one type");
                }
                true
            } else {
                false
            },
            (HintType::Union(t1), HintType::Union(t2)) => t2.is_subset(t1),
            (HintType::Any, _) => true,
            _ => false,
        }
    }
}

