defmodule Gaol do
  @moduledoc """
  Documentation for `Gaol`.
  """
  alias Gaol.Native

  @doc "List all running jails"
  @spec all() :: [Gaol.Jail.t()]
  def all, do: Native.all()

  @doc "Start a new jail with the given path and name"
  @spec create(Path.t(), binary()) :: {:ok, Gaol.Jail.t()} | {:error, atom()}
  def create(path, name), do: Native.create(path, name)

  @doc "Finds a jail by jid, or returns {:error, :not_found}"
  @spec get(Gaol.Jail.jid()) :: {:ok, Gaol.Jail.t()} | {:error, :not_found}
  def get(jid), do: Native.find_jail(jid)

  @doc "Stops a jail by jid, or returns {:error, :not_found}"
  @spec kill(Gaol.Jail.jid()) :: :ok | {:error, :not_found}
  def kill(jid), do: Native.kill(jid)
end
