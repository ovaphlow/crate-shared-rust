# crate-shared-rust

## Scripts

Test

```shell
cargo test -- --nocapture
```

Build

```shell
cargo build --release
```

Build for Windows

```shell
rustup target add x86_64-pc-windows-gnu
```

```shell
cargo build --target x86_64-pc-windows-gnu --release
```

## Build Script

The build script `build.rs` will rename the generated library file to include the version number.