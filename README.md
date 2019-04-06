[![hex.pm version](https://img.shields.io/hexpm/v/rustled_syntect.svg?style=flat)](https://hex.pm/packages/rustled_syntect)
[![hex.pm downloads](https://img.shields.io/hexpm/dt/rustled_syntect.svg?style=flat)](https://hex.pm/packages/rustled_syntect)
[![API Docs](https://img.shields.io/badge/api-docs-yellow.svg?style=flat)](https://hexdocs.pm/rustled_syntect/)
[![MIT License](https://img.shields.io/badge/mit-license-green.svg?style=flat)](https://mit-license.org)

# rustled_syntect

An [Elixir] binding for [syntect], a syntax highlighting library written in [Rust].
Powered by [Rustler].
Supports line by line streaming and outputs [I/O lists for that awesome `writev` call](https://www.evanmiller.org/elixir-ram-and-the-template-of-doom.html)!

[Elixir]: https://elixir-lang.org
[Rust]: https://www.rust-lang.org
[Rustler]: https://github.com/rusterlium/rustler
[syntect]: https://github.com/trishume/syntect

## Installation

Add rustled_syntect to your project's dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:rustled_syntect, "~> 0.1.0"}
  ]
end
```

And fetch your project's dependencies:

```shell
$ mix deps.get
```

## Usage

```elixir
iex> RustledSyntect.hilite_stream(["(0..69).each do |x|", "  puts x", "end"], lang: "Ruby") |> Enum.into([]) |> IO.puts
```

```html
<span class="source ruby"><span class="punctuation definition group begin ruby">(</span><span class="constant numeric ruby">0</span><span class="keyword operator ruby">..</span><span class="constant numeric ruby">69</span><span class="punctuation definition group end ruby">)</span><span class="punctuation accessor ruby">.</span>each <span class="keyword control start-block ruby">do</span> <span class="meta block parameters ruby"><span class="punctuation definition parameters begin ruby">|</span></span><span class="meta block parameters ruby"><span class="variable parameter ruby">x</span><span class="meta block parameters ruby"><span class="punctuation definition parameters end ruby">|</span></span></span>
  <span class="support function builtin ruby">puts</span> x
<span class="keyword control ruby">end</span></span>
```

```elixir
iex> RustledSyntect.supported_langs
[
  %RustledSyntect.Syntax{
    file_extensions: ["txt"],
    first_line_match: nil,
    name: "Plain Text"
  },
  %RustledSyntect.Syntax{
    file_extensions: ["asa"],
    first_line_match: nil,
    name: "ASP"
  },
  %RustledSyntect.Syntax{
    file_extensions: ["asp"],
    first_line_match: nil,
    name: "HTML (ASP)"
  },
  %RustledSyntect.Syntax{
    file_extensions: ["as"],
    first_line_match: nil,
    name: "ActionScript"
  },
  %RustledSyntect.Syntax{
    file_extensions: ["applescript", "script editor"],
    first_line_match: "^#!.*(osascript)",
    name: "AppleScript"
  },
  …
] 
```

## Contributing

Please feel free to submit pull requests!

By participating in this project you agree to follow the [Contributor Code of Conduct](https://contributor-covenant.org/version/1/4/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/rustled_syntect/graphs/contributors).

## License

Like Syntect itself, this binding is available under the MIT License.
