use jail::param::Value;
use libc;
use rustler::{Encoder, Env, Term};

#[derive(Clone, Debug)]
pub enum ParamValue {
    Int(libc::c_int),
    String(String),
    S64(i64),
    Uint(libc::c_uint),
    Long(libc::c_long),
    Ulong(libc::c_ulong),
    U64(u64),
    U8(u8),
    U16(u16),
    S8(i8),
    S16(i16),
    S32(i32),
    U32(u32),
    Ipv4Addrs(Vec<std::net::Ipv4Addr>),
    Ipv6Addrs(Vec<std::net::Ipv6Addr>),
}

impl From<Value> for ParamValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(value) => ParamValue::Int(value),
            Value::String(value) => ParamValue::String(value),
            Value::S64(value) => ParamValue::S64(value),
            Value::Uint(value) => ParamValue::Uint(value),
            Value::Long(value) => ParamValue::Long(value),
            Value::Ulong(value) => ParamValue::Ulong(value),
            Value::U64(value) => ParamValue::U64(value),
            Value::U8(value) => ParamValue::U8(value),
            Value::U16(value) => ParamValue::U16(value),
            Value::S8(value) => ParamValue::S8(value),
            Value::S16(value) => ParamValue::S16(value),
            Value::S32(value) => ParamValue::S32(value),
            Value::U32(value) => ParamValue::U32(value),
            Value::Ipv4Addrs(values) => ParamValue::Ipv4Addrs(values),
            Value::Ipv6Addrs(values) => ParamValue::Ipv6Addrs(values),
        }
    }
}

impl Encoder for ParamValue {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ParamValue::Int(value) => value.encode(env),
            ParamValue::String(value) => value.encode(env),
            ParamValue::S64(value) => value.encode(env),
            ParamValue::Uint(value) => value.encode(env),
            ParamValue::Long(value) => value.encode(env),
            ParamValue::Ulong(value) => value.encode(env),
            ParamValue::U64(value) => value.encode(env),
            ParamValue::U8(value) => value.encode(env),
            ParamValue::U16(value) => value.encode(env),
            ParamValue::S8(value) => value.encode(env),
            ParamValue::S16(value) => value.encode(env),
            ParamValue::S32(value) => value.encode(env),
            ParamValue::U32(value) => value.encode(env),
            ParamValue::Ipv4Addrs(values) => values
                .iter()
                .map(|addr| addr.to_string())
                .collect::<Vec<String>>()
                .encode(env),
            ParamValue::Ipv6Addrs(values) => values
                .iter()
                .map(|addr| addr.to_string())
                .collect::<Vec<String>>()
                .encode(env),
        }
    }
}
