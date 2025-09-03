---
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git branch:*), Bash(git merge-base:*), Bash(git log:*), Task
description: 現在のブランチの変更をcode-reviewerエージェントを使用して包括的にレビューします
---

# Review Current Branch Command

Review all changes made in the current branch using the code-reviewer agent.

## Usage
```
/review-current-branch [ISSUE_NUMBER]
```

### Arguments
- `ISSUE_NUMBER` (optional): Issue or task number to find the corresponding base branch
  - If provided: compares against the branch containing that issue number (e.g., TASK#6, Issue#123)
  - If omitted: compares against the main branch

## Command Features
1. Check current branch status with `git status`
2. Determine the base branch based on issue number argument:
   - If no argument provided: use 'main' as base branch
   - If argument provided: search for branch containing that issue number using `git branch --all | grep [ISSUE_NUMBER]`
3. Find the merge base between current branch and determined base branch using `git merge-base`
4. Get comprehensive diff of changes using `git diff [base-branch]...HEAD`
5. Analyze all modified files using `git log --name-only [base-branch]...HEAD`
6. Use Task tool to invoke code-reviewer agent with comprehensive review instructions
7. Display detailed review results in console

## Branch Detection Logic
When ISSUE_NUMBER is provided:
1. Search for branches containing the issue number: `git branch --all | grep -i "task#${ISSUE_NUMBER}\|issue#${ISSUE_NUMBER}"`
2. Filter out the current branch from results
3. Use the first matching branch as base branch
4. If no matching branch found, fall back to 'main' branch

## Code Review Execution
The command will invoke the code-reviewer agent with the following request structure:
```
Please conduct a comprehensive code review of the current branch changes:

**Branch Information**
- Current Branch: [CURRENT_BRANCH]
- Base Branch: [BASE_BRANCH]
- Merge Base: [MERGE_BASE_COMMIT]

**Changes to Review**
[Git diff output showing all changes]

**Modified Files**
[List of all modified files]

Follow the comprehensive review instructions from .claude/prompts/comprehensive-code-review.md

**Output Requirements**
- Report ALL issues found (including minor ones)
- Include specific fix suggestions for each issue
- Categorize by severity (Critical/High/Medium/Low)
- Specify file names and line numbers
- Provide Japanese language output as specified in project requirements
```

## Features
- Flexible base branch selection via issue number argument
- Reviews all files changed in the current branch
- Provides security, performance, and code quality analysis
- Offers specific improvement suggestions
- Considers Rust-specific best practices
- Evaluates GraphQL and project-specific patterns
- Intelligent branch detection for issue-based workflows

## Review Scope
The command reviews changes between the determined base branch and current branch using git merge-base to find the accurate divergence point and capture all modifications introduced by the current branch.

## Output
Displays comprehensive review results directly in the console with:
- Security vulnerability analysis
- Performance optimization suggestions  
- Code quality improvements
- Rust idiom recommendations
- Architecture and design feedback
- Testing and documentation suggestions

## Error Handling
- If git operations fail, display clear error messages
- If no changes are found between branches, notify user
- If branch detection fails, fall back to main branch comparison
- If code-reviewer agent fails, display error and suggest manual review

## Examples

### Basic usage (compare with main branch)
```
/review-current-branch
```

### Compare with specific issue branch
```
/review-current-branch 6
```
This will search for branches containing "TASK#6" or "Issue#6" and use the first match as base branch.

## Important Notes
- The command only reviews changes made in the current branch (not the entire codebase)
- Uses shared comprehensive code review instructions from `.claude/prompts/comprehensive-code-review.md`
- Review results are displayed in Japanese as per project requirements in CLAUDE.md
- All git operations are performed with proper error handling
- The code-reviewer agent is invoked using the Task tool for consistency

Note: This command uses the shared comprehensive code review instructions from @.claude/prompts/comprehensive-code-review.md