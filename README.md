# Gaol

FreeBSD jails.

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
