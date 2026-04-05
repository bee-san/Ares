# Binary Instrumentation 2 #
 
## Overview ##

300 points

Category: [Reverse Engineering](../)

Tags: `#reverseengineering #windows #winapi #frida`

## Description ##

I've been learning more Windows API functions to do my bidding. Hmm... I swear this program was supposed to create a file and write the flag directly to the file. Can you try and intercept the file writing function to see what went wrong?
Download the exe here. Unzip the archive with the password picoctf 

## Approach ##

With our new found [frida](https://frida.re/) knowledge from [Binary Instrumentation 1](../Binary%20Instrumentation%201/Binary%20Instrumentation%201.md) and knowing just enough to be dangerous, I was ready to attack this challenge.

Given the description of this challenge, I first generated default handlers for all file related Win32 APIs using, where `-X KERNEL32` limits the instrumentation to this Dynamic Link Library (DLL) or module:

    $ frida-trace -i *File* -f bininst2.exe -X KERNEL32

There was evidence of the `CreateFileA()` API being called by `bininst2.exe`. For reference the `CreateFileA()` API function prototype is:

    HANDLE CreateFileA(
      [in]           LPCSTR                lpFileName,
      [in]           DWORD                 dwDesiredAccess,
      [in]           DWORD                 dwShareMode,
      [in, optional] LPSECURITY_ATTRIBUTES lpSecurityAttributes,
      [in]           DWORD                 dwCreationDisposition,
      [in]           DWORD                 dwFlagsAndAttributes,
      [in, optional] HANDLE                hTemplateFile
    );

Modifying the handler within `__handlers__\KERNEL32.DLL\CreateFileA.js` to instrument key parameters (`lpFileName`) and also inspect the return value (type `HANDLE`).

    defineHandler({
      onEnter(log, args, state) {
        log('CreateFileA() - lpFileName = ', args[0].readCString());
      },

      onLeave(log, retval, state) {
        log('CreateFileA() - retVal = ', retval)
      }
    });

We now see the following when running `bininst2.exe` through `frida-trace` :

               /* TID 0x7270 */
      4891 ms  NtDeviceIoControlFile()
      4891 ms  RtlDosApplyFileIsolationRedirection_Ustr()
      4891 ms  RtlDosApplyFileIsolationRedirection_Ustr()
      4891 ms  RtlDosApplyFileIsolationRedirection_Ustr()
      4891 ms  NtQueryAttributesFile()
      4891 ms  NtQueryAttributesFile()
      4891 ms  NtOpenFile()
      4891 ms  RtlDosApplyFileIsolationRedirection_Ustr()
      4908 ms  GetSystemTimeAsFileTime()
      4908 ms     | GetSystemTimeAsFileTime()
      4908 ms  GetModuleFileNameW()
      4908 ms     | GetModuleFileNameW()
      4908 ms  AreFileApisANSI()
      4908 ms     | AreFileApisANSI()
      4908 ms  CreateFileA() - lpFileName =  <Insert path here>
      4908 ms     | CreateFileA()
      4908 ms     | CreateFileA() - retVal =  0xffffffffffffffff
    Process terminated

It looks like our original programmer forgot a `TODO` here and our file is trying to be created with the filename `"<Insert path here>"` and failing (return value `-1`), given the invalid filename.

Ok, so lets fix that for them and name our the file to be created `"flag.txt"` instead, by modifying the `lpFileName` parameter in our `onEnter()` handler:

    defineHandler({
      onEnter(log, args, state) {
        log('CreateFileA() - lpFileName = ', args[0].readCString());
        // set valid file name
        const buf = Memory.allocUtf8String('flag.txt')
        this.buf = buf;
        args[0] = buf;
      },

      onLeave(log, retval, state) {
        log('CreateFileA() - retVal = ', retval)
      }
    });

Now quite hopeful of a `flag.txt` fully populated with our flag contents, we are bitterly disappointed when the file is now created successfully, however contains zero data. The output from `frida-trace` confirms the `WriteFile()` Win32 API is being called, so we need to add some further instrumentation to the handler for this API call to see what is going wrong.

    BOOL WriteFile(
      [in]                HANDLE       hFile,
      [in]                LPCVOID      lpBuffer,
      [in]                DWORD        nNumberOfBytesToWrite,
      [out, optional]     LPDWORD      lpNumberOfBytesWritten,
      [in, out, optional] LPOVERLAPPED lpOverlapped
    );

Adding log entries for `hFile` (to verify this mataches our open file `HANDLE` from `CreateFileA()` and `nNumberOfBytesToWrite` in the handler `__handlers__\KERNEL32.DLL\WriteFile.js`:

    defineHandler({
      onEnter(log, args, state) {
        log('WriteFile() - hFile = ', args[0]);
        log('WriteFile() - nNumberOfBytesToWrite = ', args[2]);
      },

      onLeave(log, retval, state) {
      }
    });

We find another bug in `bininst2.exe`, the `WriteFile()` call is being requested to write 0 bytes via the `nNumberOfBytesToWrite` parameter.

           /* TID 0x79cc */
    78 ms  CreateFileA() - lpFileName =  <Insert path here>
    78 ms     | CreateFileA()
    78 ms     | CreateFileA() - retVal =  0x26c
    78 ms  WriteFile() - hFile =  0x26c
    78 ms  WriteFile() - nNumberOfBytesToWrite =  0x0
    78 ms     | WriteFile()
    Process terminated

Not knowing what this value should be, I chose to inspect the `lpBuffer` to write instead, which as it turns out, contained our flag payload.

## Solution ##

The final `CreateFileA()` handler `__handlers__\KERNEL32.DLL\CreateFileA.js` :

    defineHandler({
      onEnter(log, args, state) {
        log('CreateFileA() - lpFileName = ', args[0].readCString());
        // set valid file name
        const buf = Memory.allocUtf8String('flag.txt')
        this.buf = buf;
        args[0] = buf;
      },

      onLeave(log, retval, state) {
        log('CreateFileA() - retVal = ', retval)
      }
    });

The final `WriteFile()` handler `__handlers__\KERNEL32.DLL\WriteFile.js` :

    defineHandler({
      onEnter(log, args, state) {
        log('WriteFile() - hFile = ', args[0]);
        log('WriteFile() - lpBuffer = ', hexdump(args[1]));
        log('WriteFile() - nNumberOfBytesToWrite = ', args[2]);
      },

      onLeave(log, retval, state) {
      }
    });

Yields the following output when instrumenting with `frida-trace` :

    $ frida-trace -i CreateFileA -i WriteFile -f bininst2.exe -X KERNEL32
    Instrumenting...
    CreateFileA: Loaded handler at "D:\CTF\picoCTF-2025\Binary-Instrumentation-2\bininst2\__handlers__\KERNEL32.DLL\CreateFileA.js"
    WriteFile: Loaded handler at "D:\CTF\picoCTF-2025\Binary-Instrumentation-2\bininst2\__handlers__\KERNEL32.DLL\WriteFile.js"
    Started tracing 2 functions. Web UI available at http://localhost:61906/
               /* TID 0x79cc */
        78 ms  CreateFileA() - lpFileName =  <Insert path here>
        78 ms     | CreateFileA()
        78 ms     | CreateFileA() - retVal =  0x26c
        78 ms  WriteFile() - hFile =  0x26c
        78 ms  WriteFile() - lpBuffer =              0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F  0123456789ABCDEF
    140002270  63 47 6c 6a 62 30 4e 55 52 6e 74 6d 63 6a 46 6b  cGljb0NURntmcjFk
    140002280  59 56 39 6d 4d 48 4a 66 59 6a 46 75 58 32 6c 75  YV9mMHJfYjFuX2lu
    140002290  4e 58 52 79 64 57 30 7a 62 6e 51 30 64 47 6c 76  NXRydW0zbnQ0dGlv
    1400022a0  62 69 46 66 59 6a 49 78 59 57 56 6d 4d 7a 6c 39  biFfYjIxYWVmMzl9
    1400022b0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    1400022c0  40 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00  @...............
    1400022d0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    1400022e0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    1400022f0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    140002300  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    140002310  00 00 00 00 00 00 00 00 00 30 00 40 01 00 00 00  .........0.@....
    140002320  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    140002330  a0 21 00 40 01 00 00 00 b0 21 00 40 01 00 00 00  .!.@.....!.@....
    140002340  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    140002350  00 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
    140002360  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
        78 ms  WriteFile() - nNumberOfBytesToWrite =  0x0
        78 ms     | WriteFile()
    Process terminated

As we can see a base64 encoded string is within the `lpBuffer` to write. At this point we can either further modify the `WriteFile()` handler to fix `nNumberOfBytesToWrite`, or just copy, paste and tidy up the base64 encoded string from this output and decode it - which is what I did.

    cGljb0NURntmcjFkYV9mMHJfYjFuX2luNXRydW0zbnQ0dGlvbiFfYjIxYWVmMzl9

Ran through [base64 Decode and Encode](https://base64.decode.org) to decode the flag:

    picoCTF{...........redacted.............}

Where the actual flag value has been redacted for the purposes of this write up.
