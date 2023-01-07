use jail::RunningJail;
use rustler::types::elixir_struct;
use rustler::{Encoder, Env, Term};
use std::convert::TryFrom;

use crate::atoms;

#[derive(Clone, Debug)]
pub struct Jail {
    pub hostname: String,
    pub jid: u32,
    pub name: String,
    pub path: String,
}

impl From<RunningJail> for Jail {
    fn from(jail: RunningJail) -> Self {
        Jail {
            hostname: jail.hostname().unwrap(),
            jid: u32::try_from(jail.jid).unwrap(),
            name: jail.hostname().unwrap(),
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
            .map_put(atoms::path().to_term(env), &self.path)
            .unwrap()
    }
}
