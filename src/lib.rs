use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NesType {
    #[default]
    Undefined,
    Bool,
    Char,
    Int(IntType),
    Float(FloatType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntType {
    Signed8,
    Unsigned8,
    Signed16,
    Unsigned16,
    Signed32,
    Unsigned32,
    Signed64,
    Unsigned64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatType {
    Bit32,
    Bit64,
}

impl NesType {
    //constructors
    pub const fn bool() -> Self {
        Self::Bool
    }

    pub const fn char() -> Self {
        Self::Char
    }

    pub const fn u8() -> Self {
        Self::Int(IntType::Unsigned8)
    }

    pub const fn i8() -> Self {
        Self::Int(IntType::Signed8)
    }

    pub const fn u16() -> Self {
        Self::Int(IntType::Unsigned16)
    }

    pub const fn i16() -> Self {
        Self::Int(IntType::Signed16)
    }

    pub const fn u32() -> Self {
        Self::Int(IntType::Unsigned32)
    }

    pub const fn i32() -> Self {
        Self::Int(IntType::Signed32)
    }

    pub const fn u64() -> Self {
        Self::Int(IntType::Unsigned64)
    }

    pub const fn i64() -> Self {
        Self::Int(IntType::Signed64)
    }

    pub const fn f32() -> Self {
        Self::Float(FloatType::Bit32)
    }

    pub const fn f64() -> Self {
        Self::Float(FloatType::Bit64)
    }
    
    // TODO: Add type conversion.
    pub fn try_resolve(data_type1: NesType, data_type2: NesType) -> Option<NesType> {
        use NesType as T;
        match (data_type1, data_type2) {
            (T::Undefined, T::Undefined) => Some(T::Undefined),
            (T::Undefined, t) | (t, T::Undefined) => Some(t),
            (a, b) if a == b => Some(a),
            _ => None,
        }
    }
}
