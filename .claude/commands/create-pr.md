# Claude Command: Create PR
This guide explains how to create pull requests using GitHub CLI in our project.
**Important**: All PR titles and descriptions should be written in Japanese.

## Base Branch Inference from Issue Number
This command accepts an Issue number as an argument and automatically infers the base branch by searching for branches containing that number.

Usage example:
```bash
/create-pr 11
```
This will search for branches with the `TASK#11_*` format and use it as the base branch.

## Prerequisites
1. Install GitHub CLI if you haven't already:
   ```bash
   # macOS
   brew install gh
   # Windows
   winget install --id GitHub.cli
   # Linux
   # Follow instructions at https://github.com/cli/cli/blob/trunk/docs/install_linux.md
   ```
2. Authenticate with GitHub:
   ```bash
   gh auth login
   ```

## Creating a New Pull Request

### With Issue Number (Recommended)
When you provide an Issue number, the command will automatically find the appropriate base branch:
```bash
# The command will search for TASK#11_* branches and use it as base
/create-pr 11
```

### Manual Base Branch Specification
1. First, prepare your PR description following the template in @.github/pull_request_template.md
2. Use the `gh pr create --draft` command to create a new pull request:
   ```bash
   # Basic command structure with auto-detected base
   gh pr create --draft --title "✨(scope): 説明的なタイトル" --body "PRの説明" --base DETECTED_BASE_BRANCH
   ```
   For more complex PR descriptions with proper formatting, use the `--body-file` option with the exact PR template structure:
   ```bash
   # Create PR with proper template structure and auto-detected base
   gh pr create --draft --title "✨(scope): 説明的なタイトル" --body-file .github/pull_request_template.md --base DETECTED_BASE_BRANCH
   ```

## Best Practices
1. **Language**: Always use Japanese for PR titles and descriptions
2. **PR Title Format**: Use conventional commit format with emojis
   - Always include an appropriate emoji at the beginning of the title
   - Use the actual emoji character (not the code representation like `:sparkles:`)
   - Examples:
     - `✨(supabase): ステージング環境のリモート設定を追加`
     - `🐛(auth): ログインリダイレクトの問題を修正`
     - `📝(readme): インストール手順を更新`
3. **Description Template**: Always use our PR template structure from @.github/pull_request_template.md:
4. **Template Accuracy**: Ensure your PR description precisely follows the template structure:
   - Don't modify or rename the PR-Agent sections (`pr_agent:summary` and `pr_agent:walkthrough`)
   - Keep all section headers exactly as they appear in the template
   - Don't add custom sections that aren't in the template
5. **Draft PRs**: Start as draft when the work is in progress
   - Use `--draft` flag in the command
   - Convert to ready for review when complete using `gh pr ready`

### Common Mistakes to Avoid
1. **Using English Text**: All PR content must be in Japanese
2. **Incorrect Section Headers**: Always use the exact section headers from the template
3. **Adding Custom Sections**: Stick to the sections defined in the template
4. **Using Outdated Templates**: Always refer to the current @.github/pull_request_template.md file

### Missing Sections
Always include all template sections, even if some are marked as "N/A" or "なし"

## Additional GitHub CLI PR Commands
Here are some additional useful GitHub CLI commands for managing PRs:
```bash
# List your open pull requests
gh pr list --author "@me"
# Check PR status
gh pr status
# View a specific PR
gh pr view <PR-NUMBER>
# Check out a PR branch locally
gh pr checkout <PR-NUMBER>
# Convert a draft PR to ready for review
gh pr ready <PR-NUMBER>
# Add reviewers to a PR
gh pr edit <PR-NUMBER> --add-reviewer username1,username2
# Merge a PR
gh pr merge <PR-NUMBER> --squash
```

## Using Templates for PR Creation
To simplify PR creation with consistent descriptions, you can create a template file:
1. Create a file named `pr-template.md` with your PR template
2. Use it when creating PRs:
```bash
gh pr create --draft --title "feat(scope): タイトル" --body-file pr-template.md --base main
```

## Related Documentation
- [PR Template](/.github/pull_request_template.md)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub CLI documentation](https://cli.github.com/manual/)
