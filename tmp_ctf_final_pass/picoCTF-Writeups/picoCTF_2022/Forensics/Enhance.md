# Enhance!

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Forensics, svg
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES
 
Description:
Download this image file and find the flag.
 
Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/265](https://play.picoctf.org/practice/challenge/265)

## Solution

Let's try the quick-and-dirty trick to just `grep` for the flag.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep picoCTF drawing.flag.svg 
```

Nope, not that easy...

Studying the contents of the file more closely in a text editor we find that the flag is divided up in several `<tspan>` tags.

```xml
    <text
       xml:space="preserve"
       style="font-style:normal;font-weight:normal;font-size:0.00352781px;line-height:1.25;font-family:sans-serif;letter-spacing:0px;word-spacing:0px;fill:#ffffff;fill-opacity:1;stroke:none;stroke-width:0.26458332;"
       x="107.43014"
       y="132.08501"
       id="text3723"><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.08501"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3748">p </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.08942"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3754">i </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.09383"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3756">c </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.09824"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3758">o </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.10265"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3760">C </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.10706"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3762">T </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.11147"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3764">F { 3 n h 4 n </tspan><tspan
         sodipodi:role="line"
         x="107.43014"
         y="132.11588"
         style="font-size:0.00352781px;line-height:1.25;fill:#ffffff;stroke-width:0.26458332;"
         id="tspan3752">c 3 d _ 2 4 3 7 4 6 7 5 }</tspan></text>
```

We could reconstruct the flag manually but let's not do that.

All lines with flag contents contain the `</tspan>` tag so lets `grep` for that to start with

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep '</tspan>' drawing.flag.svg
         id="tspan3748">p </tspan><tspan
         id="tspan3754">i </tspan><tspan
         id="tspan3756">c </tspan><tspan
         id="tspan3758">o </tspan><tspan
         id="tspan3760">C </tspan><tspan
         id="tspan3762">T </tspan><tspan
         id="tspan3764">F { 3 n h 4 n </tspan><tspan
         id="tspan3752">c 3 d _ 2 4 3 7 4 6 7 5 }</tspan></text>
```

Then let's divide the lines with the `>` character as the delimiter with `cut` and only keep the second field (that is, everything to the right of it)

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep '</tspan>' drawing.flag.svg | cut -d ">" -f2
p </tspan
i </tspan
c </tspan
o </tspan
C </tspan
T </tspan
F { 3 n h 4 n </tspan
c 3 d _ 2 4 3 7 4 6 7 5 }</tspan
```

Do that again with the `<` character and only keep the first field (everything to the left of it)

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep '</tspan>' drawing.flag.svg | cut -d ">" -f2 | cut -d "<" -f1 
p 
i 
c 
o 
C 
T 
F { 3 n h 4 n 
c 3 d _ 2 4 3 7 4 6 7 5 }
```

Now we are close. Then we delete any line breaks with `tr`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep '</tspan>' drawing.flag.svg | cut -d ">" -f2 | cut -d "<" -f1 | tr -d '\r\n' 
p i c o C T F { 3 n h 4 n c 3 d _ 2 4 3 7 4 6 7 5 } 
```

Finally, we delete all the spaces

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Enhance]
└─$ grep '</tspan>' drawing.flag.svg | cut -d ">" -f2 | cut -d "<" -f1 | tr -d '\r\n' | tr -d " "
picoCTF{<REDACTED>}   
```

For additional information, please see the references below.

## References

- [cut - Linux manual page](https://man7.org/linux/man-pages/man1/cut.1.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [SVG - Wikipedia](https://en.wikipedia.org/wiki/SVG)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
