# このリポジトリについて
Bridge パターンの学習用リポジトリです。
Rust を用いて、GraphQL API を作成します。これは API テスト駆動で作ります。

重要：ClaudeCodeAIAgent は、プロジェクトの基盤のみを作成してください。学習用リポジトリのため、基本的なコードは私が書いていきます。

## ClaudeCodeAIAgent への指示
- やり取りは日本語で行ってください。
- コード作成・修正タスクでは必ず coder エージェントを使用してください
- コードレビューが必要な場合は code-reviewer エージェントを使用してください
- 複雑なタスクは適切な専門エージェントに委譲してください
- 新しいコードを書く際は coder エージェント使用必須
- バグ修正時は coder エージェント使用必須
- リファクタリング時は coder エージェント使用必須
- テスト作成時は tester エージェント使用必須

## プロジェクト要件
1. GraphQL のリクエスト送信方法を学ぶため、学習用テスト(Learning Test)を書く。具体的には、[https://countries.trevorblades.com](https://countries.trevorblades.com/) に対する api テストを書く
2. 実行環境はコンテナ。API コンテナと API テストコンテナ、及びDBコンテナの3種類を用意する。
3. API エンドポイントとして、簡単な CRUD 操作を作成する。
4. API テストでは実際に DB 操作をテストするため、テスト実行ごとに DB のデータをクリーンアップする処理が必要である。これは、Makeコマンドの作成により実現する。

## 仕様技術
使用言語：Rust
動作環境： Dockerコンテナ(Docker/Docker Compose)

## ディレクトリ構成
```
rust-bridge(root)/
├ containers/
│   ├ api/
│   │ └ Dockerfile
│   ├ api-test/
│   │ └ Dockerfile
│   └ db/
│     ├ Dockerfile
│     └ init/
│       ├ 01_create_tables.sql
│       └ 02_insert_test_data.sql
├ application/           # Workspaceルート
│   ├ Cargo.toml        # Workspaceメンバー設定
│   ├ Cargo.lock
│   └ bacon.toml
├ compose.yaml
├ Makefile
├ CLAUDE.md
└ README.md
```

## クレートの構成
```
application/
├ api_schema/           # GraphQLスキーマ定義
├ api/                  # GraphQL APIサーバー
├ api_test/             # APIテストクレート
├ repositories/         # データアクセス層（新規追加）
└ mailer/              # メール送信機能（新規追加）
```

### api_schema クレート
- rust コードベースの graphql スキーマを定義

### api クレート
- GraphQL API サーバー
- 主要な依存クレートは以下
  - anyhow
  - async-graphql
  - async-graphql-actix-web
  - actix-web
  - tokio

### api_test クレート
- api クレートで立ち上げたサーバーに対して api テストを実行する
- [https://countries.trevorblades.com](https://countries.trevorblades.com/) に対する api テストを書き、基本的な GraphQL リクエストの方法を学ぶ
- MailHogを使用したメール送信のテストも含む
- 主要な依存クレートは以下
  - anyhow
  - cynic
  - reqwest
  - tokio

### repositories クレート（新規追加）
- データアクセス層を提供するクレート
- PostgreSQLとの接続とCRUD操作を担当
- モックとリアルな実装の両方を提供
- 主要な依存クレートは以下
  - anyhow
  - tokio
  - serde
  - sqlx（PostgreSQL接続）
  - async-trait

### mailer クレート（新規追加）
- メール送信機能を提供するクレート
- SMTPサーバーを通じたメール送信
- テスト環境ではMailHogとの連携
- 主要な依存クレートは以下
  - lettre（SMTP送信）
  - anyhow
  - uuid（メッセージID生成）
  - tokio
  - html-escape
  - async-trait
  - dotenvy（環境変数管理）

## コンテナサービス
### api サービス
- compose up したときに起動

### api-test サービス
- test profile を指定して compose up したときにのみ実行
- api サービスに依存している

## コードスタイル
- コードスタイルは Rust の公式スタイルガイドに従う。
- 最新の Rust 構文に従い、`mod.rs` ファイルは使わない。

sample:
```rust
use actix_web::{web, Scope};

pub fn sample_scope() -> Scope {
  web::scope("")
    .service(handle_funcs::hello)
    .service(handle_funcs::echo)
    .service(handle_funcs::fivesix)
    .route("/hey", web::get().to(handle_funcs::manual_hello))
}

mod handle_funcs {
  use actix_web::{get, post, HttpResponse, Responder};

  #[get("/")]
  pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
  }

  #[post("/")]
  pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
  }

  #[get("/fivesix")]
  pub async fn fivesix() -> impl Responder {
    #[derive(serde::Serialize)]
    struct Numbers {
      num1: i32,
      num2: i32,
    }
    let numbers = Numbers { num1: 5, num2: 6 };
    HttpResponse::Ok().json(numbers)
  }

  pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
  }
}
```

## エラーハンドリング
- エラーハンドリングは、`anyhow` クレートを使用して行う。
- 最終的なエラー処理は以下の箇所でのみ行う。
  - `api/src/main.rs` の `main` 関数内で、実行時のエラーをキャッチし、処理を中断もしくはリカバーを行う。
  - 各エンドポイントのハンドラ関数内でエラーが発生した場合は適切な HTTP ステータスコードとメッセージを返す。
- 上記以外の箇所では、エラーを `anyhow::Result` 型で返却し、呼び出し元で適切に処理されることを期待する。

## DB
- DB には postgresql を採用
- データはボリュームマウントで保持する
- テスト毎のデータクリーンアップに備え、マイグレーション機構を備える

## Makefile コマンド
プロジェクトルートで以下のコマンドを実行してください。

### `make build`
- Docker Compose のコンテナをビルドします

### `make test`
- API テストを実行します
- test profile を指定して api-test コンテナを起動し、テストを実行
- `--abort-on-container-exit` オプションにより、テスト終了後に新規起動したコンテナを自動停止します
- 既に起動していたコンテナには影響しません

### `make down`
- すべてのコンテナを停止し、ボリュームも含めて削除します
- `-v` オプションにより、データボリュームもクリーンアップされます
