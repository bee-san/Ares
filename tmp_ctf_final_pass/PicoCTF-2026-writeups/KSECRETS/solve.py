#!/usr/bin/env python3
"""
KSECRETS - picoCTF 2026
Category: General Skills | Points: 100

Retrieves the flag from Kubernetes secrets by enumerating all secrets
across all namespaces, decoding base64 data, and searching for the flag.

Prerequisites:
    - kubectl must be installed and configured with cluster access
    - The challenge typically provides a webshell or SSH session with
      kubectl pre-configured

Usage:
    python3 solve.py
    python3 solve.py --namespace <specific_namespace>
    python3 solve.py --secret <specific_secret_name>
"""

import subprocess
import sys
import json
import base64
import re
import argparse

# ──────────────────────────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────────────────────────
FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")


def run(cmd):
    """Run a shell command and return stdout."""
    result = subprocess.run(
        cmd, shell=True, capture_output=True, text=True
    )
    return result.stdout.strip(), result.stderr.strip(), result.returncode


def check_kubectl():
    """Verify kubectl is available and connected."""
    print("[*] Checking kubectl connection...")

    stdout, stderr, rc = run("kubectl cluster-info 2>/dev/null")
    if rc != 0:
        # Try without 2>/dev/null to see the error
        stdout, stderr, rc = run("kubectl cluster-info")
        if rc != 0:
            print(f"[-] kubectl not available or not connected: {stderr}")
            print("    Make sure kubectl is installed and configured.")
            print("    If using the challenge webshell, kubectl should be pre-configured.")
            return False

    print(f"[+] Connected to cluster")

    # Show nodes
    stdout, _, _ = run("kubectl get nodes 2>/dev/null")
    if stdout:
        print(f"    Nodes:\n{stdout}")

    return True


def get_namespaces():
    """Get all namespaces in the cluster."""
    print("[*] Enumerating namespaces...")
    stdout, stderr, rc = run("kubectl get namespaces -o jsonpath='{.items[*].metadata.name}'")
    if rc != 0:
        print(f"    Warning: Could not list namespaces: {stderr}")
        return ["default"]

    namespaces = stdout.strip("'").split()
    print(f"    Found namespaces: {namespaces}")
    return namespaces


def get_secrets_in_namespace(namespace):
    """Get all secrets in a specific namespace."""
    stdout, stderr, rc = run(
        f"kubectl get secrets -n {namespace} -o json 2>/dev/null"
    )
    if rc != 0 or not stdout:
        return []

    try:
        data = json.loads(stdout)
        return data.get("items", [])
    except json.JSONDecodeError:
        return []


def get_all_secrets():
    """Get all secrets across all namespaces."""
    print("[*] Fetching all secrets across all namespaces...")
    stdout, stderr, rc = run("kubectl get secrets -A -o json 2>/dev/null")
    if rc == 0 and stdout:
        try:
            data = json.loads(stdout)
            return data.get("items", [])
        except json.JSONDecodeError:
            pass

    # Fallback: enumerate namespace by namespace
    print("    Falling back to per-namespace enumeration...")
    all_secrets = []
    namespaces = get_namespaces()
    for ns in namespaces:
        secrets = get_secrets_in_namespace(ns)
        all_secrets.extend(secrets)
    return all_secrets


def decode_secret_data(secret):
    """Decode all base64-encoded data fields in a secret."""
    decoded = {}
    raw_data = secret.get("data", {})
    if not raw_data:
        return decoded

    for key, value in raw_data.items():
        try:
            decoded_value = base64.b64decode(value).decode("utf-8", errors="replace")
            decoded[key] = decoded_value
        except Exception as e:
            decoded[key] = f"<decode error: {e}>"

    return decoded


def search_secrets(secrets):
    """Search all secrets for the flag."""
    flags_found = []

    print(f"\n[*] Analyzing {len(secrets)} secrets...")
    print("-" * 60)

    for secret in secrets:
        name = secret.get("metadata", {}).get("name", "unknown")
        namespace = secret.get("metadata", {}).get("namespace", "unknown")
        secret_type = secret.get("type", "unknown")

        # Skip service account tokens (usually not interesting)
        if secret_type == "kubernetes.io/service-account-token":
            continue

        print(f"\n[*] Secret: {namespace}/{name} (type: {secret_type})")

        decoded = decode_secret_data(secret)
        if not decoded:
            print("    (no data)")
            continue

        for key, value in decoded.items():
            # Truncate long values for display
            display_val = value[:200] + "..." if len(value) > 200 else value
            print(f"    {key} = {display_val}")

            # Search for flag
            matches = FLAG_PATTERN.findall(value)
            if matches:
                for flag in matches:
                    print(f"\n    [+] FLAG FOUND in {namespace}/{name} [{key}]: {flag}")
                    flags_found.append(flag)

            # Also check if the value looks interesting
            if any(kw in value.lower() for kw in ["flag", "ctf", "pico", "secret"]):
                print(f"    ^ Interesting value detected!")

        # Also check annotations and labels
        annotations = secret.get("metadata", {}).get("annotations", {})
        for k, v in annotations.items():
            m = FLAG_PATTERN.findall(str(v))
            if m:
                print(f"    [+] FLAG in annotation {k}: {m[0]}")
                flags_found.extend(m)

    return flags_found


