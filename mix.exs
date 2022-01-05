defmodule RustledSyntect.MixProject do
  use Mix.Project

  def project do
    [
      app: :rustled_syntect,
      description:
        "Rustler binding for the Syntect syntax highlighter, with streaming and iolists",
      version: "0.1.2",
      elixir: "~> 1.7",
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package()
    ]
  end

  def application do
    []
  end

  defp deps do
    [
      {:rustler, git: "https://github.com/rusterlium/rustler.git", sparse: "rustler_mix"},
      {:ex_doc, ">= 0.0.0", only: :dev}
    ]
  end

  defp package do
    [
      files: ["native", "lib", "mix.exs", "README.md", "CODE_OF_CONDUCT.md", "LICENSE.txt"],
      maintainers: ["Greg V"],
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/myfreeweb/rustled_syntect"}
    ]
  end
end
