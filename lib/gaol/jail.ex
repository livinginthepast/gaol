defmodule Gaol.Jail do
  @moduledoc """
  A representation of a running jail.
  """

  defstruct [
    :hostname,
    :jid,
    :name,
    :params,
    :path
  ]

  @typedoc """
  A representation of a running jail.
  """
  @type t() :: %__MODULE__{
          hostname: binary(),
          jid: pos_integer(),
          name: binary(),
          params: %{binary() => any()},
          path: binary()
        }
end
