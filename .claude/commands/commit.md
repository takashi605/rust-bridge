---
allowed-tools: Bash(git status:*), Bash(git add:*), Bash(git diff:*), Bash(git branch:*)
description: 適切なコミットメッセージとemojisを使用したwell-formattedなコミットを作成します
---

# Claude Command: Commit

This command helps create well-formatted commits using conventional commit messages and emojis.

## Usage

To create a commit, simply enter:

```
/commit
```

## Commit Best Practices
- **Pre-commit verification**: Ensure code is linted, builds correctly, and documentation is updated
- **Atomic commits**: Each commit should contain related changes that serve a single purpose
- **Split large changes**: If changes span multiple concerns, split into separate commits
- **Conventional commit format**: Use `<type>: <description>` format. Type should be one of:
  - feat: New features
  - fix: Bug fixes
  - docs: Documentation changes
  - style: Code style changes (formatting, etc.)
  - refactor: Code changes that neither fix bugs nor add features
  - perf: Performance improvements
  - test: Adding or correcting tests
  - chore: Changes to build process, tools, etc.
- **Present tense, imperative mood**: Write commit messages as commands (e.g., "add feature" not "added feature")
- **Concise first line**: Keep the first line under 72 characters
- **Emojis**: Each commit type is paired with appropriate emojis:
  - ✨ feat: New features
  - 🐛 fix: Bug fixes
  - 📝 docs: Documentation
  - 💄 style: Formatting/styling
  - ♻️ refactor: Code refactoring
  - ⚡️ perf: Performance improvements
  - ✅ test: Testing
  - 🔧 chore: Tools, configuration
  - 🚀 ci: CI/CD improvements
  - 🗑️ revert: Reverting changes
  - 🧪 test: Adding failing tests
  - 🚨 fix: Fixing compiler/linter warnings
  - 🔒️ fix: Fixing security issues
  - 👥 chore: Adding or updating contributors
  - 🚚 refactor: Moving or renaming resources
  - 🏗️ refactor: Making architectural changes
  - 🔀 chore: Merging branches
  - 📦️ chore: Adding or updating compiled files or packages
  - ➕ chore: Adding dependencies
  - ➖ chore: Removing dependencies
  - 🌱 chore: Adding or updating seed files
  - 🧑‍💻 chore: Improving developer experience
  - 🧵 feat: Adding or updating code related to multithreading or concurrency
  - 🔍️ feat: Improving SEO
  - 🏷️ feat: Adding or updating types
  - 💬 feat: Adding or updating text and literals
  - 🌐 feat: Internationalization and localization
  - 👔 feat: Adding or updating business logic
  - 📱 feat: Working on responsive design
  - 🚸 feat: Improving user experience/usability
  - 🩹 fix: Simple fix for a non-critical issue
  - 🥅 fix: Catching errors
  - 👽️ fix: Updating code due to external API changes
  - 🔥 fix: Removing code or files
  - 🎨 style: Improving structure/format of the code
  - 🚑️ fix: Critical hotfix
  - 🎉 chore: Beginning a project
  - 🔖 chore: Releasing/version tags
  - 🚧 wip: Work in progress
  - 💚 fix: Fixing CI build
  - 📌 chore: Pinning dependencies to specific versions
  - 👷 ci: Adding or updating CI build system
  - 📈 feat: Adding or updating analytics or tracking code
  - ✏️ fix: Fixing typos
  - ⏪️ revert: Reverting changes
  - 📄 chore: Adding or updating license
  - 💥 feat: Introducing breaking changes
  - 🍱 assets: Adding or updating assets
  - ♿️ feat: Improving accessibility
  - 💡 docs: Adding or updating comments in source code
  - 🗃️ db: Performing database related changes
  - 🔊 feat: Adding or updating logs
  - 🔇 fix: Removing logs
  - 🤡 test: Creating mocks
  - 🥚 feat: Adding or updating an easter egg
  - 🙈 chore: Adding or updating a .gitignore file
  - 📸 test: Adding or updating snapshots
  - ⚗️ experiment: Performing experiments
  - 🚩 feat: Adding, updating, or removing feature flags
  - 💫 ui: Adding or updating animations and transitions
  - ⚰️ refactor: Removing dead code
  - 🦺 feat: Adding or updating code related to validation
  - ✈️ feat: Improving offline support

