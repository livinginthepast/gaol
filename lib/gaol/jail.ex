defmodule Gaol.Jail do
  @moduledoc """
  A representation of a running jail.
  """

  defstruct [
    :hostname,
    :jid,
    :name,
    :path
  ]

  @typedoc """
  A representation of a running jail.
  """
  @type t() :: %__MODULE__{
          hostname: binary(),
          jid: pos_integer(),
          name: binary(),
          path: binary()
        }
end
