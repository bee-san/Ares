# KSECRETS - picoCTF 2026

**Category:** General Skills
**Points:** 100

## Challenge Description
We have a kubernetes cluster setup and flag is in the secrets. You think you can get it?

## Approach
This challenge tests basic Kubernetes administration skills. The flag is stored as a Kubernetes Secret, and we need to retrieve and decode it using `kubectl`.

### Kubernetes Secrets Overview

Kubernetes Secrets are objects designed to store sensitive data such as passwords, tokens, and keys. However, by default, Kubernetes Secrets are only base64-encoded -- they are **not encrypted**. Anyone with access to the cluster and appropriate RBAC permissions can read them.

Key facts about Kubernetes Secrets:
- Stored in etcd (the cluster's backing store)
- Base64-encoded by default (NOT encrypted)
- Accessible via the Kubernetes API or `kubectl`
- Can be of type `Opaque`, `kubernetes.io/tls`, `kubernetes.io/dockerconfigjson`, etc.

### kubectl Commands for Secrets

The primary tool for interacting with Kubernetes Secrets is `kubectl`:

1. **List all secrets**: `kubectl get secrets` -- shows all secrets in the current namespace
2. **Describe a secret**: `kubectl describe secret <name>` -- shows metadata (but not the data values)
3. **Get secret data**: `kubectl get secret <name> -o jsonpath='{.data}'` -- shows base64-encoded values
4. **Decode secret data**: The base64 values must be decoded to get the plaintext
5. **Check all namespaces**: `kubectl get secrets --all-namespaces` or `-A` -- the secret may be in a non-default namespace

### Challenge Strategy

1. Connect to the provided Kubernetes environment (likely a webshell or SSH session)
2. Enumerate secrets across all namespaces
3. Retrieve the secret data
4. Base64-decode the values to reveal the flag

## Solution

### Step 1: Check the cluster connection
```bash
kubectl cluster-info
kubectl get nodes
```
Verify you have a working connection to the cluster.

### Step 2: List all secrets
```bash
# List secrets in the default namespace
kubectl get secrets

# List secrets across ALL namespaces
kubectl get secrets -A
# or
kubectl get secrets --all-namespaces
```

### Step 3: Examine the secrets
Look for any secret that seems relevant (e.g., named `flag`, `ctf-secret`, `picoctf`, etc.).

```bash
# Describe the secret to see its structure
kubectl describe secret <secret_name>
kubectl describe secret <secret_name> -n <namespace>
```

### Step 4: Extract the secret data
```bash
# Get the full secret as YAML (shows base64-encoded data)
kubectl get secret <secret_name> -o yaml

# Get the full secret as JSON
kubectl get secret <secret_name> -o json

# Extract a specific key's value
kubectl get secret <secret_name> -o jsonpath='{.data.flag}'
kubectl get secret <secret_name> -o jsonpath='{.data}'
```

### Step 5: Decode the base64 value
```bash
# Decode the flag value
kubectl get secret <secret_name> -o jsonpath='{.data.flag}' | base64 -d

# Or decode all data fields
kubectl get secret <secret_name> -o jsonpath='{.data}' | base64 -d

# If the key name is unknown, get all data:
kubectl get secret <secret_name> -o go-template='{{range $k,$v := .data}}{{printf "%s: " $k}}{{$v | base64decode}}{{"\n"}}{{end}}'
```

### Alternative: One-liner to dump all secrets
If you are unsure which secret contains the flag:
```bash
# Dump all secret values across all namespaces
kubectl get secrets -A -o json | python3 -c "
import json, base64, sys
data = json.load(sys.stdin)
for item in data['items']:
    ns = item['metadata']['namespace']
    name = item['metadata']['name']
    if 'data' in item and item['data']:
        for k, v in item['data'].items():
            decoded = base64.b64decode(v).decode('utf-8', errors='replace')
            if 'picoCTF' in decoded or 'flag' in k.lower():
                print(f'[{ns}/{name}] {k} = {decoded}')
"
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
