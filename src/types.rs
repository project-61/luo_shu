

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
