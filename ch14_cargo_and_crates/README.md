Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

[profile.my]
inherits = "release"
opt-level = 1
debug = true
```

```bash
cargo build --profile my
```