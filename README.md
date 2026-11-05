# fantasy

fantasy reads TDLib's [`td_api.tl`](https://github.com/tdlib/td/blob/master/td/generate/scheme/td_api.tl) and generates Rust for two crates:

1. [rtdlib](https://github.com/fewensa/rtdlib) — TDLib types
2. [telegram-client](https://github.com/fewensa/telegram-client) — a client built on rtdlib

The schema checked in at `schema/v1.8.69/td_api.tl` is TDLib **1.8.69**, commit `4c6377aa7386c39217878a9a56d581922ad37ca4`. That is the newest TDLib release (the version in `CMakeLists.txt` on `master`). GitHub tags stop at `v1.8.0`; later releases are version bumps on `master`.

## Regenerate rtdlib

From this repository:

```bash
RTD_OUT=target/rtd-out cargo run --release -p fantasy
```

That parses `schema/v1.8.69/td_api.tl`, applies `schema/td_type_fill.toml`, renders `template/`, and rustfmt's the output (`edition = 2021`, `tab_spaces = 2`). Generated types land in `target/rtd-out/src/types`. `src/lib.rs`, `src/errors.rs`, and `README.md` are copied from `template/rtdlib` only when those files are not already in the output tree.

Copy the types onto an rtdlib checkout:

```bash
rm -rf ../rtdlib/src/types
cp -a target/rtd-out/src/types ../rtdlib/src/types
```

`TL_FILE` overrides the schema path. `RTD_OUT` overrides the output directory. Pointing `RTD_OUT` at an existing rtdlib tree replaces only `src/types` and leaves `Cargo.toml`, `src/lib.rs`, and `src/errors.rs` in place.

```bash
RTD_OUT=../rtdlib cargo run --release -p fantasy
```

Telegram-client sources are generated only when `TG_OUT` is set:

```bash
TG_OUT=target/telegram-client-out RTD_OUT=target/rtd-out cargo run --release -p fantasy
```

## What the generator adjusts

- Scalar `int64` fields deserialize from a JSON string or a number (`serde-aux`).
- A TL field named `extra` becomes Rust `extra_` and still serializes as `extra`, so it does not clash with `@extra`.
- Direct cycles listed in `schema/td_type_fill.toml` (`RichText`, `PageBlock`, `InternalLinkType` / `TargetChat`) use `Box`.

## Build

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
