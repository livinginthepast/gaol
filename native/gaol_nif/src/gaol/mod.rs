pub mod error;

use jail::param::Value;
use jail::RunningJail;
use libc;
use rustler::types::elixir_struct;
use rustler::{Atom, Encoder, Env, Term};
use std::collections::hash_map::HashMap;
use std::convert::TryFrom;

use crate::atoms;

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

#[derive(Clone, Debug)]
pub struct Jail {
    pub hostname: String,
    pub jid: u32,
    pub name: String,
    pub params: HashMap<String, ParamValue>,
    pub path: String,
}

impl From<RunningJail> for Jail {
    fn from(jail: RunningJail) -> Self {
        Jail {
            hostname: jail.hostname().unwrap(),
            jid: u32::try_from(jail.jid).unwrap(),
            name: jail.name().unwrap(),
            params: jail
                .params()
                .unwrap()
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            path: jail.path().unwrap().into_os_string().into_string().unwrap(),
        }
    }
}

impl Encoder for Jail {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let jail = elixir_struct::make_ex_struct(env, "Elixir.Gaol.Jail").unwrap();

        jail.map_put(atoms::jid().to_term(env), &self.jid)
            .unwrap()
            .map_put(atoms::hostname().to_term(env), &self.hostname)
            .unwrap()
            .map_put(atoms::name().to_term(env), &self.name)
            .unwrap()
            .map_put(atoms::params().to_term(env), &self.params)
            .unwrap()
            .map_put(atoms::path().to_term(env), &self.path)
            .unwrap()
    }
}

#[rustler::nif]
fn all(env: Env) -> Term {
    let running_jails = RunningJail::all();

    let jails: Vec<Jail> = running_jails
        .into_iter()
        .map(|running_jail| running_jail.into())
        .collect();

    jails.encode(env)
}

#[rustler::nif]
fn find_jail<'a>(env: Env<'a>, jid_term: Term<'a>) -> Result<Term<'a>, Atom> {
    let jid: i32 = jid_term.decode().unwrap();

    match RunningJail::from_jid(jid) {
        Some(jail) => Ok(<RunningJail as Into<Jail>>::into(jail).encode(env)),
        None => Err(atoms::not_found()),
    }
}

#[rustler::nif]
fn kill<'a>(env: Env<'a>, jid_term: Term<'a>) -> Term<'a> {
    let jid: i32 = jid_term.decode().unwrap();

    let jail = match RunningJail::from_jid(jid) {
        Some(jail) => jail,
        None => return (atoms::error(), atoms::not_found()).encode(env),
    };

    match jail.kill() {
        Ok(()) => atoms::ok().encode(env),
        Err(jail_err) => {
            log::debug!("Unable to kill jail: {:?}\r", jail_err);
            (atoms::error(), error::to_atom(jail_err)).encode(env)
        }
    }
}
