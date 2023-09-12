# rpinyin

rust wasm pinyin

## Install

```shell
cargo install wasm-pack
```

## Build

```shell
# Build wasm
wasm-pack build --target web

# Build npm pkg
wasm-pack build --target bundler
cd pkg
npm link
```

## Dependencies

[rust-pinyin](https://github.com/mozillazg/rust-pinyin)
