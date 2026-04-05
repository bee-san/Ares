#!/usr/bin/env python3
"""
Binary Instrumentation 4 - picoCTF 2026
Category: Reverse Engineering | Points: 400

The binary sends the flag over the network. We use Frida to hook Winsock
send functions (send, WSASend, sendto) and intercept the flag data before
it leaves the machine.

Requirements:
    pip install frida frida-tools

Usage:
    1. Place the challenge binary (bininst4.exe) in the current directory.
    2. Run: python3 solve.py

    Alternatively, use the generated Frida script directly:
        frida -f bininst4.exe -l hook_network.js
"""

import subprocess
import sys
import os
import tempfile
import base64
import re

# --- Configuration ---
BINARY_NAME = "bininst4.exe"

# Frida JavaScript hook script that intercepts all common Winsock send functions
FRIDA_SCRIPT = r"""
'use strict';

// Hook send() from WS2_32.dll
// int send(SOCKET s, const char *buf, int len, int flags);
try {
    var sendAddr = Module.getExportByName('WS2_32.dll', 'send');
    Interceptor.attach(sendAddr, {
        onEnter: function(args) {
            var buf = args[1];
            var len = args[2].toInt32();
            if (len > 0) {
                var data = Memory.readUtf8String(buf, len);
                console.log('[send] Length: ' + len);
                console.log('[send] Data (UTF-8): ' + data);
                console.log('[send] Hex dump:');
                console.log(hexdump(buf, { length: len, ansi: true }));

                // Check if data is base64-encoded and try decoding
                try {
                    var raw = Memory.readByteArray(buf, len);
                    send({ type: 'send_data', payload: Array.from(new Uint8Array(raw)) });
                } catch(e) {}
            }
        }
    });
    console.log('[*] Hooked send()');
} catch(e) {
    console.log('[!] Could not hook send(): ' + e);
}

// Hook sendto() from WS2_32.dll
// int sendto(SOCKET s, const char *buf, int len, int flags,
//            const struct sockaddr *to, int tolen);
try {
    var sendtoAddr = Module.getExportByName('WS2_32.dll', 'sendto');
    Interceptor.attach(sendtoAddr, {
        onEnter: function(args) {
            var buf = args[1];
            var len = args[2].toInt32();
            if (len > 0) {
                var data = Memory.readUtf8String(buf, len);
                console.log('[sendto] Length: ' + len);
                console.log('[sendto] Data (UTF-8): ' + data);
                console.log('[sendto] Hex dump:');
                console.log(hexdump(buf, { length: len, ansi: true }));

                try {
                    var raw = Memory.readByteArray(buf, len);
                    send({ type: 'sendto_data', payload: Array.from(new Uint8Array(raw)) });
                } catch(e) {}
            }
        }
    });
    console.log('[*] Hooked sendto()');
} catch(e) {
    console.log('[!] Could not hook sendto(): ' + e);
}

// Hook WSASend() from WS2_32.dll
// int WSASend(SOCKET s, LPWSABUF lpBuffers, DWORD dwBufferCount,
//             LPDWORD lpNumberOfBytesSent, DWORD dwFlags,
//             LPWSAOVERLAPPED lpOverlapped,
//             LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine);
try {
    var wsaSendAddr = Module.getExportByName('WS2_32.dll', 'WSASend');
    Interceptor.attach(wsaSendAddr, {
        onEnter: function(args) {
            var lpBuffers = args[1];
            var dwBufferCount = args[2].toInt32();

            for (var i = 0; i < dwBufferCount; i++) {
                // WSABUF: { ULONG len; CHAR FAR *buf; }
                var offset = i * Process.pointerSize * 2;
                var bufLen = lpBuffers.add(offset).readU32();
                var bufPtr = lpBuffers.add(offset + Process.pointerSize).readPointer();

                if (bufLen > 0) {
                    var data = Memory.readUtf8String(bufPtr, bufLen);
                    console.log('[WSASend] Buffer ' + i + ' Length: ' + bufLen);
                    console.log('[WSASend] Buffer ' + i + ' Data: ' + data);
                    console.log(hexdump(bufPtr, { length: bufLen, ansi: true }));

                    try {
                        var raw = Memory.readByteArray(bufPtr, bufLen);
                        send({ type: 'wsasend_data', payload: Array.from(new Uint8Array(raw)) });
                    } catch(e) {}
                }
            }
        }
    });
    console.log('[*] Hooked WSASend()');
} catch(e) {
    console.log('[!] Could not hook WSASend(): ' + e);
}

// Also hook connect() to see where data is being sent
try {
    var connectAddr = Module.getExportByName('WS2_32.dll', 'connect');
    Interceptor.attach(connectAddr, {
        onEnter: function(args) {
            var sockaddr = args[1];
            var family = sockaddr.readU16();
            if (family === 2) { // AF_INET
                var port = (sockaddr.add(2).readU8() << 8) | sockaddr.add(3).readU8();
                var ip = sockaddr.add(4).readU8() + '.' +
                         sockaddr.add(5).readU8() + '.' +
                         sockaddr.add(6).readU8() + '.' +
                         sockaddr.add(7).readU8();
                console.log('[connect] Connecting to ' + ip + ':' + port);
            }
        }
    });
    console.log('[*] Hooked connect()');
} catch(e) {
    console.log('[!] Could not hook connect(): ' + e);
}

console.log('[*] All hooks installed. Waiting for network activity...');
"""


