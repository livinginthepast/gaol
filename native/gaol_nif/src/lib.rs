mod atoms;
mod gaol;

mod macros {
    macro_rules! unwrap_or_return {
        ( $e:expr, $f:expr ) => {
            match $e {
                Ok(x) => x,
                Err(_) => return Err($f),
            }
        };
    }

    macro_rules! decode_or_error {
        ( $e:expr ) => {
            match $e.decode() {
                Ok(x) => x,
                Err(_) => return Err(atoms::decoder_error()),
            }
        };
    }

    macro_rules! decode_or_error_tuple {
        ( $e:expr, $env:expr ) => {
            match $e.decode() {
                Ok(x) => x,
                Err(_) => return (atoms::error(), atoms::decoder_error()).encode($env),
            }
        };
    }

    pub(crate) use decode_or_error;
    pub(crate) use decode_or_error_tuple;
    pub(crate) use unwrap_or_return;
}

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
        gaol::add_ip,
        gaol::all,
        gaol::create,
        gaol::find_jail,
        gaol::kill,
        gaol::set_hostname,
        gaol::set_param,
        gaol::start,
    ],
    load = on_load
);
