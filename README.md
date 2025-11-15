# log-parser

Rustで開発するログファイル解析・フィルタリングCLIツール

## 概要

log-parserは様々な形式のログファイルを効率的に解析し、必要な情報をフィルタリング・抽出するためのコマンドラインツールです。

## 主な機能

### フィルタリング機能

- **日時範囲フィルタ** - 指定した期間内のログのみを抽出
- **ログレベルフィルタ** - ERROR、WARN、INFO、DEBUG等でフィルタ
- **キーワード検索** - 正規表現対応の文字列検索・除外
- **IPアドレスフィルタ** - 特定のIPアドレスからのアクセスログを抽出
- **HTTPステータスフィルタ** - レスポンスコード別でフィルタ

### 出力機能

- **フォーマット変換** - JSON、CSV、プレーンテキスト形式で出力
- **統計情報表示** - エラー数、アクセス数等の集計
- **カラー出力** - 重要度に応じた色分け表示

### 対応ログ形式

- Apache/Nginx access log
- アプリケーションログ（JSON形式）
- syslog形式
- カスタムログ形式（設定ファイルで定義可能）

## 使用例

```bash
# エラーレベルのログのみ表示
log-parser app.log --level error

# 特定期間のログを抽出
log-parser access.log --since "2024-01-01" --until "2024-01-31"

# キーワードで検索
log-parser app.log --grep "database.*timeout"

# JSON形式で出力
log-parser nginx.log --format json

# 特定IPのアクセスログ
log-parser access.log --ip 192.168.1.100

# 5xx エラーのみ
log-parser access.log --status 5xx

# 複数条件の組み合わせ
log-parser app.log --level error --since "2024-01-01" --grep "payment" --format json
```

## インストール

```bash
cargo build --release
cp target/release/log-parser /usr/local/bin/
```

## 技術スタック

- **Rust** - メイン言語
- **clap** - CLI引数解析
- **regex** - 正規表現処理
- **chrono** - 日時処理
- **serde** - JSON serialization
- **rayon** - 並列処理による高速化
- **anyhow** - エラーハンドリング

## 開発予定機能

- [ ] リアルタイム監視機能
- [ ] ログローテーション対応
- [ ] 設定ファイル対応
- [ ] プラグインシステム
- [ ] Web UI
- [ ] 分散ログ対応

## ライセンス

MIT License
