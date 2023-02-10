defmodule Gaol.MixProject do
  use Mix.Project

  @description "FreeBSD jails"
  @scm_url "https://github.com/livinginthepast/gaol"
  @version "0.1.0"

  def application,
    do: [
      extra_applications: [:logger]
    ]

  def project,
    do: [
      app: :gaol,
      deps: deps(),
      description: @description,
      docs: docs(),
      elixir: "~> 1.14",
      homepage_url: @scm_url,
      package: package(),
      preferred_cli_env: [credo: :test, dialyzer: :test, docs: :dev],
      source_url: @scm_url,
      start_permanent: Mix.env() == :prod,
      version: @version
    ]

  defp deps,
    do: [
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.2", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.29", only: [:dev], runtime: false},
      {:mix_audit, "~> 2.1", only: :dev, runtime: false},
      {:rustler, "~> 0.27.0"}
    ]

  defp docs,
    do: [
      main: "readme",
      source_ref: "v#{@version}"
    ]

  defp package,
    do: [
      licenses: ["MIT"],
      maintainers: ["Eric Saxby"],
      links: %{"GitHub" => @scm_url},
      files: ~w(
        LICENSE.md
        README.md
        lib
        mix.exs
        native/gaol_nif/src
        native/gaol_nif/Cargo.toml
        native/gaol_nif/Cargo.lock
      )
    ]
end