## Commit Splitting Guidelines

When analyzing diff, consider splitting commits based on the following criteria:

1. **Different concerns**: Changes to unrelated parts of the codebase
2. **Different types of changes**: Mixing features, fixes, refactoring, etc.
3. **File patterns**: Changes to different kinds of files (e.g., source code vs documentation)
4. **Logical grouping**: Changes that are easier to understand or review separately
5. **Size**: Very large changes that would be clearer when split

## Examples

Good commit messages (generated in Japanese):

- ✨ feat: ユーザー認証システムを追加
- 🐛 fix: レンダリング処理のメモリリークを修正
- 📝 docs: 新しいエンドポイントの API ドキュメントを更新
- ♻️ refactor: パーサーのエラーハンドリングを簡素化
- 🚨 fix: コンポーネントのリンター警告を解消
- 🧑‍💻 chore: 開発ツールセットアップ手順を改善
- 👔 feat: 取引バリデーションのビジネスロジックを実装
- 🩹 fix: ヘッダーの軽微なスタイル不整合を修正
- 🚑️ fix: 認証フローの重大なセキュリティ脆弱性を修正
- 🎨 style: コンポーネント構造を整理し可読性向上
- 🔥 fix: 旧式コードを削除
- 🦺 feat: ユーザー登録フォームに入力バリデーションを追加
- 💚 fix: CI パイプラインの失敗を修正
- 📈 feat: ユーザーエンゲージメント用の分析トラッキングを実装
- 🔒️ fix: パスワード要件を強化
- ♿️ feat: スクリーンリーダー対応を改善

Commit splitting examples:
1. ✨ feat: 新しい solc バージョン用型定義を追加
2. 📝 docs: 新しい solc バージョンのドキュメント更新
3. 🔧 chore: package.json の依存関係を更新
4. 🏷️ feat: 新 API エンドポイントの型定義を追加
5. 🧵 feat: ワーカースレッドの並列処理を改善
6. 🚨 fix: 新コードのリンターエラーを修正
7. ✅ test: 新機能用ユニットテストを追加
8. 🔒️ fix: 脆弱性のある依存パッケージを更新


## Command Features
1. Check staged files with `git status`
2. If there are 0 staged files, automatically add all changed and new files with `git add`
3. Run `git diff` to understand the changes being committed
4. **Code Review**: Use code-reviewer agent to analyze the changes for code quality, security, performance, and best practices
   - If **critical issues** are found (security vulnerabilities, major bugs, breaking changes), stop and ask user for confirmation before committing
   - If **minor issues** are found (style improvements, optimization suggestions), proceed with commit and provide improvement suggestions to user afterward
5. Analyze diff to determine if multiple distinct logical changes exist
6. If multiple distinct changes are detected, suggest splitting the commit into multiple smaller commits
7. For each commit (or single commit if not splitting), create commit message in conventional commit format with emoji
8. Run `git branch --contains` to understand the current branch's Issue number. Include Issue number in commit message (example: ✨ feat: ブログ記事一覧画面からブログ記事編集画面へのリンクを追加 #3)

## Important Notes
- If specific files are already staged, the command will commit only those files
- If no files are staged, it will automatically stage all changed and new files
- Commit messages are constructed based on detected changes
- Before committing, the command checks the diff to identify whether multiple commits would be more appropriate
- When suggesting multiple commits, it will help stage and commit changes individually
- Always review the commit diff to ensure the message matches the changes

**Note**: All commit messages will be generated in Japanese to match the project's language requirements, while this documentation is provided in English for better international understanding.
