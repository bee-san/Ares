# Binary Instrumentation 3 - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 300

## Challenge Description
The executable was designed to write the flag but it seems like I messed up few things on the way? Can you find a way to fix or trace it?

## Approach

This is the third challenge in the Binary Instrumentation series, which progressively teaches the use of **Frida** -- a dynamic instrumentation toolkit -- to analyze and modify Windows executables at runtime.

The series progression is:
1. **BI 1** (200 pts): Hook `Sleep()` to skip a long delay, then read the flag printed to stdout.
2. **BI 2** (250 pts): Hook `CreateFileA()`/`WriteFile()` to intercept the flag being written to a file, where `nNumberOfBytesToWrite` was set to 0, so the file was created but empty.
3. **BI 3** (300 pts, this challenge): The binary "was designed to write the flag but messed up a few things." Multiple things are broken in the write path, and we need to both trace and *fix* the problems.
4. **BI 4** (400 pts): Hook network `send()` functions to intercept the flag being transmitted over a socket.

### Key Insight

The description says the executable was *designed to write the flag* but "messed up a few things on the way." This strongly suggests that the binary calls `CreateFileA()` and `WriteFile()` (like BI 2) but with multiple corrupted or incorrect parameters. Unlike BI 2 -- where only the byte count was zeroed out -- this challenge introduces additional breakage that requires us to not only *observe* the API calls but also *fix* them at runtime using Frida's `Interceptor.replace()` or by modifying arguments in `onEnter`.

Typical "messed up" parameters in a file-write scenario include:
- **`nNumberOfBytesToWrite` set to 0** -- the file is created but no data is written (same as BI 2).
- **File path is invalid or garbled** -- `CreateFileA()` receives a bad path so the file handle is `INVALID_HANDLE_VALUE` and `WriteFile()` fails silently.
- **Buffer contents are encoded/obfuscated** -- the data in the `WriteFile()` buffer is XOR-encoded, reversed, or base64-encoded and needs decoding.
- **Incorrect flags on CreateFileA** -- e.g., opening with `OPEN_EXISTING` instead of `CREATE_ALWAYS`, causing failure if the file does not exist.

The approach is:
1. Use `frida-trace` to discover which Win32 file APIs the binary calls.
2. Hook `CreateFileA` to inspect and fix the file path and creation flags.
3. Hook `WriteFile` to inspect the buffer, fix the byte count, and decode the buffer if needed.
4. Extract the flag from the corrected write buffer.

## Solution

### Step 1: Initial trace to discover API calls

```bash
frida-trace -i "*File*" -i "*Write*" -f bininst3.exe -X KERNEL32
```

This instruments all file-related and write-related functions from KERNEL32.dll, revealing which APIs the binary calls.

### Step 2: Hook CreateFileA to inspect and fix the filename

The binary may call `CreateFileA()` with a bad or garbled filename. Inspect and optionally replace it:

```javascript
// In __handlers__/KERNEL32.dll/CreateFileA.js
onEnter(log, args, state) {
    var lpFileName = args[0].readUtf8String();
    log('[CreateFileA] Filename: ' + lpFileName);
    log('[CreateFileA] Access: ' + args[1]);
    log('[CreateFileA] Creation: ' + args[4]);

    // If the filename is garbled, fix it
    // e.g., overwrite with a valid path:
    // var fixedPath = Memory.allocUtf8String("C:\\temp\\flag.txt");
    // args[0] = fixedPath;
}
```

### Step 3: Hook WriteFile to read and fix the buffer

```javascript
// In __handlers__/KERNEL32.dll/WriteFile.js
onEnter(log, args, state) {
    var hFile = args[0];
    var lpBuffer = args[1];
    var nBytesToWrite = args[2].toInt32();

    log('[WriteFile] Handle: ' + hFile);
    log('[WriteFile] nBytesToWrite: ' + nBytesToWrite);

    // Read a generous chunk of the buffer even if nBytesToWrite is 0
    var bufData = Memory.readByteArray(lpBuffer, 256);
    log('[WriteFile] Buffer hex dump:');
    log(hexdump(lpBuffer, { length: 256 }));

    // Try reading as string
    try {
        var bufStr = Memory.readUtf8String(lpBuffer);
        log('[WriteFile] Buffer string: ' + bufStr);
    } catch(e) {}

    // Fix nBytesToWrite if it is 0
    if (nBytesToWrite === 0) {
        // Determine actual length from buffer
        var str = Memory.readUtf8String(lpBuffer);
        var realLen = str.length;
        log('[WriteFile] Fixing nBytesToWrite from 0 to ' + realLen);
        args[2] = ptr(realLen);
    }
}
```

### Step 4: Run with the custom hooks

```bash
frida -f bininst3.exe -l hook.js
```

Or, if using frida-trace with modified handlers:

```bash
frida-trace -i "CreateFileA" -i "WriteFile" -f bininst3.exe -X KERNEL32
```

Then edit the auto-generated handler JS files in `__handlers__/KERNEL32.dll/`.

### Step 5: Extract the flag

The flag will appear in the `WriteFile` buffer. If it is base64-encoded, decode it:

```bash
echo "<base64_string>" | base64 -d
```

If it is XOR-encoded, the solve script handles common XOR keys automatically.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
