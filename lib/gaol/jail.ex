defmodule Gaol.Jail do
  @moduledoc """
  A representation of a running jail.
  """

  defstruct [
    :hostname,
    :jid,
    :name,
    :params,
    :path,
    :native
  ]

  @typedoc "The primary identifier of a jail"
  @type jid() :: pos_integer()

  @typedoc "A representation of a running jail."
  @type t() :: %__MODULE__{
          hostname: binary(),
          jid: jid(),
          name: binary(),
          params: %{binary() => any()},
          path: binary(),
          native: nil
        }

  @type stopped() :: %__MODULE__{
          hostname: binary(),
          jid: nil,
          name: binary(),
          params: %{binary() => any()},
          path: binary(),
          native: reference()
        }
end
