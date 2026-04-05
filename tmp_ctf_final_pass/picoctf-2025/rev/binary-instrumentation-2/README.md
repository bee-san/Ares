1. Started by trying to hook into `CreateFile` handlers (tried a few different ones `CreateFileA` ended up being the one that works).
2. Reading the args, it seemed like it was trying to use '<Insert Path Here>` as a path. I tried changing it to a real path, but no matter what it wouldn't work.
3. Instead made a fake file handler and planned to hook into the `WriteFile` function and directly read the buffer.
4. Reading the args of the `WriteFile` function I noted it was reading 0 bytes. Instead of messing with anything I directly read the buffer in memory.
5. Decoded the base 64 encoded flag.


Note that these are recreations of my original handlers I deleted the `__handlers__` folder for `bininst2.exe` on accident :p
```js
// CreateFileA handlers
{
    onEnter(log, args, state) {
        this.lpFileName = Memory.readUtf8String(args[0]);
        log(`CreateFileA(lpFileName="${this.lpFileName}")`);
    },
    
    onLeave(log, retval, state) {
        // Check if the function failed
        if (retval.equals(-1) || retval.equals(0)) {
            log("CreateFileA failed, creating our own file");
            log(`Fake handle: ${0x1337}`);
            // Replace the return value with our real handle
            retval.replace(0x1337);
        } else {
            log(`CreateFileA returned: ${retval}`);
        }
    }
}

```

```js
// WriteFile handlers
onEnter(log, args, state) {
    log(`WriteFile(hFile=${this.fileHandle}, lpBuffer=${this.buffer}, nNumberOfBytesToWrite=${this.originalSize})`);

    // Change the size from 0 to 64 bytes
    const newSize = 64; // Read 64 bytes instead of 0
    
    // Read what's actually in the buffer, even though program wanted to write 0 bytes
    const bufferContents = Memory.readByteArray(this.buffer, newSize);
    
    log('Buffer contents (64 bytes):');
    log(hexdump(bufferContents, {
        offset: 0,
        length: newSize,
        header: true,
        ansi: true
    }));
    
    // Try to interpret as a string
    const str = Memory.readUtf8String(this.buffer, newSize);
    log('As string: ' + str);
},

onLeave(log, retval, state) {
    log(`WriteFile returned: ${retval}`);
}
```