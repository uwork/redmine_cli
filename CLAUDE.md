# redmine_cli

Redmine API を呼び出すコマンドラインツール。

## 概要

Redmine の REST API をラップし、ターミナルからチケット・プロジェクト・ユーザーなどを操作できる CLI。

## 技術スタック

- 言語: Python 3.10+
- HTTP クライアント: `requests`
- CLI フレームワーク: `click`
- 設定管理: `~/.config/redmine_cli/config.toml`

## セットアップ

```bash
pip install -e .
redmine config set --url https://your-redmine.example.com --api-key YOUR_API_KEY
```

## 開発

```bash
pip install -e ".[dev]"
pytest
```

## ディレクトリ構成

```
redmine_cli/
  __init__.py
  cli.py          # Click エントリーポイント
  client.py       # Redmine API クライアント
  config.py       # 設定ファイル読み書き
  commands/       # サブコマンド群
```
