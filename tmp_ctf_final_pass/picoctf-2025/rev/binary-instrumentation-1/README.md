1. Used binwalk and found lzma compressed data. Couldn't decompress it, so figured I would go the normal route.
2. Downloaded frida-trace after uninstalling python on my windows VM :(
3. Attached frida to any "sleep" commands `frida-trace -i "Sleep" -i "SleepEx" .\bininst1.exe`.
4. Edited the "Sleep()" handler which I found was called twice.
5. Got flag.

```javascript
defineHandler({
  onEnter(log, args, state) {
    // Log original sleep call + duration
    log(`Sleep(${args[0]})`);
    
    if (args[0].toInt32() > 1000) {
      args[0] = ptr(1); // Change to 1ms instead of the original long duration
      log('Sleep duration modified to 1ms');
    }
  },
  
  onLeave(log, retval, state) {
  }
})
// picoCTF{w4ke_m3_up_w1th_fr1da_f27acc38}
```