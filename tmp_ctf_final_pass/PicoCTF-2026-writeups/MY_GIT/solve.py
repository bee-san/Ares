#!/usr/bin/env python3
"""
MY GIT - picoCTF 2026
Category: General Skills | Points: 50

Interacts with a custom Git server to find a hidden flag.
This script clones the repo and performs exhaustive enumeration of
branches, tags, commits, notes, stash, and dangling objects.

Usage:
    python3 solve.py <git_clone_url>
    python3 solve.py https://challenge.picoctf.org/repo.git
"""

import subprocess
import sys
import os
import re
import tempfile

FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")


def run(cmd, cwd=None, timeout=30):
    """Run a shell command and return stdout + stderr combined."""
    try:
        result = subprocess.run(
            cmd, shell=True, capture_output=True, text=True,
            cwd=cwd, timeout=timeout
        )
        return (result.stdout + "\n" + result.stderr).strip()
    except subprocess.TimeoutExpired:
        return ""


def search_for_flag(text):
    """Search text for picoCTF flag pattern."""
    return FLAG_PATTERN.findall(text)


def clone_repo(url, dest):
    """Clone the git repository."""
    print(f"[*] Cloning {url}...")
    output = run(f"git clone {url} {dest}")
    print(f"    {output}")

    if not os.path.exists(os.path.join(dest, ".git")):
        # Try with --mirror for bare repos
        print("[*] Trying mirror clone...")
        output = run(f"git clone --mirror {url} {dest}/.git")
        if os.path.exists(os.path.join(dest, ".git")):
            run("git config --bool core.bare false", cwd=dest)
            run("git checkout", cwd=dest)

    return os.path.exists(os.path.join(dest, ".git"))


