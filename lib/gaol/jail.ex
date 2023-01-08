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

  @typedoc "The primary identifier of a jail"
  @type jid() :: pos_integer()

  @typedoc "A representation of a running jail."
  @type t() :: %__MODULE__{
          hostname: binary(),
          jid: jid(),
          name: binary(),
          params: %{binary() => any()},
          path: binary()
        }
end
