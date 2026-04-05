# Description

This challenge will introduce you to 'Anti-Debugging.' <br> 
Malware developers don't like it when you attempt to <br> 
debug their executable files because debugging these <br> 
files reveals many of their secrets! That's why, they <br> 
include a lot of code logic specifically designed to <br> 
interfere with your debugging process. <br>
Now that you've understood the context, go ahead and <br> 
debug this Windows executable! <br>
This challenge binary file is a Windows console <br>
application and you can start with running it using cmd <br> 
on Windows. <br>
Challenge can be downloaded here. Unzip the archive <br>
with the password picoctf

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-winantidbg0x100).

First, get the file with this command: `wget https://artifacts.picoctf.net/c_titan/84/WinAntiDbg0x100.zip`

Download [x64dbg](https://x64dbg.com/) to debug the exe. Then after unzipping the WinAntiDbg0x100.zip with `picoctf` as the password load the WinAntiDbg0x100.exe file into x96dbg.exe when running. If you run it as 64 bit it will say to run as 32 bit.

It could be manually looked for until seen but it is easier to search for it. With this application to search go to "View" then "Symbol Info" and click the WinAntiDbg0x100.exe file to see the symbols there. From here "debug" or any keyword to find "IsDebuggerPresent" address. Once seen it could be double-clicked to send to that address. Once there a breakpoint should be set on that line so that when you hover over "Breakpoint Enabled" should be seen.

Now the program could be run and once at the "IsDebuggerPresent" function the "EIP" should be seen on the very left and from here "Step Into" is used. After one time the "ret" could be seen in the 2 lines below. Use "step into" until where "ret" redirects to. Now these lines can be seen:

```
00FA1602 | 85C0                     | test eax,eax                            |
00FA1604 | 74 15                    | je winantidbg0x100.FA161B               |
00FA1606 | 68 C835FA00              | push winantidbg0x100.FA35C8             | FA35C8:L"### Oops! The debugger was detected. Try to bypass this check to get the flag!\n"
```

EIP should be on the first line and could be seen on the very left. If the debugger was not detected it goes to `FA161B`. So just a couple of lines down that address could be seen. By right-clicking that line and then using "Set EIP Here" the EIP should now be in the right location. Now by using the "Step over" function a few times, the flag can be seen on a line that looks like this:

`00FA1634 | A1 0854FA00              | mov eax,dword ptr ds:[FA5408]           | 00FA5408:&"picoCTF{d3bug_f0r_th3_Win_0x100_e7...}"`

By right-clicking and going to "copy" and "selection" the flag could be retrieved.

Flag: `picoCTF{d3bug_f0r_th3_Win_0x100_e7...}` 
