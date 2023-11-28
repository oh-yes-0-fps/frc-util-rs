use crate::{FrcTimestampedValue, FrcValue, error::{CastErrorReason, FrcValueError}};

impl From<f64> for FrcValue {
    fn from(v: f64) -> Self {
        FrcValue::Double(v)
    }
}
impl From<f32> for FrcValue {
    fn from(v: f32) -> Self {
        FrcValue::Float(v)
    }
}
impl From<i64> for FrcValue {
    fn from(v: i64) -> Self {
        FrcValue::Int(v)
    }
}
impl From<i32> for FrcValue {
    fn from(v: i32) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<i16> for FrcValue {
    fn from(v: i16) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<i8> for FrcValue {
    fn from(v: i8) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<u64> for FrcValue {
    fn from(v: u64) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<u32> for FrcValue {
    fn from(v: u32) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<u16> for FrcValue {
    fn from(v: u16) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<u8> for FrcValue {
    fn from(v: u8) -> Self {
        FrcValue::Int(v as i64)
    }
}
impl From<bool> for FrcValue {
    fn from(v: bool) -> Self {
        FrcValue::Boolean(v)
    }
}
impl From<String> for FrcValue {
    fn from(v: String) -> Self {
        FrcValue::String(v)
    }
}
impl From<&str> for FrcValue {
    fn from(v: &str) -> Self {
        FrcValue::String(v.to_string())
    }
}
impl From<Vec<bool>> for FrcValue {
    fn from(v: Vec<bool>) -> Self {
        FrcValue::BooleanArray(v)
    }
}
impl From<Vec<i64>> for FrcValue {
    fn from(v: Vec<i64>) -> Self {
        FrcValue::IntArray(v)
    }
}
impl From<Vec<i32>> for FrcValue {
    fn from(v: Vec<i32>) -> Self {
        FrcValue::IntArray(v.iter().map(|v| *v as i64).collect())
    }
}
impl From<Vec<u64>> for FrcValue {
    fn from(v: Vec<u64>) -> Self {
        FrcValue::IntArray(v.iter().map(|v| *v as i64).collect())
    }
}
impl From<Vec<u32>> for FrcValue {
    fn from(v: Vec<u32>) -> Self {
        FrcValue::IntArray(v.iter().map(|v| *v as i64).collect())
    }
}
impl From<Vec<f32>> for FrcValue {
    fn from(v: Vec<f32>) -> Self {
        FrcValue::FloatArray(v)
    }
}
impl From<Vec<f64>> for FrcValue {
    fn from(v: Vec<f64>) -> Self {
        FrcValue::DoubleArray(v)
    }
}
impl From<Vec<String>> for FrcValue {
    fn from(v: Vec<String>) -> Self {
        FrcValue::StringArray(v)
    }
}
impl From<Vec<&str>> for FrcValue {
    fn from(v: Vec<&str>) -> Self {
        FrcValue::StringArray(v.iter().map(|s| s.to_string()).collect())
    }
}

impl Into<FrcValue> for FrcTimestampedValue {
    fn into(self) -> FrcValue {
        self.value
    }
}

impl TryFrom<FrcValue> for f64 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Double(v) => Ok(v),
            FrcValue::Float(v) => Ok(v as f64),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(f64),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for f32 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Double(v) => {
                if v > f32::MAX as f64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(f32),
                        CastErrorReason::Overflow
                    ))
                } else if v < f32::MIN as f64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(f32),
                        CastErrorReason::Underflow
                    ))
                } else {
                    Ok(v as f32)
                }
            },
            FrcValue::Float(v) => Ok(v),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(f32),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for i64 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => Ok(v),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(i64),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for i32 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v > i32::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i32),
                        CastErrorReason::Overflow
                    ))
                } else if v < i32::MIN as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i32),
                        CastErrorReason::Underflow
                    ))
                } else {
                    Ok(v as i32)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(i32),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for i16 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v > i16::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i16),
                        CastErrorReason::Overflow
                    ))
                } else if v < i16::MIN as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i16),
                        CastErrorReason::Underflow
                    ))
                } else {
                    Ok(v as i16)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(i16),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for i8 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v > i8::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i8),
                        CastErrorReason::Overflow
                    ))
                } else if v < i8::MIN as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(i8),
                        CastErrorReason::Underflow
                    ))
                } else {
                    Ok(v as i8)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(i8),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for u64 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v < 0 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u64),
                        CastErrorReason::Underflow
                    ))
                } else {
                    Ok(v as u64)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(u64),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for u32 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v < 0 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u32),
                        CastErrorReason::Underflow
                    ))
                } else if v > u32::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u32),
                        CastErrorReason::Overflow
                    ))
                } else {
                    Ok(v as u32)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(u32),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for u16 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v < 0 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u16),
                        CastErrorReason::Underflow
                    ))
                } else if v > u16::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u16),
                        CastErrorReason::Overflow
                    ))
                } else {
                    Ok(v as u16)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(u16),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for u8 {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Int(v) => {
                if v < 0 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u8),
                        CastErrorReason::Underflow
                    ))
                } else if v > u8::MAX as i64 {
                    Err(FrcValueError::InvalidCast(
                        value.get_type(),
                        stringify!(u8),
                        CastErrorReason::Overflow
                    ))
                } else {
                    Ok(v as u8)
                }
            },
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(u8),
                CastErrorReason::Type
            ))
        }
    }
}

