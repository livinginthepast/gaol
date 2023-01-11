defmodule Gaol.Native do
  @moduledoc false
  use Rustler, otp_app: :gaol, crate: :gaol_nif

  @type t() :: reference()

  @spec add_ip(t(), binary()) :: {:ok, t(), Gaol.Jail.stopped()} | {:error, atom()}
  def add_ip(_ref, _ip), do: __not_loaded__()

  @spec all() :: [Gaol.Jail.t()]
  def all, do: __not_loaded__()

  @spec create(Path.t(), binary()) :: {:ok, t(), Gaol.Jail.stopped()}
  def create(_path, _name), do: __not_loaded__()

  @spec find_jail(Gaol.Jail.jid()) :: {:ok, Gaol.Jail.t()} | {:error, :not_found}
  def find_jail(_jid), do: __not_loaded__()

  @spec kill(Gaol.Jail.jid()) :: :ok | {:error, atom()}
  def kill(_jid), do: __not_loaded__()

  @spec set_hostname(t(), binary()) :: {:ok, t(), Gaol.Jail.stopped()} | {:error, atom()}
  def set_hostname(_ref, _hostname), do: __not_loaded__()

  @spec set_param(t(), binary(), binary()) :: {:ok, t(), Gaol.Jail.stopped()} | {:error, atom()}
  def set_param(_ref, _key, _value), do: __not_loaded__()

  @spec start(t()) :: {:ok, Gaol.Jail.t()} | {:error, atom()}
  def start(_ref), do: __not_loaded__()

  # # #

  defp __not_loaded__, do: :erlang.nif_error(:nif_not_loaded)
end
