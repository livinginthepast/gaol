use jail::RunningJail;
use rustler::types::elixir_struct;
use rustler::{Encoder, Env, Term};
use std::convert::TryFrom;

mod atoms;

#[derive(Clone, Debug)]
struct Jail {
    pub jid: u32,
}

impl From<RunningJail> for Jail {
    fn from(jail: RunningJail) -> Self {
        Jail { jid: u32::try_from(jail.jid).unwrap() }
    }
}

impl Encoder for Jail {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let jail = elixir_struct::make_ex_struct(env, "Elixir.Gaol.Jail").unwrap();

        jail.map_put(atoms::jid().to_term(env), self.jid).unwrap()
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

rustler::init!("Elixir.Gaol.Native", [all]);
