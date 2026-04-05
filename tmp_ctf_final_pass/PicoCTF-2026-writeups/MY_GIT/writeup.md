# MY GIT - picoCTF 2026

**Category:** General Skills
**Points:** 50

## Challenge Description
I have built my own Git server with my own rules!

## Approach
This challenge involves interacting with a custom Git server that has non-standard behavior. The "my own rules" phrasing suggests that the server may:

1. **Have custom Git hooks** that enforce specific rules (pre-receive, post-receive, update hooks)
2. **Use non-standard branch naming** or require specific operations
3. **Hide the flag** in a non-obvious location within the repo (branches, tags, commit messages, notes, stash, or git objects)
4. **Require specific Git commands** to extract the flag (e.g., `git log`, `git branch -a`, `git tag`, `git notes`, `git reflog`)

Common patterns in picoCTF Git challenges:
- Flag split across multiple branches
- Flag hidden in old commits (use `git log --all`)
- Flag in a deleted branch (use `git reflog` or `git fsck`)
- Flag embedded in commit messages or author fields
- Flag stored as a git note or tag annotation
- Custom server that requires specific HTTP/SSH interactions

### Strategy:
1. Clone the repository from the provided URL
2. Enumerate all refs: branches, tags, notes
3. Search the entire commit history
4. Check for hidden or unusual objects

## Solution

### Step 1: Clone the repository
```bash
# The challenge typically provides a URL like:
git clone <challenge_url>
cd <repo_name>
```

### Step 2: Enumerate all branches
```bash
git branch -a
# Check all branches, including remote-tracking branches
git branch -r
```

### Step 3: Check all commit history
```bash
# View all commits across all branches
git log --all --oneline --graph

# Search commit messages for the flag
git log --all --format="%H %s" | grep -i "flag\|pico\|secret"

# Search commit diffs for the flag
git log --all -p | grep "picoCTF{"
```

### Step 4: Check tags
```bash
git tag -l
# Show all tag details (annotated tags may contain the flag)
git tag -l | xargs -I{} git show {}
```

### Step 5: Check git notes
```bash
git notes list
git log --show-notes='*'
```

### Step 6: Check stash
```bash
git stash list
git stash show -p
```

### Step 7: Check for dangling objects
```bash
git fsck --unreachable --no-reflogs
# Examine each dangling object
git fsck --lost-found
cat .git/lost-found/other/*
```

### Step 8: Check reflog
```bash
git reflog --all
```

### Step 9: Explore custom server behavior
```bash
# Try listing remote refs directly
git ls-remote origin

# Try fetching all refs including non-standard ones
git fetch origin '+refs/*:refs/remotes/origin/*'

# Check for custom refs
git for-each-ref
```

### Step 10: Check hidden refs or custom namespaces
```bash
# Some servers use custom ref namespaces
git ls-remote origin | grep -v "HEAD\|main\|master"

# Fetch everything
git fetch --all --tags --prune

# Check .git/config for unusual remote configurations
cat .git/config
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
