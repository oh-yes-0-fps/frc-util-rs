use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use bytes::Bytes;
// use protobuf::descriptor::FileDescriptorProto;
use serde::{Deserialize, Serialize};

mod error;
pub mod structure;
#[cfg(test)]
mod test;
mod trait_impls;
mod traits;

pub use error::FrcValueError;
use structure::FrcStructDesc;
pub use traits::IntoFrcValue;

pub use bytes;
pub use inventory;
// pub use protobuf;

/// Measured in microseconds <p>
/// depending on source can be from unix epoch or some arbitrary start time
pub type FrcTimestamp = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrcType {
    Void,
    Boolean,
    Int,
    Double,
    Float,
    String,
    BoolArray,
    IntArray,
    FloatArray,
    DoubleArray,
    StringArray,
    Raw,
    Struct,
    // Protobuf,
}
impl Display for FrcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrcType::Void => write!(f, "Void"),
            FrcType::Boolean => write!(f, "Boolean"),
            FrcType::Int => write!(f, "Int"),
            FrcType::Double => write!(f, "Double"),
            FrcType::Float => write!(f, "Float"),
            FrcType::String => write!(f, "String"),
            FrcType::BoolArray => write!(f, "BoolArray"),
            FrcType::IntArray => write!(f, "IntArray"),
            FrcType::FloatArray => write!(f, "FloatArray"),
            FrcType::DoubleArray => write!(f, "DoubleArray"),
            FrcType::StringArray => write!(f, "StringArray"),
            FrcType::Raw => write!(f, "Raw"),
            FrcType::Struct => write!(f, "Struct"),
            // FrcType::Protobuf => write!(f, "Protobuf"),
        }
    }
}

impl Serialize for FrcType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string().to_lowercase().replace("array", "[]"))
    }
}

impl<'a> Deserialize<'a> for FrcType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'a>>::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "boolean" => Ok(FrcType::Boolean),
            "int" => Ok(FrcType::Int),
            "double" => Ok(FrcType::Double),
            "float" => Ok(FrcType::Float),
            "string" => Ok(FrcType::String),
            "json" => Ok(FrcType::String),
            "bool[]" => Ok(FrcType::BoolArray),
            "int[]" => Ok(FrcType::IntArray),
            "float[]" => Ok(FrcType::FloatArray),
            "double[]" => Ok(FrcType::DoubleArray),
            "string[]" => Ok(FrcType::StringArray),
            "raw" => Ok(FrcType::Raw),
            "rpc" => Ok(FrcType::Raw),
            "msgpack" => Ok(FrcType::Raw),
            // "protobuf" => Ok(FrcType::Protobuf),
            "struct" => Ok(FrcType::Struct),
            _ => Err(serde::de::Error::custom(format!("Invalid FrcType: {}", s))),
        }
    }
}

