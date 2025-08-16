# Git Commands

Always use `--no-abbrev-commit` and proper formatting flags for all git terminal commands to avoid shell parsing issues.

## Specific Commands to Use:

**Instead of:**
```bash
git log --oneline
git show --stat HEAD
git branch -v
```

**Use:**
```bash
git log --pretty=format:"%h %s" --no-abbrev-commit
git show --stat --no-abbrev-commit HEAD
git branch --show-current
```

## Key Flags:
- `--no-abbrev-commit` - Prevents abbreviated commit hashes
- `--pretty=format:"..."` - Use explicit formatting
- `--porcelain` - For cleaner output when available
- `--no-color` - Remove ANSI color codes
- `--show-current` - For branch operations

## Common Patterns:
- `git log master..HEAD` → `git log --pretty=format:"%h %s" --no-abbrev-commit master..HEAD`
- `git status` → `git status --porcelain`
- `git diff --name-only master` → `git diff --name-only master`
