# redmine_cli

Redmine API を呼び出すコマンドラインツール。

## 概要

Redmine の REST API をラップし、ターミナルからチケット・プロジェクト・ユーザーなどを操作できる CLI。

## 技術スタック

- 言語: Rust (edition 2021)
- CLI フレームワーク: `clap` (derive マクロ)
- HTTP クライアント: `reqwest` (async + TLS)
- ランタイム: `tokio`
- シリアライズ: `serde` / `serde_json`
- 設定ファイル: `toml` / `dirs`
- テーブル出力: `tabled`
- エラー処理: `anyhow`

## セットアップ

```bash
cargo build --release
./target/release/redmine config set --url https://your-redmine.example.com --api-key YOUR_API_KEY
```

## 開発

```bash
cargo build       # ビルド
cargo test        # テスト実行
cargo clippy      # Lint
cargo fmt         # フォーマット
```

## ディレクトリ構成

```
src/
  main.rs         # エントリーポイント
  cli.rs          # Clap コマンド定義
  client.rs       # Redmine API クライアント
  config.rs       # 設定ファイル読み書き
  commands/       # サブコマンド実装
    mod.rs
    issues.rs
    projects.rs
    users.rs
    time_entries.rs
  models/         # API レスポンス型定義
    mod.rs
    issue.rs
    project.rs
    user.rs
```
