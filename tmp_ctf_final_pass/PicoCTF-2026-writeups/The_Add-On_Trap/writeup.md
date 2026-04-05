# The Add/On Trap - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 200

## Challenge Description

What kind of information can an Add/On reach? Is it possible to exfiltrate them without me noticing? Do they really do what they say?

We are provided with a browser extension (add-on) file to reverse engineer. The challenge hints that the extension may be doing something malicious or hidden beyond its advertised functionality -- possibly exfiltrating data or containing a hidden flag.

## Approach

Browser extensions (also called add-ons) are essentially ZIP archives containing JavaScript, HTML, CSS, and a `manifest.json` configuration file. They can be unpacked and analyzed statically to understand their true behavior.

### Understanding Browser Extension Structure

A typical browser extension contains:

- **`manifest.json`** -- The extension's configuration file. Declares permissions, content scripts, background scripts, popup pages, and other metadata.
- **Background scripts** -- Run persistently in the background; can access privileged Chrome/Firefox APIs.
- **Content scripts** -- Injected into web pages; can read/modify the DOM of visited sites.
- **Popup HTML/JS** -- The UI that appears when clicking the extension icon.
- **Other resources** -- Images, CSS, additional JS libraries.

### Key areas to investigate

1. **Permissions** -- Does the extension request excessive permissions? (`tabs`, `cookies`, `webRequest`, `<all_urls>`, `storage`, `clipboardRead`, etc.)

2. **Content scripts** -- What pages do they inject into? What data do they access from the DOM?

3. **Background scripts** -- Do they send data to external servers? Do they intercept network requests?

4. **Obfuscation** -- Is any JavaScript obfuscated, encoded in base64, or hidden in unusual locations (e.g., inside image files, CSS comments, or metadata)?

5. **Hidden data** -- The flag could be embedded directly in the extension files: hardcoded in JavaScript, hidden in comments, encoded in base64, split across multiple files, or stored in unusual manifest fields.

### Common hiding techniques in CTF extension challenges

- Flag embedded as a base64 string in a JS file
- Flag split across multiple files or variables
- Flag XOR-encoded or otherwise obfuscated in the source
- Flag hidden in the `manifest.json` metadata fields (description, version_name, etc.)
- Flag assembled at runtime by a background or content script
- Flag hidden in a resource file (image EXIF data, CSS comment, etc.)
- Flag exfiltrated via a network request visible in the code (the URL or payload contains the flag)

## Solution

### Step 1: Obtain and extract the extension

Download the provided extension file. It may be a `.crx`, `.xpi`, or `.zip` file. Regardless of extension, it is a ZIP archive:

```bash
# If .crx, strip the CRX header first (or just use unzip which often handles it)
mkdir addon_extracted
unzip addon_file.crx -d addon_extracted/
# OR
unzip addon_file.xpi -d addon_extracted/
# OR
unzip addon_file.zip -d addon_extracted/
```

### Step 2: Examine manifest.json

```bash
cat addon_extracted/manifest.json | python3 -m json.tool
```

Look at:
- `permissions` and `host_permissions` -- what does the extension have access to?
- `content_scripts` -- what scripts are injected and into which pages?
- `background` -- what background scripts or service workers are declared?
- Any unusual fields that might contain encoded data.

### Step 3: Analyze all JavaScript files

Read through each JS file looking for:
- Hardcoded strings (especially base64 or hex-encoded ones)
- `fetch()` or `XMLHttpRequest` calls to external URLs
- DOM manipulation that reads sensitive data
- `chrome.cookies`, `chrome.tabs`, `chrome.storage` API usage
- String concatenation or array-join operations that build a flag
- Comments containing hints or flag fragments

```bash
# Search for obvious flag patterns
grep -r "picoCTF" addon_extracted/
grep -r "flag" addon_extracted/
grep -r "ctf" addon_extracted/ -i

# Search for base64 encoded strings
grep -rE "[A-Za-z0-9+/]{20,}={0,2}" addon_extracted/

# Search for hex encoded strings
grep -rE "\\\\x[0-9a-fA-F]{2}" addon_extracted/
grep -rE "0x[0-9a-fA-F]+" addon_extracted/

# Search for fetch/XHR to external domains
grep -rE "(fetch|XMLHttpRequest|sendBeacon|navigator\.sendBeacon)" addon_extracted/
```

### Step 4: Decode any obfuscated data

If you find base64 strings, hex arrays, or XOR-encoded data, decode them:

```bash
echo "base64stringhere" | base64 -d
```

For more complex obfuscation, use the solve script or a JavaScript console.

### Step 5: Check for data exfiltration patterns

The challenge description asks "Is it possible to exfiltrate them without me noticing?" -- look for:
- Data being sent to external servers via `fetch`, `XMLHttpRequest`, image pixel tracking (`new Image().src = ...`), or WebSocket
- The URL or request body may contain the flag or reveal the flag construction logic

### Step 6: Piece together the flag

The flag may be constructed from multiple pieces. Trace the code execution flow to understand how the flag is assembled and extract it.

## Solution Script

```
python3 solve.py
```

The script extracts the extension archive, parses all files, searches for flag patterns, decodes obfuscated strings, and attempts to reconstruct the flag.

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
