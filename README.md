Tools for converting rkengine .rk (model) and .anim (animation) files.
Also includes a tool for extracting .ark files.

This fork tries to load all the images from the game as png files instead of pvr files. I made this change due to how the game as recently been including textures in a format that this program doesn't support.

# Building

Requires [Rust](https://rustup.rs/) and a C++ compiler (for use with
[`cc`](https://lib.rs/crates/cc)).

First, initialize the submodules:

```sh
git submodule update --init
```

Then, to build the tools:

```sh
cargo build --release
```

# Usage

## .ark files
This program does include a tool to extract ark files, however it doesn't work with the newer v4 files. You can use [Luna Kit](https://github.com/ego-lay-atman-bay/luna-kit/) to extract the files in the latest version (v4 ark files).

If you have v3 ark files and want to use this program, here's how to use the tool in the program (**Will not work with files from the latest version**).

To extract a .ark file:

```shell
cargo run --release --bin unark -- path/to/file.ark
```

This will extract the contents of the .ark into the current directory.
You should probably run this in a new, empty directory.

## Converting models
Since this fork searches to png files instead of pvr files, you have to use [Luna Kit](https://github.com/ego-lay-atman-bay/luna-kit/) to convert all the pvr files, though the `dump` command does this automatically.

```shell
luna-kit pvr "folder/**/*.pvr" -o "{dir}/{name}.{format}" -f png -n
```

To convert a model to glTF:

```shell
# Without animation:
cargo run --release --bin model-to-gltf -- path/to/model.rk
# With animation:
cargo run --release --bin model-to-gltf -- path/to/model.rk path/to/anim.csv
cargo run --release --bin model-to-gltf -- path/to/model.rk path/to/anim.anim
```

For a model named `XXX_YYY_lodN.rk`, there will usually be accompanying
animation files `XXX.anim`, `XXX.xml`, and `XXX.csv` that define animations for
that category of models.  Pass the `.csv` file if there is one - this file
divides the frames of the `.anim` file into separate, named animations.
Otherwise, pass the `.anim` file to get all frames as a single giant animation.
For models that aren't animated, don't pass an animation file at all.

The output will be saved as `out.glb` in glTF binary format.
