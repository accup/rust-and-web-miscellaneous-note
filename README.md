# Rust and Web miscellaneous note

# 必須要件

- [rustup] 最新版
- [rustfmt]
- [wasm-pack] 最新版
- [Node.js] バージョンは [.node-version](./.node-version) 参照

[rustup]: https://rust-lang.github.io/rustup/
[rustfmt]: https://github.com/rust-lang/rustfmt
[wasm-pack]: https://rustwasm.github.io/wasm-pack/
[Node.js]: https://nodejs.org/ja/

# フォルダ構成

```
lib/rust/       Rust からビルドされた Wasm 等のファイルが生成されるディレクトリ
node_modules/   NPM パッケージがインストールされるディレクトリ
rust/src/       Rust ソースコードを含むディレクトリ
src/            Web 側のソースコードを含むディレクトリ
```
