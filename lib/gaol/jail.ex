defmodule Gaol.Jail do
  @moduledoc """
  A representation of a running jail.
  """

  defstruct [
    :jid
  ]

  @typedoc """
  A representation of a running jail.
  """
  @type t() :: %__MODULE__{
          jid: pos_integer()
        }
end
