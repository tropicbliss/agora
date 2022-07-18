# Agora

A port of [Texture Rotation Reverser Java](https://github.com/19MisterX98/TextureRotations) by 19MisterX98 to Rust.

I'm not going to re-explain everything on their readme, go follow the link and read it yourself.

## Compiling from source

If you are on another platform, compile the binary yourself to try it out:

```sh
git clone https://github.com/tropicbliss/agora
cd agora
cargo build --release
```

Compiling from source requires the latest stable version of Rust. Older Rust versions may be able to compile `agora`, but they are not guaranteed to keep working.

The binary will be located in `target/release`.

## Usage

```toml
# config.toml

sodium = false

# optional fields
x_bounds = [-10000, 10000]
y_bounds = [10, 60]
z_bounds = [-10000, 10000]

[[rotation_info]]
x = 1
y = 0
z = 0
rotation = 1
is_side = true

[[rotation_info]]
x = 1
y = 1
z = 0
rotation = 1
is_side = true
```

For other enquiries use the `--help` command line flag.
