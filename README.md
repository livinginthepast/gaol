# Gaol

FreeBSD jails.

## Dependencies

```shell
pkg install rust
```

Rust must be available in the environment used to compile Gaol. When Gaol is included
in an Elixir release that is packaged for installation in a different system, Rust is
not necessary in that system.

Note that many jail interactions require root.

## Installation

Gaol should only be included in projects built for FreeBSD. See the docs
[Mix targets](https://hexdocs.pm/mix/1.14.2/Mix.html#module-targets) for
more information.

```elixir
def deps do
  [
    {:gaol, "~> 0.1.0", target: :freebsd}
  ]
end
```

## Development

The log level of the NIF may be configured by setting `RUST_LOG` in the environment.

```shell
RUST_LOG=debug sudo iex -S mix
```
