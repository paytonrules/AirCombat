# AirCombat

This is a port of this [Godot Tutorial](https://devga.me/tutorials/godot2d/getting-started-with-godot/) to Rust. I started a write-up of the entire process on [my blog](https://paytonrules.com/post/games-in-rust-with-godot-part-one/), unfortuantely godot-rust created a large number of breaking changes while I was writing part 2, so that has been scrapped in favor of a new writeup that....doesn't exist yet.

## Requirements

* Godot 3.2 [download](https://godotengine.org/)
* Rust [download](https://www.rust-lang.org/) - tested with Rust 1.41.0
* Bindgen [directions](https://rust-lang.github.io/rust-bindgen/requirements.html) - As a Mac and Homebrew user I was able to just say `brew install llvm`. I have not tried this on Windows/Linux and would love to hear people's results.

## Building

To build and run the Godot app first build the Rust library by running `cargo build` from the `src/air_combat` directory. Then open the project in Godot and hit play. Please submit any issues you have!


