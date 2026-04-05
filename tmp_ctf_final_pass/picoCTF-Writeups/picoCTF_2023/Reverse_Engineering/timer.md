# timer

- [Challenge information](#challenge-information)
- [Grepping for the flag solution](#grepping-for-the-flag-solution)
- [Decompiling with JADX-GUI solution](#decompiling-with-jadx-gui-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Reverse Engineering, android
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL
 
Description:
You will find the flag after analysing this apk

Download here.
 
Hints:
1. Decompile
2. mobsf or jadx
```

Challenge link: [https://play.picoctf.org/practice/challenge/381](https://play.picoctf.org/practice/challenge/381)

There are several ways to solve this challenge. Here are two solutions presented in increasing difficulty.

## Grepping for the flag solution

APK-files are simply a Zip-file and can be unpacked with a tool such as [7-Zip](https://www.7-zip.org/).  
Unpack the [APK-file](https://en.wikipedia.org/wiki/Apk_(file_format)) and then just use `grep` recursively on all the unpacked files

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Reverse_Engineering\timer\timer>grep -iR picoCTF *
apktool.yml:  versionName: picoCTF{<REDACTED>}
smali_classes3/com/example/timer/BuildConfig.smali:.field public static final VERSION_NAME:Ljava/lang/String; = "picoCTF{<REDACTED>}"
```

As you can see the flag was present in two different files.

## Decompiling with JADX-GUI solution

A more sofisticated solution is to decompile the APK-file with [Jadx-GUI](https://github.com/skylot/jadx) and study the decompiled code.

Since the APK-file contains a lot of files, the fastest way to find the flag is to use the 'Text search' feature.  
It is available both in the Navigation-menu and as a button on the tool bar.

In this case, searching for `picoCTF` just gives you one hit, in `com.example.timer.BuildConfig`

```C
package com.example.timer;

/* loaded from: classes3.dex */
public final class BuildConfig {
    public static final String APPLICATION_ID = "com.example.timer";
    public static final String BUILD_TYPE = "debug";
    public static final boolean DEBUG = Boolean.parseBoolean("true");
    public static final int VERSION_CODE = 1;
    public static final String VERSION_NAME = "picoCTF{<REDACTED>}";
}
```

For additional information, please see the references below.

## References

- [7-Zip - Homepage](https://www.7-zip.org/)
- [apk (file format) - Wikipedia](https://en.wikipedia.org/wiki/Apk_(file_format))
- [Jadx-GUI - GitHub](https://github.com/skylot/jadx)
