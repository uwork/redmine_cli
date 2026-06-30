# redmine_cli

Redmine の REST API をコマンドラインから操作できる CLI ツール。

## 概要

`redmine_cli` は Redmine の REST API をラップし、チケットの一覧・作成・更新・クローズなどの日常操作をターミナルから素早く行えるようにするツールです。

```bash
# チケット一覧を表示
redmine issues list --project myproject

# チケットを作成
redmine issues create --project myproject --subject "バグ修正" --priority high

# チケットの詳細を表示
redmine issues show 1234

# チケットを更新
redmine issues update 1234 --status "In Progress" --assigned-to me
```

## インストール

```bash
pip install redmine-cli
```

または開発版:

```bash
git clone https://github.com/uwork/redmine_cli.git
cd redmine_cli
pip install -e .
```

## 設定

```bash
redmine config set --url https://your-redmine.example.com --api-key YOUR_API_KEY
```

設定は `~/.config/redmine_cli/config.toml` に保存されます。

## 実装予定機能

### 設定管理
- [ ] `redmine config set` — URL・API キーの保存
- [ ] `redmine config show` — 現在の設定を表示
- [ ] 複数プロファイル対応（本番/ステージング切り替え）

### チケット (Issues)
- [ ] `redmine issues list` — チケット一覧（フィルタ: プロジェクト・担当者・ステータス・優先度）
- [ ] `redmine issues show <id>` — チケット詳細表示
- [ ] `redmine issues create` — チケット作成
- [ ] `redmine issues update <id>` — チケット更新（ステータス・担当者・優先度など）
- [ ] `redmine issues close <id>` — チケットをクローズ
- [ ] `redmine issues comment <id>` — コメント追加

### プロジェクト (Projects)
- [ ] `redmine projects list` — プロジェクト一覧
- [ ] `redmine projects show <id>` — プロジェクト詳細

### ユーザー (Users)
- [ ] `redmine users list` — ユーザー一覧
- [ ] `redmine users me` — 自分自身の情報を表示

### 時間記録 (Time Entries)
- [ ] `redmine time log <issue_id>` — 作業時間を記録
- [ ] `redmine time list` — 時間記録の一覧

### 出力フォーマット
- [ ] テーブル形式（デフォルト）
- [ ] JSON 出力 (`--format json`)
- [ ] CSV 出力 (`--format csv`)

### その他
- [ ] ページネーション対応
- [ ] 環境変数による設定上書き (`REDMINE_URL`, `REDMINE_API_KEY`)
- [ ] シェル補完スクリプト生成

## ライセンス

MIT
