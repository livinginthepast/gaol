defmodule Gaol do
  @moduledoc """
  Documentation for `Gaol`.
  """
  alias Gaol.Native

  @type param_key() :: binary()
  @type param_value() :: binary() | integer() | [binary()]

  @doc "Adds an IP address to a jail that has not yet been started"
  @spec add_ip(Gaol.Jail.stopped(), binary()) :: Gaol.Jail.stopped() | {:error, atom()}
  def add_ip(%Gaol.Jail{native: ref}, ip) when is_reference(ref),
    do: Native.add_ip(ref, ip) |> into_stopped()

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
  @spec set_hostname(Gaol.Jail.stopped(), binary()) :: Gaol.Jail.stopped() | {:error, atom()}
  def set_hostname(%Gaol.Jail{native: ref}, hostname) when is_reference(ref),
    do: Native.set_hostname(ref, hostname) |> into_stopped()

  @doc "Sets a parameter on a jail that has not yet been started"
  @spec set_param(Gaol.Jail.stopped(), param_key(), param_value()) :: Gaol.Jail.stopped() | {:error, atom()}
  def set_param(%Gaol.Jail{native: ref}, key, value) when is_reference(ref),
    do: Native.set_param(ref, key, value) |> into_stopped()

  @doc "Starts a jail after all configuration has been set"
  @spec start(Gaol.Jail.stopped()) :: {:ok, Gaol.Jail.t()} | {:error, atom()}
  def start(%Gaol.Jail{native: ref}) when is_reference(ref),
    do: Native.start(ref)

  # # #

  defp into_stopped({:ok, ref, jail}), do: %{jail | native: ref}
  defp into_stopped({:error, error}), do: {:error, error}
end
