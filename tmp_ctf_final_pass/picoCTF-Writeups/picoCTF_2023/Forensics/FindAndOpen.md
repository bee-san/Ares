# FindAndOpen

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
Someone might have hidden the password in the trace file.
Find the key to unlock this file. This tracefile might be good to analyze.

Hints:
1. Download the pcap and look for the password or flag.
2. Don't try to use a password cracking tool, there are easier ways here.
```

Challenge link: [https://play.picoctf.org/practice/challenge/348](https://play.picoctf.org/practice/challenge/348)

## Solution

### Search for a password

Open the PCAP-file in [Wireshark](https://www.wireshark.org/) and browse through the traffic.  
We are looking for a password for the zip-file.

The data in the Ethernet frames contains interesting text for us:

|Protocol|Data|
|----|----|
|0x6865|Flying on Ethernet secret: Is this the flag|
|0x3143|iBwaWNvQ1RGe1Could the flag have been splitted?|
|0x4c4b|AABBHHPJGTFRLKVGhpcyBpcyB0aGUgc2VjcmV0OiBwaWNvQ1RGe1IzNERJTkdfTE9LZF8=|
|0x7361|PBwaWUvQ1RGesabababkjaASKBKSBACVVAVSDDSSSSDSKJBJS|
|0x314d|PBwaWUvQ1RGe1Maybe try checking the other file|

Two of the data portions, those with protocol `0x4c4b` and `0x7361`, like like they could be [base64 encoded](https://en.wikipedia.org/wiki/Base64).

### Decode the data

Let's use tshark to extract only the `data.data` field from these two communications.  
Then we make it unique with `uniq`, convert the data to ascii with `xxd` and then base64 decode

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/FindAndOpen]
└─$ tshark -r dump.pcap -Y 'eth.type == 0x4c4b' -T fields -e data.data | uniq | xxd -r -p | base64 -d             
This is the secret: picoCTF{R34DING_LOKd_ 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/FindAndOpen]
└─$ tshark -r dump.pcap -Y 'eth.type == 0x7361' -T fields -e data.data | uniq | xxd -r -p | base64 -d         
"��base64: invalid input
```

OK, looks like we have a possible password/secret for the zip-file (`picoCTF{R34DING_LOKd_`).

### Get the flag

Finally, we unpack the zip-file and get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/FindAndOpen]
└─$ unzip flag.zip 
Archive:  flag.zip
[flag.zip] flag password: 
 extracting: flag                    

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/FindAndOpen]
└─$ cat flag                   
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [uniq - Linux manual page](https://man7.org/linux/man-pages/man1/uniq.1.html)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
- [Wireshark - Home page](https://www.wireshark.org/)
- [Wireshark - tshark](https://www.wireshark.org/docs/man-pages/tshark.html)
