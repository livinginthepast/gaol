defmodule Gaol.Native do
  @moduledoc false
  use Rustler, otp_app: :gaol, crate: :gaol_nif

  @spec all() :: [Gaol.Jail.t()]
  def all, do: __not_loaded__()

  @spec find_jail(Gaol.Jail.jid()) :: {:ok, Gaol.Jail.t()} | {:error, :not_found}
  def find_jail(_jid), do: __not_loaded__()

  # # #

  defp __not_loaded__, do: :erlang.nif_error(:nif_not_loaded)
end
