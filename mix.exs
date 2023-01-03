defmodule Gaol.MixProject do
  use Mix.Project

  def application,
    do: [
      extra_applications: [:logger]
    ]

  def project,
    do: [
      app: :gaol,
      deps: deps(),
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      version: "0.1.0"
    ]

  defp deps,
    do: [
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.2", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.29", only: [:dev], runtime: false},
      {:mix_audit, "~> 2.1", only: :dev, runtime: false},
      {:rustler, "~> 0.26.0"}
    ]
end