impl TryFrom<FrcValue> for bool {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::Boolean(v) => Ok(v),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(bool),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for String {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::String(v) => Ok(v),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(String),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<f64> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::DoubleArray(va) => Ok(va),
            FrcValue::FloatArray(va) => Ok(
                va.into_iter().map(|v| v as f64).collect()
            ),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<f64>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<f32> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::DoubleArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v > f32::MAX as f64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<f32>),
                            CastErrorReason::Overflow
                        ))
                    } else if *v < f32::MIN as f64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<f32>),
                            CastErrorReason::Underflow
                        ))
                    } else {
                        ret_vec.push(*v as f32);
                    }
                }
                Ok(ret_vec)
            }
            FrcValue::FloatArray(v) => Ok(v),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<f32>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i64> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(va) => Ok(va),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<i64>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i32> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v > i32::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i32>),
                            CastErrorReason::Overflow
                        ))
                    } else if *v < i32::MIN as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i32>),
                            CastErrorReason::Underflow
                        ))
                    } else {
                        ret_vec.push(*v as i32);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<i32>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i16> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v > i16::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i16>),
                            CastErrorReason::Overflow
                        ))
                    } else if *v < i16::MIN as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i16>),
                            CastErrorReason::Underflow
                        ))
                    } else {
                        ret_vec.push(*v as i16);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<i16>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<i8> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v > i8::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i8>),
                            CastErrorReason::Overflow
                        ))
                    } else if *v < i8::MIN as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<i8>),
                            CastErrorReason::Underflow
                        ))
                    } else {
                        ret_vec.push(*v as i8);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<i8>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u64> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v < 0 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u64>),
                            CastErrorReason::Underflow
                        ))
                    } else {
                        ret_vec.push(*v as u64);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<u64>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u32> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v < 0 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u32>),
                            CastErrorReason::Underflow
                        ))
                    } else if *v > u32::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u32>),
                            CastErrorReason::Overflow
                        ))
                    } else {
                        ret_vec.push(*v as u32);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<u32>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u16> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v < 0 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u16>),
                            CastErrorReason::Underflow
                        ))
                    } else if *v > u16::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u16>),
                            CastErrorReason::Overflow
                        ))
                    } else {
                        ret_vec.push(*v as u16);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<u16>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<u8> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::IntArray(ref va) => {
                let mut ret_vec = Vec::with_capacity(va.len());
                for v in va {
                    if *v < 0 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u8>),
                            CastErrorReason::Underflow
                        ))
                    } else if *v > u8::MAX as i64 {
                        return Err(FrcValueError::InvalidCast(
                            value.get_type(),
                            stringify!(Vec<u8>),
                            CastErrorReason::Overflow
                        ))
                    } else {
                        ret_vec.push(*v as u8);
                    }
                }
                Ok(ret_vec)
            }
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<u8>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<bool> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::BooleanArray(va) => Ok(va),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<bool>),
                CastErrorReason::Type
            )),
        }
    }
}

