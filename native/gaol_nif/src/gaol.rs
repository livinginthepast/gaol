use jail::RunningJail;
use rustler::types::elixir_struct;
use rustler::{Encoder, Env, Term};
use std::convert::TryFrom;

use crate::atoms;

#[derive(Clone, Debug)]
pub struct Jail {
    pub jid: u32,
}

impl From<RunningJail> for Jail {
    fn from(jail: RunningJail) -> Self {
        Jail {
            jid: u32::try_from(jail.jid).unwrap(),
        }
    }
}

impl Encoder for Jail {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let jail = elixir_struct::make_ex_struct(env, "Elixir.Gaol.Jail").unwrap();

        jail.map_put(atoms::jid().to_term(env), self.jid).unwrap()
    }
}
