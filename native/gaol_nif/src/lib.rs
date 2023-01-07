mod atoms;
mod gaol;

use jail::RunningJail;
use rustler::{Encoder, Env, Term};

#[rustler::nif]
fn all(env: Env) -> Term {
    let running_jails = RunningJail::all();

    let jails: Vec<gaol::Jail> = running_jails
        .into_iter()
        .map(|running_jail| running_jail.into())
        .collect();

    jails.encode(env)
}

rustler::init!("Elixir.Gaol.Native", [all]);