def write_frida_script():
    """Write the Frida hook script to a file for standalone use."""
    script_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "hook_network.js")
    with open(script_path, "w") as f:
        f.write(FRIDA_SCRIPT)
    print(f"[*] Frida script written to: {script_path}")
    return script_path


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
        return

    captured_data = []

    def on_message(message, data):
        """Callback for messages from the Frida script."""
        if message['type'] == 'send':
            payload = message.get('payload', {})
            if isinstance(payload, dict) and 'payload' in payload:
                raw_bytes = bytes(payload['payload'])
                captured_data.append(raw_bytes)

                # Try to find the flag in the raw data
                text = raw_bytes.decode('utf-8', errors='replace')
                flag_match = re.search(r'picoCTF\{[^}]+\}', text)
                if flag_match:
                    print(f"\n[FLAG FOUND] {flag_match.group(0)}")

                # Try base64 decoding
                try:
                    decoded = base64.b64decode(raw_bytes).decode('utf-8', errors='replace')
                    flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                    if flag_match:
                        print(f"\n[FLAG FOUND - base64 decoded] {flag_match.group(0)}")
                except Exception:
                    pass
        elif message['type'] == 'error':
            print(f"[Frida Error] {message.get('description', message)}")

    print(f"[*] Spawning {binary_path} with Frida instrumentation...")

    # Spawn the process
    device = frida.get_local_device()
    pid = device.spawn([binary_path])
    session = device.attach(pid)

    # Load the instrumentation script
    script = session.create_script(FRIDA_SCRIPT)
    script.on('message', on_message)
    script.load()

    # Resume execution
    device.resume(pid)

    print("[*] Process resumed. Monitoring network sends...")
    print("[*] Press Ctrl+C to stop.\n")

    try:
        import time
        # Wait for the process to finish or user interrupt
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        print("\n[*] Stopping...")
    finally:
        session.detach()

    # Print summary of all captured data
    if captured_data:
        print("\n" + "=" * 60)
        print("CAPTURED NETWORK DATA SUMMARY")
        print("=" * 60)
        for i, data in enumerate(captured_data):
            text = data.decode('utf-8', errors='replace')
            print(f"\n[Capture {i+1}] ({len(data)} bytes): {text}")

            # Check for picoCTF flag pattern
            flag_match = re.search(r'picoCTF\{[^}]+\}', text)
            if flag_match:
                print(f"  >>> FLAG: {flag_match.group(0)}")

            # Try base64
            try:
                decoded = base64.b64decode(data).decode('utf-8', errors='replace')
                flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                if flag_match:
                    print(f"  >>> FLAG (base64): {flag_match.group(0)}")
            except Exception:
                pass
    else:
        print("\n[*] No network data was captured.")
        print("[*] The binary might use a different send mechanism.")
        print("[*] Try: frida-trace -i '*send*' -i '*Send*' -i '*write*' -f " + binary_path)


def main():
    print("=" * 60)
    print("Binary Instrumentation 4 - picoCTF 2026")
    print("Intercept the flag being sent over the network")
    print("=" * 60)

    # Write the standalone Frida JS script regardless
    script_path = write_frida_script()

    # Look for the binary
    binary_path = BINARY_NAME
    if not os.path.exists(binary_path):
        # Try common alternative names
        alternatives = ["bininst4.exe", "binary_instrumentation_4.exe", "challenge.exe"]
        for alt in alternatives:
            if os.path.exists(alt):
                binary_path = alt
                break
        else:
            print(f"\n[!] Binary not found. Place the challenge binary in this directory.")
            print(f"[!] Expected name: {BINARY_NAME}")
            print(f"\n[*] You can still use the Frida script manually:")
            print(f"    frida -f <binary_name> -l {script_path}")
            print(f"\n[*] Or use frida-trace to discover relevant APIs:")
            print(f"    frida-trace -i 'send*' -i 'WSA*' -i 'connect' -f <binary_name> -X WS2_32")
            return

    run_with_frida(binary_path)


if __name__ == "__main__":
    main()
