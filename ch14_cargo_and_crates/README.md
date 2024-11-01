## 1 Profiles
Cargo.toml

```toml
[profile.dev]
opt-level = 0
```

```bash
cargo build
```

---

```toml
[profile.release]
opt-level = 3
```

```bash
cargo build --release
```

---

```toml
[profile.my]
inherits = "release"
opt-level = 1
debug = true
```

```bash
cargo build --profile my
```


## 2 Publication
-- snip

- token https://crates.io/
- 
```bash
cargo login abcdefghijklmnopqrstuvwxyz012345
cargo publish
```

deprecating versions
```bash
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo
```


## 3 Workspaces
adder/Cargo.toml
```toml
# snip
[dependencies]
add_one = { path = "../add_one" }
```

```bash
cargo run -p adder
```

"If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock."


```bash
cargo test
cargo test -p adder
```

## 4 Binary to OS
to $PATH

```bash
cargo install ripgrep
rg --help
```

## 5 Custom Command
"If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running cargo something"

```bash
cargo --list
```
