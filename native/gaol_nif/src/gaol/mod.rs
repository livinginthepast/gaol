pub mod error;
mod param_value;

use jail::{RunningJail, StoppedJail};
use rustler::types::elixir_struct;
use rustler::{Atom, Encoder, Env, ResourceArc, Term};
use std::collections::hash_map::HashMap;
use std::convert::TryFrom;

use crate::atoms;

pub struct JailResource {
    jail: StoppedJail,
}

#[derive(Clone, Debug)]
pub struct Jail {
    pub hostname: String,
    pub jid: Option<u32>,
    pub name: String,
    pub params: HashMap<String, param_value::ParamValue>,
    pub path: std::path::PathBuf,
}

impl From<RunningJail> for Jail {
    fn from(jail: RunningJail) -> Self {
        Jail {
            hostname: jail.hostname().unwrap(),
            jid: u32::try_from(jail.jid).ok(),
            name: jail.name().unwrap(),
            params: jail
                .params()
                .unwrap()
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            path: jail.path().unwrap(),
        }
    }
}

impl From<StoppedJail> for Jail {
    fn from(jail: StoppedJail) -> Self {
        Jail {
            hostname: jail.hostname.unwrap_or("".to_string()),
            jid: None,
            name: jail.name.unwrap_or("".to_string()),
            params: jail
                .params
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            path: jail.path.unwrap_or("/".into()),
        }
    }
}

impl Encoder for Jail {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let jail = elixir_struct::make_ex_struct(env, "Elixir.Gaol.Jail").unwrap();
        let jid = match &self.jid {
            Some(jid) => jid.encode(env),
            None => (rustler::types::atom::nil()).encode(env),
        };

        jail.map_put(atoms::jid().to_term(env), jid)
            .unwrap()
            .map_put(atoms::hostname().to_term(env), &self.hostname)
            .unwrap()
            .map_put(atoms::name().to_term(env), &self.name)
            .unwrap()
            .map_put(atoms::params().to_term(env), &self.params)
            .unwrap()
            .map_put(
                atoms::path().to_term(env),
                &self.path.clone().into_os_string().into_string().unwrap(),
            )
            .unwrap()
            .map_put(
                atoms::native().to_term(env),
                rustler::types::atom::nil().to_term(env),
            )
            .unwrap()
    }
}

pub fn load(env: Env) -> bool {
    rustler::resource!(JailResource, env);
    true
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
fn create<'a>(env: Env<'a>, path_term: Term<'a>, name_term: Term<'a>) -> Term<'a> {
    let path: String = path_term.decode().unwrap();
    let name: String = name_term.decode().unwrap();

    let stopped = StoppedJail::new(path).name(name);
    let jail = <StoppedJail as Into<Jail>>::into(stopped.clone());
    let resource = ResourceArc::new(JailResource { jail: stopped });

    (atoms::ok(), resource, jail.encode(env)).encode(env)
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

#[rustler::nif]
fn start<'a>(env: Env<'a>, resource: ResourceArc<JailResource>) -> Result<Term<'a>, Atom> {
    match resource.jail.clone().start() {
        Ok(jail) => Ok(<RunningJail as Into<Jail>>::into(jail).encode(env)),
        Err(jail_err) => {
            log::debug!("Error creating jail: {:?}\r", jail_err);
            Err(error::to_atom(jail_err))
        }
    }
}