def enumerate_repo(repo_dir):
    """Exhaustively enumerate a git repo for flags."""
    all_flags = []

    # ── 1. List remote refs ──
    print("\n[*] Step 1: Listing remote refs...")
    output = run("git ls-remote origin 2>/dev/null", cwd=repo_dir)
    print(f"    Remote refs:\n{output}")
    all_flags.extend(search_for_flag(output))

    # Fetch ALL refs (including non-standard namespaces)
    print("[*] Fetching all refs...")
    run("git fetch --all --tags --prune 2>/dev/null", cwd=repo_dir)
    run("git fetch origin '+refs/*:refs/remotes/origin/*' 2>/dev/null", cwd=repo_dir)
    # Some challenges hide refs under custom namespaces
    run("git fetch origin '+refs/hidden/*:refs/hidden/*' 2>/dev/null", cwd=repo_dir)
    run("git fetch origin '+refs/secret/*:refs/secret/*' 2>/dev/null", cwd=repo_dir)
    run("git fetch origin '+refs/flag/*:refs/flag/*' 2>/dev/null", cwd=repo_dir)

    # ── 2. List all branches ──
    print("\n[*] Step 2: Enumerating branches...")
    output = run("git branch -a", cwd=repo_dir)
    print(f"    Branches:\n{output}")
    branches = [b.strip().lstrip("* ").strip() for b in output.split("\n") if b.strip()]

    # ── 3. Check each branch ──
    print("\n[*] Step 3: Checking each branch...")
    for branch in branches:
        branch_name = branch.replace("remotes/origin/", "").replace("remotes/", "")
        if "HEAD" in branch:
            continue

        # Log with patches
        log_output = run(f"git log {branch} --format='%H|%an|%ae|%s' 2>/dev/null", cwd=repo_dir)
        flags = search_for_flag(log_output)
        if flags:
            print(f"    [+] Flag in commit metadata on {branch}: {flags}")
            all_flags.extend(flags)

        # Check actual file contents at branch tip
        show_output = run(f"git show {branch}: 2>/dev/null", cwd=repo_dir)
        # List and show all files on this branch
        ls_output = run(f"git ls-tree -r --name-only {branch} 2>/dev/null", cwd=repo_dir)
        if ls_output:
            for filepath in ls_output.split("\n"):
                filepath = filepath.strip()
                if filepath:
                    content = run(f"git show {branch}:{filepath} 2>/dev/null", cwd=repo_dir)
                    flags = search_for_flag(content)
                    if flags:
                        print(f"    [+] Flag in {branch}:{filepath}: {flags}")
                        all_flags.extend(flags)

        # Check diffs
        diff_output = run(f"git log {branch} -p 2>/dev/null", cwd=repo_dir)
        flags = search_for_flag(diff_output)
        if flags:
            print(f"    [+] Flag in diff on {branch}: {flags}")
            all_flags.extend(flags)

    # ── 4. Check tags ──
    print("\n[*] Step 4: Checking tags...")
    tags_output = run("git tag -l", cwd=repo_dir)
    if tags_output:
        print(f"    Tags: {tags_output}")
        for tag in tags_output.split("\n"):
            tag = tag.strip()
            if tag:
                show = run(f"git show {tag} 2>/dev/null", cwd=repo_dir)
                flags = search_for_flag(show)
                if flags:
                    print(f"    [+] Flag in tag {tag}: {flags}")
                    all_flags.extend(flags)
    else:
        print("    No tags found")

    # ── 5. Check notes ──
    print("\n[*] Step 5: Checking git notes...")
    # Fetch notes
    run("git fetch origin 'refs/notes/*:refs/notes/*' 2>/dev/null", cwd=repo_dir)
    notes_output = run("git notes list 2>/dev/null", cwd=repo_dir)
    if notes_output:
        print(f"    Notes: {notes_output}")
        for line in notes_output.split("\n"):
            parts = line.strip().split()
            if len(parts) >= 2:
                note_content = run(f"git notes show {parts[1]} 2>/dev/null", cwd=repo_dir)
                flags = search_for_flag(note_content)
                if flags:
                    print(f"    [+] Flag in note: {flags}")
                    all_flags.extend(flags)

    # Also check all notes namespaces
    notes_log = run("git log --show-notes='*' --all 2>/dev/null", cwd=repo_dir)
    flags = search_for_flag(notes_log)
    if flags:
        print(f"    [+] Flag in notes log: {flags}")
        all_flags.extend(flags)

    # ── 6. Check stash ──
    print("\n[*] Step 6: Checking stash...")
    stash_output = run("git stash list 2>/dev/null", cwd=repo_dir)
    if stash_output:
        print(f"    Stash: {stash_output}")
        stash_diff = run("git stash show -p 2>/dev/null", cwd=repo_dir)
        flags = search_for_flag(stash_diff)
        if flags:
            print(f"    [+] Flag in stash: {flags}")
            all_flags.extend(flags)
    else:
        print("    No stash entries")

    # ── 7. Check reflog ──
    print("\n[*] Step 7: Checking reflog...")
    reflog_output = run("git reflog --all 2>/dev/null", cwd=repo_dir)
    if reflog_output:
        flags = search_for_flag(reflog_output)
        if flags:
            print(f"    [+] Flag in reflog: {flags}")
            all_flags.extend(flags)

    # ── 8. Check dangling/unreachable objects ──
    print("\n[*] Step 8: Checking dangling objects...")
    fsck_output = run("git fsck --unreachable --no-reflogs 2>/dev/null", cwd=repo_dir)
    if fsck_output:
        for line in fsck_output.split("\n"):
            hash_match = re.search(r"([0-9a-f]{40})", line)
            if hash_match:
                obj_hash = hash_match.group(1)
                content = run(f"git cat-file -p {obj_hash} 2>/dev/null", cwd=repo_dir)
                flags = search_for_flag(content)
                if flags:
                    print(f"    [+] Flag in dangling object {obj_hash}: {flags}")
                    all_flags.extend(flags)

    # Also use lost-found
    run("git fsck --lost-found 2>/dev/null", cwd=repo_dir)
    lost_dir = os.path.join(repo_dir, ".git", "lost-found")
    if os.path.exists(lost_dir):
        for root, dirs, files in os.walk(lost_dir):
            for f in files:
                fpath = os.path.join(root, f)
                try:
                    content = open(fpath).read()
                    flags = search_for_flag(content)
                    if flags:
                        print(f"    [+] Flag in lost-found {f}: {flags}")
                        all_flags.extend(flags)
                except:
                    pass

    # ── 9. Check for-each-ref (all refs including custom) ──
    print("\n[*] Step 9: Checking all refs...")
    refs_output = run("git for-each-ref --format='%(refname) %(objectname)' 2>/dev/null", cwd=repo_dir)
    if refs_output:
        for line in refs_output.split("\n"):
            parts = line.strip().split()
            if len(parts) >= 2:
                ref_name, obj_hash = parts[0], parts[1]
                content = run(f"git cat-file -p {obj_hash} 2>/dev/null", cwd=repo_dir)
                flags = search_for_flag(content)
                if flags:
                    print(f"    [+] Flag in ref {ref_name}: {flags}")
                    all_flags.extend(flags)

    # ── 10. Check .git/config and description ──
    print("\n[*] Step 10: Checking .git metadata files...")
    for meta_file in ["config", "description", "info/refs", "packed-refs", "COMMIT_EDITMSG"]:
        fpath = os.path.join(repo_dir, ".git", meta_file)
        if os.path.exists(fpath):
            try:
                content = open(fpath).read()
                flags = search_for_flag(content)
                if flags:
                    print(f"    [+] Flag in .git/{meta_file}: {flags}")
                    all_flags.extend(flags)
            except:
                pass

    # ── 11. Brute-force: check ALL files in working directory ──
    print("\n[*] Step 11: Searching working directory files...")
    for root, dirs, files in os.walk(repo_dir):
        # Skip .git directory internals (objects, etc.)
        if "/.git/objects" in root or "/.git/hooks" in root:
            continue
        for f in files:
            fpath = os.path.join(root, f)
            try:
                content = open(fpath, errors="replace").read()
                flags = search_for_flag(content)
                if flags:
                    rel_path = os.path.relpath(fpath, repo_dir)
                    print(f"    [+] Flag in {rel_path}: {flags}")
                    all_flags.extend(flags)
            except:
                pass

    return list(set(all_flags))


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 solve.py <git_clone_url_or_repo_path>")
        print("")
        print("Examples:")
        print("  python3 solve.py https://challenge.picoctf.org/repo.git")
        print("  python3 solve.py /path/to/cloned/repo")
        sys.exit(1)

    target = sys.argv[1]

    # Check if target is a URL or existing directory
    if os.path.isdir(target):
        repo_dir = os.path.abspath(target)
        print(f"[*] Using existing repo: {repo_dir}")
    else:
        # Clone from URL
        repo_dir = tempfile.mkdtemp(prefix="my_git_")
        success = clone_repo(target, repo_dir)
        if not success:
            print("[-] Failed to clone repository")
            print("[*] Trying alternative clone methods...")

            # Try HTTP(S) if SSH failed, or vice versa
            alt_url = target.replace("git@", "https://").replace(":", "/", 1)
            if "https://" not in target:
                alt_url = f"https://{target}"
            success = clone_repo(alt_url, repo_dir)

            if not success:
                print("[-] All clone attempts failed")
                sys.exit(1)

    # Enumerate the repo
    print(f"\n{'=' * 60}")
    print(f"Enumerating repository: {repo_dir}")
    print(f"{'=' * 60}")

    flags = enumerate_repo(repo_dir)

    # Results
    print(f"\n{'=' * 60}")
    print("RESULTS")
    print(f"{'=' * 60}")

    if flags:
        for flag in flags:
            print(f"[+] FLAG: {flag}")
    else:
        print("[-] No flags found automatically.")
        print("    Additional manual steps:")
        print("    1. Try interacting with the server directly:")
        print("       git push origin main  (check server response)")
        print("    2. Check for server-side hooks:")
        print("       git push origin HEAD:refs/heads/test")
        print("    3. Try pushing specific content:")
        print("       echo 'flag' > test.txt && git add . && git commit -m 'test' && git push")
        print("    4. Check the challenge description for specific server rules")
        print("    5. Try accessing the server via web browser (may have a web interface)")
        print(f"\n    Repo preserved at: {repo_dir}")


if __name__ == "__main__":
    main()
