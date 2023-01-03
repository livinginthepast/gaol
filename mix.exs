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
      {:rustler, "~> 0.26.0"}
    ]
end
