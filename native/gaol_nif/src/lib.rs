mod atoms;
mod gaol;

use rustler::{Env, Term};

fn on_load(_env: Env, _info: Term) -> bool {
    match env_logger::try_init() {
        Ok(()) => log::debug!("Logger initialized succsessfully\r"),
        Err(_reason) => log::debug!("Logger already initialized. Ignoring.\r"),
    };
    true
}

rustler::init!(
    "Elixir.Gaol.Native",
    [gaol::all, gaol::find_jail],
    load = on_load
);
