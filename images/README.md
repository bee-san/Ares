# Steps to make the gifs

Install asciinema and svg-term-cli.

Record with asciinema:

asciinema rec demo.cast

This records the session in the asciicast v2 plaintext file format (newline-delimited JSON with an initial header object followed by a timestamped event stream of stdin and stdout).

Convert the .cast file to .svg with svg-term-cli:

svg-term --in demo.cast --out demo.svg --window --width 80 --height 22 --no-optimize

You probably want to play around with width and height
window adds a fake OS window around the terminal session
I found that no-optimize fixed some weird font rendering issues on my macOS – not sure why
