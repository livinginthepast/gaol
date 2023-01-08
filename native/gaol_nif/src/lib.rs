mod atoms;
mod gaol;

use jail::RunningJail;
use rustler::{Atom, Encoder, Env, Term};

#[rustler::nif]
fn all(env: Env) -> Term {
    let running_jails = RunningJail::all();

    let jails: Vec<gaol::Jail> = running_jails
        .into_iter()
        .map(|running_jail| running_jail.into())
        .collect();

    jails.encode(env)
}

#[rustler::nif]
fn find_jail<'a>(env: Env<'a>, jid_term: Term<'a>) -> Result<Term<'a>, Atom> {
    let jid: i32 = jid_term.decode().unwrap();

    match RunningJail::from_jid(jid) {
        Some(jail) => Ok(<RunningJail as Into<gaol::Jail>>::into(jail).encode(env)),
        None => Err(atoms::not_found()),
    }
}

rustler::init!("Elixir.Gaol.Native", [all, find_jail]);
