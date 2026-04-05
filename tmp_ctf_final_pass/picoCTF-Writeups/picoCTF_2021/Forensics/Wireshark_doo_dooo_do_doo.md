# Wireshark doo dooo do doo...

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DYLAN

Description:
Can you find the flag? 
shark1.pcapng.
 
Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/115](https://play.picoctf.org/practice/challenge/115)

## Solution

Open up the PCAP-file in [Wireshark](https://www.wireshark.org/).

On easier challenges it can sometimes be worth searching for the flag in plaintext by entering a display filter of `tcp.payload contains "picoCTF"`. However, in this case it doesn't work. No packets matches.

Let's get an overview of the traffic by selecting `Protocol Hierarchy` from the `Statistics` menu.  
The traffic consists almost exclusively of HTTP-traffic.

Set a display filter of `http`. Then right-click on the first displayed packet (packet #2) and select `Follow ->` and `TCP Stream`.

Here we see the stream content displayed in the same sequence as it appeared on the network. Non-printable characters are replaced by dots. Traffic from the client to the server is colored **red**, while traffic from the server to the client is colored **blue**.

Down in the lower right corner of the window you can step through the streams by increasing or decreasing the stream number.

At stream #5 we find this content

```text
GET / HTTP/1.1
Host: 18.222.37.134
Connection: keep-alive
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.105 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Accept-Encoding: gzip, deflate
Accept-Language: en-US,en;q=0.9

HTTP/1.1 200 OK
Date: Mon, 10 Aug 2020 01:51:45 GMT
Server: Apache/2.4.29 (Ubuntu)
Last-Modified: Fri, 07 Aug 2020 00:45:02 GMT
ETag: "2f-5ac3eea4fcf01"
Accept-Ranges: bytes
Content-Length: 47
Keep-Alive: timeout=5, max=100
Connection: Keep-Alive
Content-Type: text/html

Gur synt vf cvpbPGS{c33xno00_1_f33_h_qrnqorrs}
```

Ah, it looks like a encoded flag at the bottom.

### Decode the flag with CyberChef

To decode the flag we could use an online site such as [CyberChef](https://gchq.github.io/CyberChef/) with the 'ROT13' recipe.  
Enter 'rot' in the `Operations` search bar, then drag and drop `ROT13` to the `Recipe`.  
Copy the encoded flag to the `Input` pane and press `BAKE`.

### Decode the flag with a rot13 tool

Alternatively, we can use a prepackaged `rot13` tool.  
There are at least two sets of packages that contains prepacked `rot13` tools:

- [hxtools](https://manpages.debian.org/testing/hxtools/hxtools.7.en.html)
- [bsdgames](https://wiki.linuxquestions.org/wiki/BSD_games)

Install them with either `sudo apt install hxtools` or `sudo apt install bsdgames`.

The tool from `hxtools` installs as `/usr/bin/rot13` and is a script that invokes the `tr` command more or less as described below.

The tool from `bsdgames` installs as `/usr/games/rot13` and calls the `caesar` tool (which is also included in the package) but with a rotation of 13.

After one of these tools have been installed you can run

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_doo_dooo_do_doo]
└─$ echo "Gur synt vf cvpbPGS{c33xno00_1_f33_h_qrnqorrs}" | rot13  
The flag is picoCTF{<REDACTED>}
```

### Decode the flag with the tr command

Finally, we can use the `tr` tool to do the decoding

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_doo_dooo_do_doo]
└─$ echo "Gur synt vf cvpbPGS{c33xno00_1_f33_h_qrnqorrs}" | tr 'A-Za-z' 'N-ZA-Mn-za-m'
The flag is picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [rot13 - Linux manual page](https://manpages.debian.org/bookworm/bsdgames/rot13.6.en.html)
- [ROT13 - Wikipedia](https://en.wikipedia.org/wiki/ROT13)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
- [Wireshark display filter syntax and reference](https://www.wireshark.org/docs/man-pages/wireshark-filter.html)
- [Wireshark - Following Protocol Streams](https://www.wireshark.org/docs/wsug_html_chunked/ChAdvFollowStreamSection.html)
- [Wireshark - Homepage](https://www.wireshark.org/)
- [Wireshark - Wikipedia](https://en.wikipedia.org/wiki/Wireshark)