impl TryFrom<FrcValue> for Vec<String> {
    type Error = crate::FrcValueError;
    fn try_from(value: FrcValue) -> Result<Self, Self::Error> {
        match value {
            FrcValue::StringArray(va) => Ok(va),
            _ => Err(FrcValueError::InvalidCast(
                value.get_type(),
                stringify!(Vec<String>),
                CastErrorReason::Type
            )),
        }
    }
}





#[cfg(any(feature = "rmpv-casting", feature = "json-casting"))]
use crate::FrcType;

#[cfg(feature = "rmpv-casting")]
use rmpv::Value as MPValue;

#[cfg(feature = "rmpv-casting")]
impl TryFrom<MPValue> for FrcValue {
    type Error = crate::FrcValueError;
    fn try_from(value: MPValue) -> Result<Self, Self::Error> {
        match value {
            MPValue::Nil => Ok(Self::Void),
            MPValue::Boolean(b) => Ok(Self::Boolean(b)),
            MPValue::Integer(i) => Ok(Self::Int(i.as_i64().unwrap_or_default())),
            MPValue::F32(f) => Ok(Self::Float(f)),
            MPValue::F64(f) => Ok(Self::Double(f)),
            MPValue::String(s) => Ok(Self::String(s.to_string())),
            MPValue::Binary(b) => Ok(Self::Raw(Box::new(bytes::Bytes::from(b)))),
            MPValue::Array(a) => {
                let mut arr = Vec::with_capacity(a.len());
                for v in a {
                    arr.push(Self::try_from(v)?);
                }
                if arr.len() == 0 {
                    return Ok(Self::empty());
                }
                let first_type = arr[0].get_type();
                if arr.iter().all(|v| v.get_type() == first_type) {
                    match first_type {
                        FrcType::Boolean => Ok(Self::BooleanArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Int => Ok(Self::IntArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Float => Ok(Self::FloatArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Double => Ok(Self::DoubleArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::String => Ok(Self::StringArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        any => Err(FrcValueError::InvalidCast(
                            any,
                            stringify!(MPValue),
                            CastErrorReason::Type
                        ))
                    }
                } else {
                    Err(FrcValueError::InvalidCast(
                        first_type,
                        stringify!(MPValue),
                        CastErrorReason::Type
                    ))
                }
            },
            _ => Err(FrcValueError::UnrepresentableCast)
        }
    }
}

#[cfg(feature = "rmpv-casting")]
impl From<FrcValue> for MPValue {
    fn from(value: FrcValue) -> Self {
        match value {
            FrcValue::Void => Self::Nil,
            FrcValue::Boolean(b) => Self::Boolean(b),
            FrcValue::Int(i) => Self::Integer(i.into()),
            FrcValue::Float(f) => Self::F32(f),
            FrcValue::Double(f) => Self::F64(f),
            FrcValue::String(s) => Self::String(s.into()),
            FrcValue::Raw(b) => Self::Binary(b.to_vec()),
            FrcValue::BooleanArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Boolean(v))
                    .collect::<Vec<Self>>()
                    .into(),
            ),
            FrcValue::IntArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Integer(v.into()))
                    .collect::<Vec<Self>>()
                    .into(),
            ),
            FrcValue::FloatArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::F32(v))
                    .collect::<Vec<Self>>()
                    .into(),
            ),
            FrcValue::DoubleArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::F64(v))
                    .collect::<Vec<Self>>()
                    .into(),
            ),
            FrcValue::StringArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::String(v.into()))
                    .collect::<Vec<Self>>()
                    .into(),
            ),
            FrcValue::Struct(_, b) => Self::Binary(b.to_vec()),
        }
    }
}

