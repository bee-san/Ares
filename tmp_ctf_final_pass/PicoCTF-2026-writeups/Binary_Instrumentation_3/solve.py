#!/usr/bin/env python3
"""
Binary Instrumentation 3 - picoCTF 2026
Category: Reverse Engineering | Points: 300

The binary was designed to write the flag to a file but "messed up a few
things on the way."  We use Frida to hook CreateFileA and WriteFile,
inspect their arguments, fix any broken parameters (zeroed byte count,
bad filename, etc.), and extract the flag from the write buffer.

The buffer may be base64-encoded, XOR-obfuscated, or reversed.

Requirements:
    pip install frida frida-tools

Usage:
    1. Place the challenge binary (bininst3.exe) in the current directory.
    2. Run: python3 solve.py

    Alternatively, use the generated Frida script directly:
        frida -f bininst3.exe -l hook_file.js
"""

import subprocess
import sys
import os
import base64
import re

# --- Configuration ---
BINARY_NAME = "bininst3.exe"

# Frida JavaScript hook script that intercepts CreateFileA and WriteFile,
# logs all parameters, fixes broken arguments, and extracts flag data.
FRIDA_SCRIPT = r"""
'use strict';

var capturedBuffers = [];

// -----------------------------------------------------------------------
// Hook CreateFileA to inspect and fix the filename / creation disposition
// HANDLE CreateFileA(
//   LPCSTR                lpFileName,
//   DWORD                 dwDesiredAccess,
//   DWORD                 dwShareMode,
//   LPSECURITY_ATTRIBUTES lpSecurityAttributes,
//   DWORD                 dwCreationDisposition,
//   DWORD                 dwFlagsAndAttributes,
//   HANDLE                hTemplateFile
// );
// -----------------------------------------------------------------------
try {
    var createFileA = Module.getExportByName('KERNEL32.dll', 'CreateFileA');
    Interceptor.attach(createFileA, {
        onEnter: function(args) {
            this.lpFileName = args[0];
            var fileName = args[0].readUtf8String();
            var access = args[1].toInt32();
            var creationDisposition = args[4].toInt32();

            console.log('[CreateFileA] Filename: ' + fileName);
            console.log('[CreateFileA] DesiredAccess: 0x' + (access >>> 0).toString(16));
            console.log('[CreateFileA] CreationDisposition: ' + creationDisposition);

            // Fix: If creation disposition is OPEN_EXISTING (3) instead of
            // CREATE_ALWAYS (2), the call will fail if the file doesn't exist.
            if (creationDisposition === 3) {
                console.log('[CreateFileA] FIX: Changing CreationDisposition from OPEN_EXISTING(3) to CREATE_ALWAYS(2)');
                args[4] = ptr(2);
            }

            // Fix: If filename looks garbled, try to repair it
            if (fileName && (fileName.indexOf('\x00') > 0 || fileName.length === 0)) {
                var fixedName = "flag_output.txt";
                var fixedPtr = Memory.allocUtf8String(fixedName);
                args[0] = fixedPtr;
                console.log('[CreateFileA] FIX: Replaced garbled filename with: ' + fixedName);
            }
        },
        onLeave: function(retval) {
            console.log('[CreateFileA] Returned handle: ' + retval);
            // INVALID_HANDLE_VALUE = -1 (0xFFFFFFFF)
            if (retval.toInt32() === -1) {
                console.log('[CreateFileA] WARNING: File creation FAILED (INVALID_HANDLE_VALUE)');
            }
        }
    });
    console.log('[*] Hooked CreateFileA()');
} catch(e) {
    console.log('[!] Could not hook CreateFileA(): ' + e);
}

// -----------------------------------------------------------------------
// Hook WriteFile to inspect the buffer and fix nNumberOfBytesToWrite
// BOOL WriteFile(
//   HANDLE       hFile,
//   LPCVOID      lpBuffer,
//   DWORD        nNumberOfBytesToWrite,
//   LPDWORD      lpNumberOfBytesWritten,
//   LPOVERLAPPED lpOverlapped
// );
// -----------------------------------------------------------------------
try {
    var writeFile = Module.getExportByName('KERNEL32.dll', 'WriteFile');
    Interceptor.attach(writeFile, {
        onEnter: function(args) {
            var hFile = args[0];
            var lpBuffer = args[1];
            var nBytesToWrite = args[2].toInt32();

            console.log('\n[WriteFile] Handle: ' + hFile);
            console.log('[WriteFile] nNumberOfBytesToWrite: ' + nBytesToWrite);

            // Read a generous chunk regardless of stated byte count
            var readLen = Math.max(nBytesToWrite, 512);
            console.log('[WriteFile] Buffer hex dump (up to ' + readLen + ' bytes):');
            console.log(hexdump(lpBuffer, { length: readLen }));

            // Try to read the buffer as a UTF-8 string
            var bufStr = '';
            try {
                bufStr = Memory.readUtf8String(lpBuffer);
                console.log('[WriteFile] Buffer as string: ' + bufStr);
            } catch(e) {
                console.log('[WriteFile] Could not read buffer as UTF-8');
            }

            // Send the raw buffer data back to Python for analysis
            try {
                var rawBytes = Memory.readByteArray(lpBuffer, readLen);
                send({ type: 'write_data', payload: Array.from(new Uint8Array(rawBytes)),
                       stated_len: nBytesToWrite });
            } catch(e) {}

            // FIX: If nNumberOfBytesToWrite is 0, calculate real length
            if (nBytesToWrite === 0 && bufStr.length > 0) {
                var realLen = bufStr.length;
                console.log('[WriteFile] FIX: Changing nBytesToWrite from 0 to ' + realLen);
                args[2] = ptr(realLen);
            }
        }
    });
    console.log('[*] Hooked WriteFile()');
} catch(e) {
    console.log('[!] Could not hook WriteFile(): ' + e);
}

// -----------------------------------------------------------------------
// Also hook CloseHandle to know when file operations are done
// -----------------------------------------------------------------------
try {
    var closeHandle = Module.getExportByName('KERNEL32.dll', 'CloseHandle');
    Interceptor.attach(closeHandle, {
        onEnter: function(args) {
            console.log('[CloseHandle] Handle: ' + args[0]);
        }
    });
    console.log('[*] Hooked CloseHandle()');
} catch(e) {
    console.log('[!] Could not hook CloseHandle(): ' + e);
}

console.log('[*] All hooks installed. Running binary...');
"""


