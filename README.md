# XMrs File format library

A no_std library to edit Sound Tracker data with pleasure.

Because "Representation is the Essence of Programming".

Suppported files:

- IT **Impulse Tracker**
- MOD **Amiga Modules**
- S3M **Scream Tracker III**
- SID **Rob Hubbard C64 files** (WIP).
- XM **FastTracker II**

To edit data, use `Module` struct.

You can serialize `Module` using serde (see `std` feature).

## HOWTO load historical tracker files

Test with `cargo run --no-default-features --features=demo --example xmrs -- --help`, then read 50 lines `examples/xmrs` example.

## About no_std

micromath is used by default in no_std. If you prefer libm, use `cargo build --no-default-features --features=libm --release`.

I leave the `import` feature enabled by default to make the documentation easier to read. To not enable it, use only micromath in your `Cargo.toml`:

```
xmrs = { version = "...", default-features = false, features=["micromath"] }
```

## About std

if you want to use std feature use `cargo build --no-default-features --features=std --release`

