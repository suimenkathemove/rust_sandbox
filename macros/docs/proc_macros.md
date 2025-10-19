# Procedural Macros, 手続きマクロ

## Overview

macro_rules!の宣言マクロより表現力が高い。

## しくみ

Rust codeをsyn crateでparseしてASTに変換する。

<!-- TODO -->

## Custom Derive

<!-- TODO -->

## Attribute-like

<!-- TODO -->

## Function-like

<!-- TODO -->

## 準備

proc-macroを作る場合は、それ専用のpackageを作成する必要がある。
Cargo.tomlに`proc-macro = true`を追加する必要がある。

```toml
[lib]
proc-macro = true
```

以下の3種類のcrateを追加する。

- proc-macro2
- quote
  - ASTからRust codeを生成する
- syn
  - Rust codeをparseしてASTに変換する

## デバッグ

### cargo-extend
