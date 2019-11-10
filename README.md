# ptail :trident:

[![Crates.io](https://img.shields.io/crates/v/ptail.svg)](https://crates.io/crates/ptail)
[![Actions Status](https://github.com/orf/ptail/workflows/CI/badge.svg)](https://github.com/orf/ptail/actions)

![](./images/demo.gif)

`ptail` is a small, and likely useless, utility that truncates the output from processes. Unlike `tail -f` it will not 
show more than the specified number of lines in your terminal.

This could be useful if you are executing a command as part of a shell script and you do not wish to show the full and 
verbose output, but you do want to be able to see what is happening.

# Install :cd:

## Homebrew (MacOS + Linux)

`brew tap orf/brew`, then `brew install ptail`

## Binaries (Windows)

Download the latest release from [the github releases page](https://github.com/orf/ptail/releases). Extract it 
and move it to a directory on your `PATH`.

## Cargo

`cargo install ptail`