/// A stardized value type for FRC data piping
///
/// This enum is used to represent all possible values that can be sent over the FRC data piping system
/// including
/// - Void
/// - Boolean
/// - Int(i64)
/// - Double
/// - Float
/// - String
/// - BoolArray
/// - IntArray
/// - FloatArray
/// - DoubleArray
/// - StringArray
/// - Raw(Bytes)
/// - Struct
///
/// Struct and Protobuf are special types that carry metadata to allow them to be decoded into their inner types
///
/// Bytes are Boxed to keep the size of the enum small
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FrcValue {
    Void,
    Boolean(bool),
    Int(i64),
    Double(f64),
    Float(f32),
    String(String),
    BooleanArray(Vec<bool>),
    IntArray(Vec<i64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<String>),
    Raw(Box<Bytes>),
    #[serde(skip_deserializing)]
    Struct(#[serde(skip)] &'static FrcStructDesc, Box<Bytes>),
    // #[serde(skip_deserializing)]
    // Protobuf(#[serde(skip)] &'static FileDescriptorProto, Box<Bytes>),
}
impl Display for FrcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrcValue::Void => write!(f, "Void"),
            FrcValue::Boolean(v) => write!(f, "{}", v),
            FrcValue::Int(v) => write!(f, "{}", v),
            FrcValue::Double(v) => write!(f, "{}", v),
            FrcValue::Float(v) => write!(f, "{}", v),
            FrcValue::String(v) => write!(f, "{}", v),
            FrcValue::BooleanArray(v) => write!(f, "{:?}", v),
            FrcValue::IntArray(v) => write!(f, "{:?}", v),
            FrcValue::FloatArray(v) => write!(f, "{:?}", v),
            FrcValue::DoubleArray(v) => write!(f, "{:?}", v),
            FrcValue::StringArray(v) => write!(f, "{:?}", v),
            FrcValue::Raw(v) => write!(f, "{:?}", v),
            FrcValue::Struct(desc, data) => write!(f, "Struct({}):{:?}", desc.type_str, data),
            // FrcValue::Protobuf(proto, data) => write!(
            //     f,
            //     "Protobuf({}):{:?}",
            //     proto.name.clone().unwrap_or("unknown".into()),
            //     data
            // ),
        }
    }
}
impl Hash for FrcValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FrcValue::Void => {}
            FrcValue::Boolean(v) => v.hash(state),
            FrcValue::Int(v) => v.hash(state),
            FrcValue::Double(v) => v.to_bits().hash(state),
            FrcValue::Float(v) => v.to_bits().hash(state),
            FrcValue::String(v) => v.hash(state),
            FrcValue::BooleanArray(v) => v.hash(state),
            FrcValue::IntArray(v) => v.hash(state),
            FrcValue::FloatArray(v) => v.iter().for_each(|v| v.to_bits().hash(state)),
            FrcValue::DoubleArray(v) => v.iter().for_each(|v| v.to_bits().hash(state)),
            FrcValue::StringArray(v) => v.hash(state),
            FrcValue::Raw(v) => v.hash(state),
            FrcValue::Struct(desc, data) => {
                desc.schema.hash(state);
                desc.type_str.hash(state);
                data.hash(state);
            } // FrcValue::Protobuf(proto, data) => {
              //     proto.name.hash(state);
              //     data.hash(state);
              // }
        }
    }
}
impl FrcValue {
    ///Returns the type enum of the value, a more memory efficient way of checking the type
    pub fn get_type(&self) -> FrcType {
        match self {
            FrcValue::Void => FrcType::Void,
            FrcValue::Boolean(_) => FrcType::Boolean,
            FrcValue::Int(_) => FrcType::Int,
            FrcValue::Double(_) => FrcType::Double,
            FrcValue::Float(_) => FrcType::Float,
            FrcValue::String(_) => FrcType::String,
            FrcValue::BooleanArray(_) => FrcType::BoolArray,
            FrcValue::IntArray(_) => FrcType::IntArray,
            FrcValue::FloatArray(_) => FrcType::FloatArray,
            FrcValue::DoubleArray(_) => FrcType::DoubleArray,
            FrcValue::StringArray(_) => FrcType::StringArray,
            FrcValue::Raw(_) => FrcType::Raw,
            FrcValue::Struct(_, _) => FrcType::Struct,
            // FrcValue::Protobuf(_, _) => FrcType::Protobuf,
        }
    }
    ///Creates an empty Binary
    pub fn empty() -> Self {
        Self::Void
    }
    ///always false if not binary, array or string
    pub fn is_empty(&self) -> bool {
        match self {
            FrcValue::Void => true,
            FrcValue::String(v) => v.is_empty(),
            FrcValue::BooleanArray(v) => v.is_empty(),
            FrcValue::IntArray(v) => v.is_empty(),
            FrcValue::DoubleArray(v) => v.is_empty(),
            FrcValue::FloatArray(v) => v.is_empty(),
            FrcValue::StringArray(v) => v.is_empty(),
            FrcValue::Raw(v) => v.is_empty(),
            FrcValue::Struct(_, v) => v.is_empty(),
            // FrcValue::Protobuf(_, v) => v.is_empty(),
            _ => false,
        }
    }
    ///Binary is false
    pub fn is_array(&self) -> bool {
        match self {
            FrcValue::BooleanArray(_) => true,
            FrcValue::IntArray(_) => true,
            FrcValue::DoubleArray(_) => true,
            FrcValue::FloatArray(_) => true,
            FrcValue::StringArray(_) => true,
            _ => false,
        }
    }
    /// Consumes itself to a timestamped value with the given timestamp
    pub fn to_timestamped(self, timestamp: FrcTimestamp) -> FrcTimestampedValue {
        FrcTimestampedValue::new(timestamp, self)
    }
    /// Clones itself to a timestamped value with the given timestamp
    pub fn as_timestamped(&self, timestamp: FrcTimestamp) -> FrcTimestampedValue {
        FrcTimestampedValue::new(timestamp, self.clone())
    }
    pub fn to_tagged(self) -> FrcTaggedValue {
        FrcTaggedValue {
            r#type: self.get_type(),
            value: self,
        }
    }
    /// Creates a default value based on the type
    ///
    /// Types that will return none:
    ///     - Void
    ///     - Struct
    pub fn default_value(r#type: FrcType) -> Option<Self> {
        match r#type {
            FrcType::Void => None,
            FrcType::Boolean => Some(FrcValue::Boolean(false)),
            FrcType::Int => Some(FrcValue::Int(0)),
            FrcType::Double => Some(FrcValue::Double(0.0)),
            FrcType::Float => Some(FrcValue::Float(0.0)),
            FrcType::String => Some(FrcValue::String(String::new())),
            FrcType::BoolArray => Some(FrcValue::BooleanArray(Vec::new())),
            FrcType::IntArray => Some(FrcValue::IntArray(Vec::new())),
            FrcType::FloatArray => Some(FrcValue::FloatArray(Vec::new())),
            FrcType::DoubleArray => Some(FrcValue::DoubleArray(Vec::new())),
            FrcType::StringArray => Some(FrcValue::StringArray(Vec::new())),
            FrcType::Raw => Some(FrcValue::Raw(Box::new(Bytes::new()))),
            FrcType::Struct => None, // FrcType::Protobuf => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct FrcTaggedValue {
    #[serde(rename = "type")]
    pub r#type: FrcType,
    pub value: FrcValue,
}
impl Display for FrcTaggedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.r#type, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct FrcTimestampedValue {
    pub timestamp: FrcTimestamp,
    pub value: FrcValue,
}
impl Display for FrcTimestampedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.value, self.timestamp)
    }
}
impl FrcTimestampedValue {
    pub fn new(timestamp: FrcTimestamp, value: FrcValue) -> Self {
        Self { timestamp, value }
    }
    pub fn get_type(&self) -> FrcType {
        self.value.get_type()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn is_array(&self) -> bool {
        self.value.is_array()
    }
    pub fn is_after_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp > timestamp
    }
    pub fn is_after_other(&self, other: &Self) -> bool {
        self.timestamp > other.timestamp
    }
    pub fn is_before_timestamp(&self, timestamp: FrcTimestamp) -> bool {
        self.timestamp < timestamp
    }
    pub fn is_before_other(&self, other: &Self) -> bool {
        self.timestamp < other.timestamp
    }
    pub fn replace_timestamp(&mut self, timestamp: FrcTimestamp) {
        self.timestamp = timestamp;
    }
    pub fn replace_value(&mut self, value: FrcValue) {
        self.value = value;
    }
    pub fn replace(&mut self, other: Self) {
        self.timestamp = other.timestamp;
        self.value = other.value;
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FrcTimeline(Vec<FrcTimestampedValue>);

// impl IntoIterator for FrcTimeline {
//     type Item = FrcTimestampedValue;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl FrcTimeline {
//     pub fn new() -> Self {
//         Self(Vec::new())
//     }
//     pub fn from_vec_sorted(vec: Vec<FrcTimestampedValue>) -> Self {
//         Self(vec)
//     }
//     pub fn from_vec(mut vec: Vec<FrcTimestampedValue>) -> Self {
//         vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
//         Self(vec)
//     }
//     pub fn to_vec(self) -> Vec<FrcTimestampedValue> {
//         self.0
//     }
//     pub fn is_all_same_type(&self) -> bool {
//         if self.0.is_empty() {
//             return true;
//         }
//         let first_type = self.0[0].get_type();
//         self.0.iter().all(|v| v.get_type() == first_type)
//     }
//     pub fn is_all_same_type_as(&self, other: &FrcType) -> bool {
//         if self.0.is_empty() {
//             return true;
//         }
//         self.0.iter().all(|v| v.get_type() == *other)
//     }
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }
//     /// if closest above will get the value with the closest timestamp that is after the given timestamp
//     /// if closest above is false, will get the value with the closest timestamp that is before the given timestamp
//     pub fn get_by_timestamp(
//         &self,
//         timestamp: u64,
//         closest_after: bool,
//     ) -> Option<&FrcTimestampedValue> {
//         if closest_after {
//             if timestamp < self.0[0].timestamp {
//                 return None;
//             }
//         } else {
//             if timestamp > self.0[self.0.len() - 1].timestamp {
//                 return None;
//             }
//         }
//         //use a bisect algorithm to find the closest value
//         let mut low = 0;
//         let mut high = self.0.len() - 1;
//         while low <= high {
//             let mid = (low + high) / 2;
//             if self.0[mid].timestamp < timestamp {
//                 low = mid + 1;
//             } else if self.0[mid].timestamp > timestamp {
//                 high = mid - 1;
//             } else {
//                 return Some(&self.0[mid]);
//             }
//         }
//         if low == self.0.len() {
//             return None;
//         }
//         if closest_after {
//             Some(&self.0[low])
//         } else {
//             Some(&self.0[low - 1])
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FrcTableInstant {
//     #[serde(flatten)]
//     pub values: HashMap<String, FrcTimestampedValue>, //just now
// }
// impl Display for FrcTableInstant {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{")?;
//         for (i, (k, v)) in self.values.iter().enumerate() {
//             if i != 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}: {}", k, v)?;
//         }
//         write!(f, "}}")
//     }
// }
// impl FrcTableInstant {
//     pub fn new_slim() -> Self {
//         Self {
//             values: HashMap::new(),
//         }
//     }
//     pub fn new() -> Self {
//         Self {
//             values: HashMap::new(),
//         }
//     }
//     pub fn from_tuples(mut tuples: Vec<(impl ToString, FrcTimestampedValue)>) -> Self {
//         let mut values = HashMap::new();
//         tuples.reverse();
//         for (k, v) in tuples {
//             values.insert(k.to_string(), v);
//         }
//         Self { values }
//     }
//     pub fn set_field(&mut self, name: impl ToString, value: FrcTimestampedValue) {
//         self.values.insert(name.to_string(), value);
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct FrcTableHistory {
//     #[serde(flatten)]
//     pub values: HashMap<String, FrcTimeline>, //all values that have occured
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", content = "table", rename_all = "lowercase")]
// pub enum FrcTable {
//     Instant(FrcTableInstant),
//     History(FrcTableHistory),
// }
