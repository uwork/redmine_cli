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

## テスト方針

### 基本ルール

- テストは各ファイル末尾の `#[cfg(test)] mod tests { ... }` に記述する
- ネットワーク・ファイルシステムに依存しない純粋な単体テストを優先する
- テスト名は「何をテストするか」を英語スネークケースで明示する

### テスト対象と優先順位

1. **純粋ロジック** (最優先): 入出力が確定できるもの
   - `Config::require_url` / `require_api_key` のエラー/成功パス
   - `User::full_name` などのメソッド
   - TOML シリアライズ/デシリアライズの往復確認

2. **JSON 入出力** (優先): API 型の正確さを保証
   - `serde_json` を使ったデシリアライズ検証
   - `#[serde(skip_serializing_if)]` が None フィールドを省略するか
   - `Default` 実装が空オブジェクトにシリアライズされるか

3. **初期化ロジック**: 副作用のない構築処理
   - `RedmineClient::new` が URL の末尾スラッシュをトリムするか

### やらないこと

- 実際の HTTP 通信を伴うテスト (モックなしでは不要)
- ファイルシステムを変更する `Config::save` / `Config::load` のテスト
- `std::env::set_var` を使う環境変数テスト (並列実行時に競合するため)

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