def write_frida_script():
    """Write the Frida hook script to a file for standalone use."""
    script_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "hook_file.js")
    with open(script_path, "w") as f:
        f.write(FRIDA_SCRIPT)
    print(f"[*] Frida script written to: {script_path}")
    return script_path


def try_decode_flag(raw_bytes):
    """Try various decodings on raw bytes to find the flag."""
    results = []

    # 1. Direct UTF-8
    try:
        text = raw_bytes.decode('utf-8', errors='replace').rstrip('\x00')
        if text.strip():
            results.append(("UTF-8 direct", text))
            match = re.search(r'picoCTF\{[^}]+\}', text)
            if match:
                return match.group(0)
    except Exception:
        pass

    # 2. Base64 decode
    try:
        # Strip null bytes and whitespace
        b64_text = raw_bytes.decode('ascii', errors='ignore').strip().rstrip('\x00')
        decoded = base64.b64decode(b64_text).decode('utf-8', errors='replace')
        results.append(("Base64 decoded", decoded))
        match = re.search(r'picoCTF\{[^}]+\}', decoded)
        if match:
            return match.group(0)
    except Exception:
        pass

    # 3. Reversed string
    try:
        text = raw_bytes.decode('utf-8', errors='replace').rstrip('\x00')
        reversed_text = text[::-1]
        results.append(("Reversed", reversed_text))
        match = re.search(r'picoCTF\{[^}]+\}', reversed_text)
        if match:
            return match.group(0)
    except Exception:
        pass

    # 4. XOR with common single-byte keys
    for key in [0x13, 0x37, 0x42, 0x55, 0xAA, 0xFF]:
        try:
            xored = bytes([b ^ key for b in raw_bytes if b != 0])
            text = xored.decode('utf-8', errors='replace')
            match = re.search(r'picoCTF\{[^}]+\}', text)
            if match:
                results.append((f"XOR 0x{key:02x}", text))
                return match.group(0)
        except Exception:
            pass

    # 5. Base64 decode of reversed string
    try:
        b64_text = raw_bytes.decode('ascii', errors='ignore').strip().rstrip('\x00')[::-1]
        decoded = base64.b64decode(b64_text).decode('utf-8', errors='replace')
        results.append(("Reversed then Base64", decoded))
        match = re.search(r'picoCTF\{[^}]+\}', decoded)
        if match:
            return match.group(0)
    except Exception:
        pass

    # Print all attempted decodings for manual inspection
    if results:
        print("\n[*] Attempted decodings:")
        for method, text in results:
            printable = ''.join(c if c.isprintable() else '.' for c in text[:200])
            print(f"    [{method}]: {printable}")

    return None


