use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NesType {
    #[default]
    Undefined,
    Bool,
    Char,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl NesType {
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
