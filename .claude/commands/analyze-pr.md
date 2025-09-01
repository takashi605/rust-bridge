---
allowed-tools: Bash(gh pr diff:*), Bash(gh pr view:*), Bash(git diff:*), Bash(git log:*), Grep, Read
description: GitHub PRの差分を分析し、変更内容の構造化されたレポートを提供します
---

# Claude Command: Analyze PR

## Command Overview
Analyzes the diff of a specific PR in the current GitHub repository and provides a structured analysis of the changes. This command focuses purely on diff analysis without providing code improvement suggestions.

## Usage
```bash
/analyze-pr <PR_NUMBER>
```

When executed without arguments, it displays a list of available PRs:
```bash
/analyze-pr
```

## Command Behavior

### With PR number argument
1. Fetches the diff for the specified PR number
2. Parses file changes and modifications
3. Categorizes changes by functional units
4. Generates a structured analysis report

### Without arguments
1. Uses `gh pr list` to fetch available PRs in the repository
2. Displays PR titles and numbers
3. Prompts user to specify which PR number to analyze

## Output Contents
- **PR Diff Summary**: Quantitative data about change scope
- **Implementation Details**: Changes organized by layer/component
- **Impact Analysis**: Dependencies and test coverage
- **Detailed View Options**: Interactive diff exploration with actual code diffs

## Key Features
- 🔍 **Objective Analysis**: Fact-based information only
- 📊 **Quantitative Metrics**: File counts, line changes statistics
- 🗂️ **Structured Organization**: Classification by layers and features
- 🎯 **Efficient Review**: Drill down into specific areas as needed
- 📝 **Actual Code Diffs**: Shows real diff output when detail options are selected

## Use Cases
- Initial PR review to understand overall scope
- Impact assessment for large-scale changes
- Team communication about modifications
- Reference material for release notes

## Important Notes
- This command performs diff analysis only and does not include code quality assessments or improvement suggestions
- For review-focused feedback, please use dedicated review commands
- When a detail number is selected, the command MUST display the actual code diff using git diff format

## Interactive Detail View Behavior
When a user inputs a number from the "詳細な差分確認" section:
1. Display a brief summary of the selected component/feature
2. **ALWAYS show the actual code diff** for relevant files using standard git diff format:
   ```diff
   --- a/path/to/file.ext
   +++ b/path/to/file.ext
   @@ -line,count +line,count @@
   -removed lines
   +added lines
   ```
3. Include all related files for that feature/component
4. Provide file paths and line numbers for easy navigation

## Example Response Format
This command generates responses in Japanese, optimized for Japanese development teams. The output follows Japanese technical documentation conventions while maintaining objective, fact-based analysis of the PR differences. Detail views must include actual code diffs, not just descriptions.

```md
## PR差分サマリー

### 変更ファイル数
- 総ファイル数: 15ファイル
- 追加: 12ファイル / 修正: 3ファイル / 削除: 0ファイル
- 総行数: +1,245行 / -23行

### 機能追加の概要
- **バックエンド**: 新着記事一覧取得API
  - エンドポイント: GET /api/v1/posts/latest
  - 関連ファイル: 8ファイル
- **フロントエンド**: 新着記事一覧画面
  - ルート: /posts/latest
  - 関連ファイル: 7ファイル

## 実装内容の詳細

### バックエンド変更点
- **新規追加**
  - `LatestsPosts` エンティティ (posts/domain/entity.go)
  - 記事取得ユースケース (posts/usecase/get_latest.go)
  - ソート処理サービス (posts/domain/sort_service.go)
  - APIハンドラー (posts/handler/latest.go)
- **呼び出し関係**
  - handler → usecase → repository → domain

### フロントエンド変更点
- **新規追加**
  - LatestPostsコンポーネント (components/LatestPosts.tsx)
  - useLatestPostsクエリ (hooks/useLatestPosts.ts)
  - usePaginationフック (hooks/usePagination.ts)
- **表示仕様**
  - デスクトップ: 21件表示（3列×7行）
  - モバイル: 10件表示（2列×5行）
  - ページネーション実装済み

## 影響範囲の分析

### 依存関係
- 既存の記事詳細画面への遷移機能を利用
- 共通コンポーネント: Button, Card, Pagination を使用
- APIクライアント: 既存のaxiosインスタンスを利用

### テストカバレッジ
- バックエンド: ユニットテスト3ファイル追加
- フロントエンド: コンポーネントテスト2ファイル追加

## 詳細な差分確認

以下の番号を入力すると、該当箇所の具体的な差分を表示します：

1. バックエンド - 新着記事取得ハンドラー実装 (handler/latest.go)
2. バックエンド - GetLatestPostsユースケース追加 (usecase/get_latest.go)
3. バックエンド - LatestPostsエンティティとソートサービス追加 (domain/entity.go, domain/sort_service.go)
4. バックエンド - 新着記事取得リポジトリ実装 (repository/latest_posts.go)
5. フロントエンド - LatestPostsコンポーネント実装 (components/LatestPosts.tsx)
6. フロントエンド - データ取得とページネーションフック追加 (hooks/useLatestPosts.ts, hooks/usePagination.ts)
7. フロントエンド - 新着記事画面のルート設定追加 (router/index.tsx)

```

## Example Response Format For "詳細な差分確認"

ユーザーが「5」を選択した場合（フロントエンド：LatestPostsコンポーネント）：

```markdown
## 5. フロントエンド：LatestPostsコンポーネント

### 差分

#### styles/LatestPosts.module.css (新規作成)
```diff
+++ b/styles/LatestPosts.module.css
@@ -0,0 +1,55 @@
+.latest-posts-container {
+  max-width: 1200px;
+  margin: 0 auto;
+  padding: 2rem;
+}
+
+.page-title {
+  font-size: 2rem;
+  font-weight: bold;
+  margin-bottom: 2rem;
+  text-align: center;
+}
```

### 分析結果

#### ファイル構成
- CSS Modulesファイルを新規作成
- コンテナとタイトルの基本スタイルを定義
```