use jail::param::{Type, Value};
use libc;
use rustler::{Atom, Encoder, Env, Term};
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::atoms;
use crate::gaol::error;
use crate::macros;

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

impl ParamValue {
    pub fn decode_value<'a>(_env: Env<'a>, key: &String, value: Term<'a>) -> Result<Value, Atom> {
        let param_type = match Type::of_param(&key) {
            Ok(param_type) => param_type,
            Err(jail_err) => return Err(error::to_atom(jail_err)),
        };

        let value = match param_type {
            Type::Int => Value::Int(macros::decode_or_error!(value)),
            Type::Long => Value::Long(macros::decode_or_error!(value)),
            Type::S16 => Value::S16(macros::decode_or_error!(value)),
            Type::S32 => Value::S32(macros::decode_or_error!(value)),
            Type::S64 => Value::S64(macros::decode_or_error!(value)),
            Type::S8 => Value::S8(macros::decode_or_error!(value)),
            Type::String => Value::String(macros::decode_or_error!(value)),
            Type::U16 => Value::U16(macros::decode_or_error!(value)),
            Type::U32 => Value::U32(macros::decode_or_error!(value)),
            Type::U64 => Value::U64(macros::decode_or_error!(value)),
            Type::U8 => Value::U8(macros::decode_or_error!(value)),
            Type::Uint => Value::Uint(macros::decode_or_error!(value)),
            Type::Ulong => Value::Ulong(macros::decode_or_error!(value)),

            Type::Ipv4Addrs => Value::Ipv4Addrs(macros::unwrap_or_return!(
                decode_ipv4(value),
                atoms::decoder_error()
            )),
            Type::Ipv6Addrs => Value::Ipv6Addrs(macros::unwrap_or_return!(
                decode_ipv6(value),
                atoms::decoder_error()
            )),
        };

        Ok(value)
    }
}

impl From<Value> for ParamValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(value) => ParamValue::Int(value),
            Value::Long(value) => ParamValue::Long(value),
            Value::S16(value) => ParamValue::S16(value),
            Value::S32(value) => ParamValue::S32(value),
            Value::S64(value) => ParamValue::S64(value),
            Value::S8(value) => ParamValue::S8(value),
            Value::String(value) => ParamValue::String(value),
            Value::U16(value) => ParamValue::U16(value),
            Value::U32(value) => ParamValue::U32(value),
            Value::U64(value) => ParamValue::U64(value),
            Value::U8(value) => ParamValue::U8(value),
            Value::Uint(value) => ParamValue::Uint(value),
            Value::Ulong(value) => ParamValue::Ulong(value),

            Value::Ipv4Addrs(values) => ParamValue::Ipv4Addrs(values),
            Value::Ipv6Addrs(values) => ParamValue::Ipv6Addrs(values),
        }
    }
}

impl Encoder for ParamValue {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ParamValue::Int(value) => value.encode(env),
            ParamValue::Long(value) => value.encode(env),
            ParamValue::S16(value) => value.encode(env),
            ParamValue::S32(value) => value.encode(env),
            ParamValue::S64(value) => value.encode(env),
            ParamValue::S8(value) => value.encode(env),
            ParamValue::String(value) => value.encode(env),
            ParamValue::U16(value) => value.encode(env),
            ParamValue::U32(value) => value.encode(env),
            ParamValue::U64(value) => value.encode(env),
            ParamValue::U8(value) => value.encode(env),
            ParamValue::Uint(value) => value.encode(env),
            ParamValue::Ulong(value) => value.encode(env),

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

fn decode_ipv4<'a>(value: Term<'a>) -> Result<Vec<Ipv4Addr>, ()> {
    let strings: Vec<String> = match value.decode() {
        Ok(strings) => strings,
        Err(_) => return Err(()),
    };

    strings
        .iter()
        .map(|addr| match addr.parse::<Ipv4Addr>() {
            Ok(addr) => Ok(addr),
            Err(_) => Err(()),
        })
        .collect::<Result<Vec<Ipv4Addr>, ()>>()
}

fn decode_ipv6<'a>(value: Term<'a>) -> Result<Vec<Ipv6Addr>, ()> {
    let strings: Vec<String> = match value.decode() {
        Ok(strings) => strings,
        Err(_) => return Err(()),
    };

    strings
        .iter()
        .map(|addr| match addr.parse::<Ipv6Addr>() {
            Ok(addr) => Ok(addr),
            Err(_) => Err(()),
        })
        .collect::<Result<Vec<Ipv6Addr>, ()>>()
}