def run_with_frida(binary_path):
    """Run the binary with Frida instrumentation and capture intercepted data."""
    try:
        import frida
    except ImportError:
        print("[!] Frida Python bindings not installed.")
        print("[!] Install with: pip install frida frida-tools")
        print("[*] Writing standalone Frida JS script instead...")
        script_path = write_frida_script()
        print(f"\n[*] Run manually with:")
        print(f"    frida -f {binary_path} -l {script_path}")
        print(f"\n[*] Or use frida-trace to discover relevant APIs:")
        print(f"    frida-trace -i '*File*' -i '*Write*' -f {binary_path} -X KERNEL32")
        return

    captured_data = []
    flag_found = None

    def on_message(message, data):
        nonlocal flag_found
        if message['type'] == 'send':
            payload = message.get('payload', {})
            if isinstance(payload, dict) and 'payload' in payload:
                raw_bytes = bytes(payload['payload'])
                stated_len = payload.get('stated_len', 0)
                captured_data.append((raw_bytes, stated_len))

                result = try_decode_flag(raw_bytes)
                if result:
                    flag_found = result
                    print(f"\n{'='*60}")
                    print(f"FLAG FOUND: {result}")
                    print(f"{'='*60}")
        elif message['type'] == 'error':
            print(f"[Frida Error] {message.get('description', message)}")

    print(f"[*] Spawning {binary_path} with Frida instrumentation...")

    device = frida.get_local_device()
    pid = device.spawn([binary_path])
    session = device.attach(pid)

    script = session.create_script(FRIDA_SCRIPT)
    script.on('message', on_message)
    script.load()

    device.resume(pid)

    print("[*] Process resumed. Monitoring file operations...")
    print("[*] Press Ctrl+C to stop.\n")

    try:
        import time
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        print("\n[*] Stopping...")
    finally:
        session.detach()

    # Summary
    print("\n" + "=" * 60)
    print("CAPTURE SUMMARY")
    print("=" * 60)

    if flag_found:
        print(f"\n[+] FLAG: {flag_found}")
    elif captured_data:
        print(f"\n[*] Captured {len(captured_data)} WriteFile buffer(s):")
        for i, (raw, stated) in enumerate(captured_data):
            text = raw.decode('utf-8', errors='replace').rstrip('\x00')[:200]
            printable = ''.join(c if c.isprintable() else '.' for c in text)
            print(f"  [{i+1}] stated_len={stated}, actual_data={printable}")
        print("\n[*] Flag not auto-detected. Review the buffer contents above.")
        print("[*] Try CyberChef or manual decoding on the captured data.")
    else:
        print("\n[*] No WriteFile data was captured.")
        print("[*] The binary might use a different write mechanism.")
        print("[*] Try broader tracing:")
        print(f"    frida-trace -i '*' -f {binary_path} -X KERNEL32")


def main():
    print("=" * 60)
    print("Binary Instrumentation 3 - picoCTF 2026")
    print("Trace and fix the broken file write to extract the flag")
    print("=" * 60)

    # Write the standalone Frida JS script
    script_path = write_frida_script()

    # Look for the binary
    binary_path = BINARY_NAME
    if not os.path.exists(binary_path):
        alternatives = [
            "bininst3.exe",
            "binary_instrumentation_3.exe",
            "challenge.exe",
            "bininst3",
        ]
        for alt in alternatives:
            if os.path.exists(alt):
                binary_path = alt
                break
        else:
            print(f"\n[!] Binary not found. Place the challenge binary in this directory.")
            print(f"[!] Expected name: {BINARY_NAME}")
            print(f"\n[*] You can still use the Frida script manually:")
            print(f"    frida -f <binary_name> -l {script_path}")
            print(f"\n[*] Or use frida-trace for initial discovery:")
            print(f"    frida-trace -i '*File*' -i '*Write*' -f <binary_name> -X KERNEL32")
            print(f"\n[*] Quick manual workflow:")
            print(f"    1. frida-trace -i CreateFileA -i WriteFile -f bininst3.exe -X KERNEL32")
            print(f"    2. Edit __handlers__/KERNEL32.dll/WriteFile.js to log buffer contents")
            print(f"    3. Edit handler to fix nNumberOfBytesToWrite if it is 0")
            print(f"    4. Check if buffer data is base64/XOR encoded and decode")
            return

    run_with_frida(binary_path)


if __name__ == "__main__":
    main()
