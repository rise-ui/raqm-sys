## libraqm Rust FFI Bindings

### Installing libraqm

In order to generate bindings, `raqm-sys` needs the libraqm to be intalled on a developer's system.

#### Building from sources
Clone the [`RAQM` repo](https://github.com/HOST-Oman/libraqm) and checkout the 0.5.0 version.
```bash
git clone git@github.com:HOST-Oman/libraqm.git
cd td
git checkout v0.5.0
```
Then proceed with the install instructions in the [libraqm repo README.md](https://github.com/HOST-Oman/libraqm/tree/v0.5.0).

#### Install from repository
Use your distribution package manager to download and install the `libraqm`
##### Arch Linux
```bash
#: pacman -S libraqm
```

### Generate Bindings

After installing `libraqm`, just add `raqm-sys` to your crate's dependencies

```toml
raqm-sys = "0.2.1"
```

And let the Cargo do it's magic!
```bash
cargo build
```
