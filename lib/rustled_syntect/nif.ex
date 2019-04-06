defmodule RustledSyntect.Nif do
  use Rustler, otp_app: :rustled_syntect, crate: "rustled_syntect"

  def new_highlighter(_lang), do: :erlang.nif_error(:nif_not_loaded)
  def highlight_line(_hl, _line), do: :erlang.nif_error(:nif_not_loaded)
  def finalize(_hl), do: :erlang.nif_error(:nif_not_loaded)
end
