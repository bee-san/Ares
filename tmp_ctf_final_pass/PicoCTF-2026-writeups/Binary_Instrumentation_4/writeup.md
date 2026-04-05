# Binary Instrumentation 4 - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 400

## Challenge Description
The executable was designed to send the flag to someone. Are you that someone? The binary can be downloaded.

## Approach

This is the fourth challenge in the Binary Instrumentation series, which progressively teaches the use of **Frida** -- a dynamic instrumentation toolkit -- to analyze and modify Windows executables at runtime.

The series progression is:
1. **BI 1**: Hook `Sleep()` to skip a long delay and reveal the flag printed to stdout.
2. **BI 2**: Hook `CreateFileA()`/`WriteFile()` to intercept the flag being written to a file.
3. **BI 3**: Hook network-related functions to intercept data being sent over the network.
4. **BI 4** (this challenge): The binary "sends the flag to someone" -- meaning it transmits the flag over a network socket. We need to intercept that transmission using Frida.

Since the description says the binary was "designed to send the flag to someone," the binary likely:
- Creates a socket connection (possibly using `WSAStartup`, `socket`, `connect`)
- Sends the flag data over the network using a function like `send()`, `sendto()`, or `WSASend()`
- The flag may be encoded (e.g., base64) or encrypted before sending

The approach is to use **frida-trace** or a custom Frida script to hook network send functions (`send`, `sendto`, `WSASend`) in the Windows Socket API (Winsock2/WS2_32.dll) and read the buffer contents before they are transmitted.

## Solution

### Step 1: Identify network-related API calls

Use frida-trace to discover which network functions the binary calls:

```
frida-trace -i "send*" -i "WSA*" -i "connect" -i "socket" -f bininst4.exe -X WS2_32
```

This traces all send-related and socket-related functions from the WS2_32.dll (Windows Sockets library).

### Step 2: Hook the `send()` or `WSASend()` function

Once you identify which send function is being used, create a Frida script to intercept it. The `send()` function signature is:

```c
int send(SOCKET s, const char *buf, int len, int flags);
```

The buffer (second argument) contains the data being sent, which should include the flag.

### Step 3: Read the buffer contents

In the Frida hook's `onEnter` callback, read the buffer argument and print it:

```javascript
// hook.js
Interceptor.attach(Module.getExportByName('WS2_32.dll', 'send'), {
    onEnter: function(args) {
        var buf = args[1];
        var len = args[2].toInt32();
        console.log('[send] Length: ' + len);
        console.log('[send] Data: ' + Memory.readUtf8String(buf, len));
        console.log('[send] Hex: ' + hexdump(buf, { length: len }));
    }
});
```

### Step 4: Run with Frida

```
frida -f bininst4.exe -l hook.js
```

The flag will appear in the intercepted buffer output. If the data is base64-encoded, decode it. If it is sent to a remote server, you may also want to hook `connect()` to see where the data is being sent.

### Alternative: Hook WSASend

If the binary uses the more advanced `WSASend()` instead of `send()`, the approach is similar but the buffer is accessed through a WSABUF structure:

```javascript
Interceptor.attach(Module.getExportByName('WS2_32.dll', 'WSASend'), {
    onEnter: function(args) {
        var lpBuffers = args[1];
        var dwBufferCount = args[2].toInt32();
        // WSABUF struct: { ULONG len; CHAR *buf; }
        var bufLen = lpBuffers.readU32();
        var bufPtr = lpBuffers.add(4).readPointer();
        console.log('[WSASend] Length: ' + bufLen);
        console.log('[WSASend] Data: ' + Memory.readUtf8String(bufPtr, bufLen));
    }
});
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
