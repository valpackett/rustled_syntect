defmodule RustledSyntect do
  alias RustledSyntect.Nif

  @doc ~S'''
  Creates a new syntax set with the default languages and all language files at the path specified.
  This can be used to load additional syntax definitions not included in the default Syntect release.

      iex> RustledSyntect.new_syntax_set([code:priv_dir(:rustled_syntect), "packages"] |> Path.join)
      #Reference<0.3040294775.638451714.65521>
  '''
  @spec new_syntax_set(String.Chars.t()) :: Reference
  def new_syntax_set(extra_syntaxes_path) when is_binary(extra_syntaxes_path) do
    Nif.new_syntax_set(extra_syntaxes_path)
  end

  @doc ~S'''
  Syntax highlight an enumerable/stream of lines, producing an iolist.

      iex> Enum.join(RustledSyntect.hilite_stream(["(0..69).each do |x|", "  puts x", "end"], lang: "Ruby"), "") <> "\n"
      """
      <span class="source ruby"><span class="punctuation definition group begin ruby">(</span><span class="constant numeric ruby">0</span><span class="keyword operator ruby">..</span><span class="constant numeric ruby">69</span><span class="punctuation definition group end ruby">)</span><span class="punctuation accessor ruby">.</span>each <span class="keyword control start-block ruby">do</span> <span class="meta block parameters ruby"><span class="punctuation definition parameters begin ruby">|</span></span><span class="meta block parameters ruby"><span class="variable parameter ruby">x</span><span class="meta block parameters ruby"><span class="punctuation definition parameters end ruby">|</span></span></span>
        <span class="support function builtin ruby">puts</span> x
      <span class="keyword control ruby">end</span></span>
      """

      iex> Enum.into(RustledSyntect.hilite_stream(["(0..69).each do |x|", "  puts x", "end"], lang: "Ruby"), [])
      [
        "<span class=\"source ruby\"><span class=\"punctuation definition group begin ruby\">(</span><span class=\"constant numeric ruby\">0</span><span class=\"keyword operator ruby\">..</span><span class=\"constant numeric ruby\">69</span><span class=\"punctuation definition group end ruby\">)</span><span class=\"punctuation accessor ruby\">.</span>each <span class=\"keyword control start-block ruby\">do</span> <span class=\"meta block parameters ruby\"><span class=\"punctuation definition parameters begin ruby\">|</span></span><span class=\"meta block parameters ruby\"><span class=\"variable parameter ruby\">x</span><span class=\"meta block parameters ruby\"><span class=\"punctuation definition parameters end ruby\">|</span></span></span>",
        "\n",
        "  <span class=\"support function builtin ruby\">puts</span> x",
        "\n",
        "<span class=\"keyword control ruby\">end</span>",
        ["</span>"]
      ]

  '''
  @spec hilite_stream(Enumerable.t(), [{:lang, String.Chars.t()}]) :: Enumerable.t()
  def hilite_stream(stream, lang: lang) do
    hilite_stream(stream, lang: lang, syntax_set: RustledSyntect.Nif.new_syntax_set())
  end

  @doc ~S'''
  Syntax hilight an enumerable/stream of lines with a provided syntax set, producing an iolist.

      # Assuming the priv/packages directory includes an Elixir.sublime-syntax syntax set:

      iex> ss = RustledSyntect.new_syntax_set([code:priv_dir(:rustled_syntect), "packages"] |> Path.join)
      #Reference<0.3040294775.638451714.65521>

      iex> RustledSyntect.hilite_stream(["defmodule Foo do", "end"], lang: "Elixir", syntax_set: ss) |> Enum.into([])
      [
        "<span class=\"source elixir\"><span class=\"meta module elixir\"><span class=\"keyword control module elixir\">defmodule</span> <span class=\"entity name class elixir\">Fo</span> <span class=\"keyword control module elixir\">do</span></span>",
        "\n",
        "<span class=\"keyword control elixir\">end</span>",
        ["</span>"]
      ]
  '''
  @spec hilite_stream(Enumerable.t(), [{:lang, String.Chars.t(), syntax_set: Reference}]) :: Enumerable.t()
  def hilite_stream(stream, lang: lang, syntax_set: ss) do
    hl = Nif.new_highlighter(ss, lang)

    stream
    |> Stream.map(fn line -> Nif.highlight_line(hl, line) end)
    |> Stream.intersperse("\n")
    |> Stream.concat(
      Stream.unfold(true, fn
        true -> {Nif.finalize(hl), false}
        false -> nil
      end)
    )
  end

  def supported_langs, do: Nif.langs()
end
