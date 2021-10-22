use std::{ffi::CStr, fmt::Debug};

use crate::ast::Constant;
use crate::types::Type;



pub type GcHandle<T> = * const T;

pub fn get_ptr<T>(i: GcHandle<T>) -> *const T {
    let i = (i as u64) << 62 >> 62;
    i as *const T
}

pub struct ValueFatPointer {
    pub value: RawValue,
    pub type_: MetaData,
}

impl From<&Constant> for ValueFatPointer {
    fn from(i: &Constant) -> Self {
        ValueFatPointer {
            type_: MetaData::from(Type::from(i)),
            value: RawValue::from(i),
        }
    }
}

impl Into<Constant> for ValueFatPointer {
    fn into(self) -> Constant {
        let typ: Type = self.type_.into();
        unsafe {
            match typ {
                Type::None => Constant::None,
                Type::Bool => Constant::Bool(self.value.bool),
                Type::Int => Constant::Int(self.value.int),
                Type::Uint => Constant::Uint(self.value.uint),
                Type::Float => Constant::Float(self.value.float),
                Type::Str => todo!("impl value to constant string"),
                Type::Vec => todo!("impl value to constant vector"),
                Type::Map => todo!("impl value to constant map"),
            }
        }
    }
}

impl Debug for ValueFatPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ: Type = self.type_.into();
        unsafe {
            match typ {
                Type::None => write!(f, "None"),
                Type::Bool => write!(f, "{}: Bool", self.value.bool),
                Type::Int => write!(f, "{}: Int", self.value.int),
                Type::Uint => write!(f, "{}: Uint", self.value.uint),
                Type::Float => write!(f, "{}: Float", self.value.float),
                Type::Str => write!(f, "todo: String"),
                Type::Vec => write!(f, "todo: vector"),
                Type::Map => write!(f, "todo: map"),
            }
        }
    }
}

/// ## tagged pointer
/// 3bit tag  61bit pointer
/// -------------------------
/// | type |    metainfo    |
/// -------------------------
/// |  3   |       61       |
/// if tag > 5, then it is a pointer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetaData(* const MetaDataInfo);


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaDataInfo {
    pub tag: MetaDataInfoTag,
    pub ptr: GcHandle<()>,
}

/// # MetaDataInfoTag
/// 2 bits for type
///         2bit tag
/// ----------------
/// |       | type |
/// ----------------
/// |   6   |   2  |
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaDataInfoTag {
    OneTypeVector = 0b000,
    ManyTypeVector = 0b001,
    Map = 0b010,
}

impl From<MetaData> for Type {
    fn from(i: MetaData) -> Self {
        let r = i.0 as u64 >> 61;
        let r = r as u8;
        unsafe { std::mem::transmute(r) }
    }
}

impl From<Type> for MetaData {
    fn from(i: Type) -> Self {
        let r = (i as u64) << 61;
        unsafe { std::mem::transmute(r) }
    }
}

impl From<&Constant> for Type {
    fn from(i: &Constant) -> Self {
        match i {
            Constant::None => Type::None,
            Constant::Bool(_) => Type::Bool,
            Constant::Int(_) => Type::Int,
            Constant::Uint(_) => Type::Uint,
            Constant::Float(_) => Type::Float,
            Constant::String(_, _) => Type::Str,
        }
    }
}

pub union RawValue {
    pub none: (),
    pub bool: bool,
    pub int: i64,
    pub uint: u64,
    pub float: f64,
    pub str: GcHandle<CStr>,
    pub vec: GcHandle<Vector>,
    pub map: GcHandle<Vector>,
}



impl From<&Constant> for RawValue {
    fn from(i: &Constant) -> Self {
        match i {
            Constant::None => RawValue { none: () },
            Constant::Bool(v) => RawValue { bool: *v },
            Constant::Int(v) => RawValue { int: *v },
            Constant::Uint(v) => RawValue { uint: *v },
            Constant::Float(v) => RawValue { float: *v },
            Constant::String(_, _v) => todo!("impl string to raw value"),
        }
    }
}

pub type Vector = [RawValue];