def try_specific_secret(secret_name, namespace="default"):
    """Try to get a specific secret by name."""
    print(f"[*] Fetching specific secret: {namespace}/{secret_name}")

    # Try to get the secret as JSON
    stdout, stderr, rc = run(
        f"kubectl get secret {secret_name} -n {namespace} -o json 2>/dev/null"
    )
    if rc != 0 or not stdout:
        print(f"    Secret not found in namespace {namespace}")
        return []

    try:
        secret = json.loads(stdout)
        decoded = decode_secret_data(secret)
        flags = []
        for key, value in decoded.items():
            print(f"    {key} = {value}")
            m = FLAG_PATTERN.findall(value)
            flags.extend(m)
        return flags
    except json.JSONDecodeError:
        print(f"    Failed to parse secret JSON")
        return []


def try_common_secret_names():
    """Try to access secrets with common flag-related names."""
    print("\n[*] Trying common secret names...")
    common_names = [
        "flag", "ctf-flag", "picoctf", "secret-flag", "the-flag",
        "challenge-flag", "ctf-secret", "pico-flag", "ksecrets",
        "k-secrets", "my-secret", "app-secret",
    ]

    namespaces = get_namespaces()
    flags = []

    for ns in namespaces:
        for name in common_names:
            stdout, _, rc = run(
                f"kubectl get secret {name} -n {ns} -o json 2>/dev/null"
            )
            if rc == 0 and stdout:
                try:
                    secret = json.loads(stdout)
                    decoded = decode_secret_data(secret)
                    for key, value in decoded.items():
                        print(f"    [{ns}/{name}] {key} = {value}")
                        m = FLAG_PATTERN.findall(value)
                        if m:
                            print(f"    [+] FLAG: {m[0]}")
                            flags.extend(m)
                except json.JSONDecodeError:
                    pass

    return flags


def try_kubectl_raw():
    """Use raw kubectl commands as fallback approaches."""
    print("\n[*] Trying raw kubectl approaches...")
    flags = []

    # Method 1: kubectl get secrets with go-template to decode all
    stdout, _, rc = run(
        "kubectl get secrets -A -o go-template='"
        "{{range .items}}"
        "{{.metadata.namespace}}/{{.metadata.name}}:\\n"
        "{{range $k,$v := .data}}"
        "  {{$k}}: {{$v | base64decode}}\\n"
        "{{end}}"
        "{{end}}' 2>/dev/null"
    )
    if rc == 0 and stdout:
        m = FLAG_PATTERN.findall(stdout)
        if m:
            print(f"[+] Found via go-template: {m}")
            flags.extend(m)

    # Method 2: Check configmaps too (flag might be there)
    stdout, _, rc = run(
        "kubectl get configmaps -A -o json 2>/dev/null"
    )
    if rc == 0 and stdout:
        m = FLAG_PATTERN.findall(stdout)
        if m:
            print(f"[+] Found in configmaps: {m}")
            flags.extend(m)

    # Method 3: Check environment variables in running pods
    stdout, _, rc = run(
        "kubectl get pods -A -o jsonpath='"
        "{range .items[*]}{range .spec.containers[*]}"
        "{range .env[*]}{.name}={.value}{\"\\n\"}"
        "{end}{end}{end}' 2>/dev/null"
    )
    if rc == 0 and stdout:
        m = FLAG_PATTERN.findall(stdout)
        if m:
            print(f"[+] Found in pod env vars: {m}")
            flags.extend(m)

    return flags


def main():
    parser = argparse.ArgumentParser(description="KSECRETS solver - extract flag from Kubernetes secrets")
    parser.add_argument("--namespace", "-n", default=None, help="Specific namespace to search")
    parser.add_argument("--secret", "-s", default=None, help="Specific secret name to check")
    args = parser.parse_args()

    print("=" * 60)
    print("KSECRETS - picoCTF 2026 Solver")
    print("=" * 60)

    # Check kubectl connectivity
    if not check_kubectl():
        print("\n[-] Cannot connect to Kubernetes cluster.")
        print("    Make sure you are running this from the challenge environment.")
        print("\n    Manual steps:")
        print("    1. kubectl get secrets -A")
        print("    2. kubectl get secret <name> -o jsonpath='{.data}'")
        print("    3. echo '<base64_value>' | base64 -d")
        sys.exit(1)

    all_flags = []

    # If a specific secret was requested
    if args.secret:
        ns = args.namespace or "default"
        flags = try_specific_secret(args.secret, ns)
        all_flags.extend(flags)
    else:
        # ── Phase 1: Get all secrets and search them ──
        secrets = get_all_secrets()
        if secrets:
            flags = search_secrets(secrets)
            all_flags.extend(flags)

        # ── Phase 2: Try common secret names ──
        if not all_flags:
            flags = try_common_secret_names()
            all_flags.extend(flags)

        # ── Phase 3: Raw kubectl approaches ──
        if not all_flags:
            flags = try_kubectl_raw()
            all_flags.extend(flags)

    # ── Results ──
    print("\n" + "=" * 60)
    print("RESULTS")
    print("=" * 60)
    unique_flags = list(set(all_flags))
    if unique_flags:
        for flag in unique_flags:
            print(f"[+] FLAG: {flag}")
    else:
        print("[-] No flag found automatically.")
        print()
        print("Manual investigation steps:")
        print("  1. kubectl get secrets -A")
        print("     (List all secrets in all namespaces)")
        print("  2. kubectl get secret <name> -n <namespace> -o yaml")
        print("     (View the secret's base64-encoded data)")
        print("  3. kubectl get secret <name> -o jsonpath='{.data.<key>}' | base64 -d")
        print("     (Decode a specific key)")
        print("  4. Check configmaps too: kubectl get configmaps -A -o yaml")
        print("  5. Check pod env vars: kubectl exec <pod> -- env | grep -i flag")


if __name__ == "__main__":
    main()
