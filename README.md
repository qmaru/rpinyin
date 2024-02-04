# rpinyin

rust wasm pinyin

## Install

```shell
cargo install wasm-pack
```

## Build

```shell
# Build wasm
wasm-pack build -t web

# Build npm pkg
# --scope YourName
wasm-pack build -t bundler --release --scope qmaru
```

## Dependencies

[rust-pinyin](https://github.com/mozillazg/rust-pinyin)
