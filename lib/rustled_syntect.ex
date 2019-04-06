defmodule RustledSyntect do
  alias RustledSyntect.Nif

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
  @spec hilite_stream(Enumerable.t(), [{:lang, String.Chars.t}]) :: Enumerable.t()
  def hilite_stream(stream, lang: lang) do
    hl = Nif.new_highlighter(lang)
    stream
    |> Stream.map(fn line -> Nif.highlight_line(hl, line) end)
    |> Stream.intersperse("\n")
    |> Stream.concat(Stream.unfold(true, fn
      true -> {Nif.finalize(hl), false}
      false -> nil
    end))
  end
end