#[cfg(feature = "json-casting")]
use serde_json::Value as JSONValue;


#[cfg(feature = "json-casting")]
impl TryFrom<JSONValue> for FrcValue {
    type Error = crate::FrcValueError;
    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        match value {
            JSONValue::Null => Ok(Self::Void),
            JSONValue::Bool(b) => Ok(Self::Boolean(b)),
            JSONValue::Number(n) => {
                if n.is_i64() {
                    Ok(Self::Int(n.as_i64().unwrap_or_default()))
                } else if n.is_f64() {
                    Ok(Self::Double(n.as_f64().unwrap_or_default()))
                } else {
                    Err(FrcValueError::InvalidCast(
                        FrcType::Double,
                        stringify!(JSONValue),
                        CastErrorReason::Type
                    ))
                }
            },
            JSONValue::String(s) => Ok(Self::String(s.to_string())),
            JSONValue::Array(a) => {
                let mut arr = Vec::with_capacity(a.len());
                for v in a {
                    arr.push(Self::try_from(v)?);
                }
                if arr.len() == 0 {
                    return Ok(Self::empty());
                }
                let first_type = arr[0].get_type();
                if arr.iter().all(|v| v.get_type() == first_type) {
                    match first_type {
                        FrcType::Boolean => Ok(Self::BooleanArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Int => Ok(Self::IntArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Float => Ok(Self::FloatArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::Double => Ok(Self::DoubleArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        FrcType::String => Ok(Self::StringArray(arr.into_iter().map(|v| v.try_into().unwrap()).collect())),
                        any => Err(FrcValueError::InvalidCast(
                            any,
                            stringify!(JSONValue),
                            CastErrorReason::Type
                        ))
                    }
                } else {
                    Err(FrcValueError::InvalidCast(
                        first_type,
                        stringify!(JSONValue),
                        CastErrorReason::Type
                    ))
                }
            },
            _ => Err(FrcValueError::UnrepresentableCast)
        }
    }
}

#[cfg(feature = "json-casting")]
impl From<FrcValue> for JSONValue {
    fn from(value: FrcValue) -> Self {
        match value {
            FrcValue::Void => Self::Null,
            FrcValue::Boolean(b) => Self::Bool(b),
            FrcValue::Int(i) => Self::Number({
                if i < 0 {
                    serde_json::Number::from(i)
                } else {
                    serde_json::Number::from(i as u64)
                }
            }),
            FrcValue::Float(f) => Self::Number(serde_json::Number::from_f64(f as f64).unwrap()),
            FrcValue::Double(f) => Self::Number(serde_json::Number::from_f64(f).unwrap()),
            FrcValue::String(s) => Self::String(s.into()),
            FrcValue::Raw(b) => Self::Array(
                b.iter()
                    .map(|v| Self::Number((*v as i64).into()))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::BooleanArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Bool(v))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::IntArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Number({
                        if v < 0 {
                            serde_json::Number::from(v)
                        } else {
                            serde_json::Number::from(v as u64)
                        }
                    }))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::FloatArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Number(serde_json::Number::from_f64(v as f64).unwrap()))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::DoubleArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::Number(serde_json::Number::from_f64(v).unwrap()))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::StringArray(a) => Self::Array(
                a.into_iter()
                    .map(|v| Self::String(v.into()))
                    .collect::<Vec<Self>>()
            ),
            FrcValue::Struct(_, b) => Self::Array(
                b.iter()
                    .map(|v| Self::Number((*v as u64).into()))
                    .collect::<Vec<Self>>()
            ),
        }
    }
}