defmodule RustledSyntect.Nif do
  use Rustler, otp_app: :rustled_syntect, crate: "rustled_syntect"

  def new_syntax_set(_extra_syntaxes_path \\ nil), do: :erlang.nif_error(:nif_not_loaded)
  def new_highlighter(_syntax_set, _lang), do: :erlang.nif_error(:nif_not_loaded)
  def highlight_line(_hl, _line), do: :erlang.nif_error(:nif_not_loaded)
  def finalize(_hl), do: :erlang.nif_error(:nif_not_loaded)

  def langs(_extra_syntaxes_path \\ nil), do: :erlang.nif_error(:nif_not_loaded)
end
