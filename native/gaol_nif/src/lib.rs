mod atoms;
mod gaol;

use rustler::{Env, Term};

fn on_load(env: Env, _info: Term) -> bool {
    match env_logger::try_init() {
        Ok(()) => log::debug!("Logger initialized succsessfully\r"),
        Err(_reason) => log::debug!("Logger already initialized. Ignoring.\r"),
    };
    gaol::load(env);

    true
}

rustler::init!(
    "Elixir.Gaol.Native",
    [
        gaol::all,
        gaol::create,
        gaol::find_jail,
        gaol::kill,
        gaol::start,
    ],
    load = on_load
);
