defmodule Gaol do
  @moduledoc """
  Documentation for `Gaol`.
  """
  alias Gaol.Native

  @doc "List all running jails"
  @spec all() :: [Gaol.Jail.t()]
  def all, do: Native.all()

  @doc "Start a new jail with the given path and name"
  @spec create(Path.t(), binary()) :: Gaol.Jail.stopped()
  def create(path, name),
    do: Native.create(path, name) |> into_stopped()

  @doc "Finds a jail by jid, or returns {:error, :not_found}"
  @spec get(Gaol.Jail.jid()) :: {:ok, Gaol.Jail.t()} | {:error, :not_found}
  def get(jid), do: Native.find_jail(jid)

  @doc "Stops a jail by jid, or returns {:error, :not_found}"
  @spec kill(Gaol.Jail.jid()) :: :ok | {:error, :not_found}
  def kill(jid), do: Native.kill(jid)

  @doc "Sets the hostname on a jail that has not yet been started"
  @spec set_hostname(Gaol.Jail.stopped(), binary()) :: {:ok, Gaol.Jail.stopped()}
  def set_hostname(%Gaol.Jail{native: ref}, hostname) when is_reference(ref),
    do: Native.set_hostname(ref, hostname) |> into_stopped()

  @doc "Starts a jail after all configuration has been set"
  @spec start(Gaol.Jail.stopped()) :: {:ok, Gaol.Jail.t()} | {:error, atom()}
  def start(%Gaol.Jail{native: ref}) when is_reference(ref),
    do: Native.start(ref)

  # # #

  defp into_stopped({:ok, ref, jail}), do: %{jail | native: ref}
end
