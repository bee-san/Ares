# droids1

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JASON

Description:
Find the pass, get the flag. Check out this file.

Hints:
1. Try using apktool and an emulator
2. https://ibotpeaches.github.io/Apktool/
3. https://developer.android.com/studio
```

Challenge link: [https://play.picoctf.org/practice/challenge/14](https://play.picoctf.org/practice/challenge/14)

## Solutions

### Identify the SDK-version

First let's check the SDK-version from the `AndroidManifest.xml` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Droids1]
└─$ apktool decode -o one one.apk  
Picked up _JAVA_OPTIONS: -Dawt.useSystemAAFontSettings=on -Dswing.aatext=true
I: Using Apktool 2.7.0-dirty on one.apk
I: Loading resource table...
I: Decoding AndroidManifest.xml with resources...
I: Loading resource table from file: /home/kali/.local/share/apktool/framework/1.apk
I: Regular manifest package...
I: Decoding file-resources...
I: Decoding values */* XMLs...
I: Baksmaling classes.dex...
I: Copying assets and libs...
I: Copying unknown files...
I: Copying original files...

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Droids1]
└─$ cat one/AndroidManifest.xml   
<?xml version="1.0" encoding="utf-8" standalone="no"?><manifest xmlns:android="http://schemas.android.com/apk/res/android" android:compileSdkVersion="29" android:compileSdkVersionCodename="10" package="com.hellocmu.picoctf" platformBuildVersionCode="29" platformBuildVersionName="10">
    <application android:allowBackup="true" android:appComponentFactory="androidx.core.app.CoreComponentFactory" android:debuggable="true" android:icon="@mipmap/ic_launcher" android:label="@string/app_name" android:roundIcon="@mipmap/ic_launcher_round" android:supportsRtl="true" android:theme="@style/AppTheme">
        <activity android:name="com.hellocmu.picoctf.MainActivity">
            <intent-filter>
                <action android:name="android.intent.action.MAIN"/>
                <category android:name="android.intent.category.LAUNCHER"/>
            </intent-filter>
        </activity>
    </application>
</manifest>   
```

We can see from the manifest file that the application uses SDK-version 29.

### Find the password

Now we need to find the password

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Droids1]
└─$ grep -iR password one
one/res/values/public.xml:    <public type="string" name="password" id="0x7f0b002f" />
one/res/values/strings.xml:    <string name="password">opossum</string>
one/smali/androidx/appcompat/widget/AppCompatTextHelper.smali:    instance-of v9, v9, Landroid/text/method/PasswordTransformationMethod;
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:.method public isPassword()Z
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:    invoke-virtual {v0}, Landroid/view/accessibility/AccessibilityNodeInfo;->isPassword()Z
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:.method public setPassword(Z)V
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:    .param p1, "password"    # Z
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:    invoke-virtual {v0, p1}, Landroid/view/accessibility/AccessibilityNodeInfo;->setPassword(Z)V
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:    const-string v2, "; password: "
one/smali/androidx/core/view/accessibility/AccessibilityNodeInfoCompat.smali:    invoke-virtual {p0}, Landroidx/core/view/accessibility/AccessibilityNodeInfoCompat;->isPassword()Z
one/smali/androidx/core/view/accessibility/AccessibilityRecordCompat.smali:.method public isPassword()Z
one/smali/androidx/core/view/accessibility/AccessibilityRecordCompat.smali:    invoke-virtual {v0}, Landroid/view/accessibility/AccessibilityRecord;->isPassword()Z
one/smali/androidx/core/view/accessibility/AccessibilityRecordCompat.smali:.method public setPassword(Z)V
one/smali/androidx/core/view/accessibility/AccessibilityRecordCompat.smali:    .param p1, "isPassword"    # Z
one/smali/androidx/core/view/accessibility/AccessibilityRecordCompat.smali:    invoke-virtual {v0, p1}, Landroid/view/accessibility/AccessibilityRecord;->setPassword(Z)V
one/smali/androidx/core/widget/TextViewCompat.smali:    instance-of v0, v0, Landroid/text/method/PasswordTransformationMethod;
one/smali/androidx/customview/widget/ExploreByTouchHelper.smali:    invoke-virtual {v1}, Landroidx/core/view/accessibility/AccessibilityNodeInfoCompat;->isPassword()Z
one/smali/androidx/customview/widget/ExploreByTouchHelper.smali:    invoke-virtual {v0, v2}, Landroid/view/accessibility/AccessibilityEvent;->setPassword(Z)V
one/smali/com/hellocmu/picoctf/FlagstaffHill.smali:    .local v0, "password":Ljava/lang/String;
one/smali/com/hellocmu/picoctf/R$string.smali:.field public static final password:I = 0x7f0b002f
```

The password `opossum` from the file `one/res/values/strings.xml` looks promising.

### Emulate the application and get the flag

Then open the `one.apk` file in [Android Studio](https://developer.android.com/studio). I selected the `Profile or Debug APK` option.

Now, we run/emulate the application on a virtual device with SDK-version 29. My virtual device was a `Pixel_3_XL_API_29` device.  
Select `Run 'one'` from the `Run`-menu in Android Studio.

The application looks exactly like the [previous challenge](droids0.md) with an input text field and a big button.

Enter the password `opossum` in the text field, click the button and the flag is revealed.

For additional information, please see the references below.

## References

- [Android Studio - Homepage](https://developer.android.com/studio)
- [apk (file format) - Wikipedia](https://en.wikipedia.org/wiki/Apk_(file_format))
- [Apktool - Homepage](https://apktool.org/)
- [Apktool - Kali Tools](https://www.kali.org/tools/apktool/)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
