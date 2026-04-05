# Ph4nt0m 1ntrud3r

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Forensics, picoCTF 2025, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: Prince Niyonshuti N.

Description:
A digital ghost has breached my defenses, and my sensitive data has been stolen! 😱💻 
Your mission is to uncover how this phantom intruder infiltrated my system and retrieve 
the hidden flag.

To solve this challenge, you'll need to analyze the provided PCAP file and track down 
the attack method. The attacker has cleverly concealed his moves in well timely manner. 
Dive into the network traffic, apply the right filters and show off your forensic 
prowess and unmask the digital intruder!
Find the PCAP file here Network Traffic PCAP file and try to get the flag.

Hints:
1. Filter your packets to narrow down your search.
2. Attacks were done in timely manner.
3. Time is essential
```

Challenge link: [https://play.picoctf.org/practice/challenge/459](https://play.picoctf.org/practice/challenge/459)

## Solution

### Analyse the file in Wireshark

We start by opening the file in [Wireshark](https://www.wireshark.org/).

Stepping through the packets we can see that the payload of each packet looks to be [Base64-encoded](https://en.wikipedia.org/wiki/Base64).  
Examples of payloads are:

- `Lhvf7II=`
- `/5fqQIg=`
- `fTd1V7s=`
- etc.

We want to extract this field and it is named `tcp.payload`.

We also note that the packets are out-of-order. The hints are emphasizing that time is important which suggests that we also need to extract the time field (`frame.time`)

### Extract the wanted fields and decode

Next, we check use `tshark` (the command-line version of Wireshark) to extract these fields and sort them numerically by

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Forensics/Ph4nt0m_1ntrud3r]
└─$ tshark -r myNetworkTraffic.pcap -T fields -e frame.time -e tcp.payload | sort -n                                           
 ** (tshark:34831) 16:03:48.745300 [WSUtil WARNING] ./wsutil/filter_files.c:242 -- read_filter_list(): '/usr/share/wireshark/cfilters' line 1 doesn't have a quoted filter name.
 ** (tshark:34831) 16:03:48.745357 [WSUtil WARNING] ./wsutil/filter_files.c:242 -- read_filter_list(): '/usr/share/wireshark/cfilters' line 2 doesn't have a quoted filter name.
Mar  6, 2025 04:32:03.852521000 CET     2f3566715149673d
Mar  6, 2025 04:32:03.852984000 CET     665464315637733d
Mar  6, 2025 04:32:03.853226000 CET     3136484147644d3d
Mar  6, 2025 04:32:03.853565000 CET     50547965564d343d
Mar  6, 2025 04:32:03.853827000 CET     62446d4f68516b3d
Mar  6, 2025 04:32:03.854091000 CET     68587a6c786d383d
Mar  6, 2025 04:32:03.854405000 CET     2f42626167676b3d
Mar  6, 2025 04:32:03.854649000 CET     525a4f446159773d
Mar  6, 2025 04:32:03.854897000 CET     474964484356733d
Mar  6, 2025 04:32:03.855125000 CET     4c6876663749493d
Mar  6, 2025 04:32:03.855354000 CET     46476e304354633d
Mar  6, 2025 04:32:03.855578000 CET     394b574c7a46343d
Mar  6, 2025 04:32:03.855801000 CET     2f454c70785a4d3d
Mar  6, 2025 04:32:03.856060000 CET     504b48545348593d
Mar  6, 2025 04:32:03.856290000 CET     663534385866513d
Mar  6, 2025 04:32:03.856513000 CET     63476c6a62304e5552673d3d
Mar  6, 2025 04:32:03.856929000 CET     657a46305833633063773d3d
Mar  6, 2025 04:32:03.857170000 CET     626e52666447673064413d3d
Mar  6, 2025 04:32:03.857492000 CET     587a4d3063336c6664413d3d
Mar  6, 2025 04:32:03.857711000 CET     596d68664e484a665a673d3d
Mar  6, 2025 04:32:03.857944000 CET     4d7a45345a4749794d673d3d
Mar  6, 2025 04:32:03.858161000 CET     66513d3d
```

So far, so good. I'm not sure why the error messages are there, let's remove them going forward.

We continue by extracting only the hex data (which is the Base64-data displayed in hex) with `cut`.  
The fields are TAB-separated as default from `tshark`.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Forensics/Ph4nt0m_1ntrud3r]
└─$ tshark -r myNetworkTraffic.pcap -T fields -e frame.time -e tcp.payload 2> /dev/null | sort -n | cut -f2 -d $'\t'
2f3566715149673d
665464315637733d
3136484147644d3d
50547965564d343d
62446d4f68516b3d
68587a6c786d383d
2f42626167676b3d
525a4f446159773d
474964484356733d
4c6876663749493d
46476e304354633d
394b574c7a46343d
2f454c70785a4d3d
504b48545348593d
663534385866513d
63476c6a62304e5552673d3d
657a46305833633063773d3d
626e52666447673064413d3d
587a4d3063336c6664413d3d
596d68664e484a665a673d3d
4d7a45345a4749794d673d3d
66513d3d
```

We can use `xxd` to convert the hex data to ASCII.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Forensics/Ph4nt0m_1ntrud3r]
└─$ tshark -r myNetworkTraffic.pcap -T fields -e frame.time -e tcp.payload 2> /dev/null | sort -n | cut -f2 -d $'\t' | xxd -r -p
/5fqQIg=fTd1V7s=16HAGdM=PTyeVM4=bDmOhQk=hXzlxm8=/Bbaggk=RZODaYw=GIdHCVs=Lhvf7II=FGn0CTc=9KWLzF4=/ELpxZM=PKHTSHY=f548XfQ=cGljb0NURg==ezF0X3c0cw==bnRfdGg0dA==XzM0c3lfdA==YmhfNHJfZg==MzE4ZGIyMg==fQ==   
```

Now we ought to be close. Let's base64-decode the result

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Forensics/Ph4nt0m_1ntrud3r]
└─$ tshark -r myNetworkTraffic.pcap -T fields -e frame.time -e tcp.payload 2> /dev/null | sort -n | cut -f2 -d $'\t' | xxd -r -p | base64 -d
���@�}7uW�ס��=<�T�l9��  �|��o�ڂ E��i�▒�G        [.��i�  7����^�B�œ<��Hv�<]�picoCTF{<REDACTED>}   
```

There we have some unknown garbage followed by the flag.

### Get the flag

Finally, we can extract only the flag part with `grep`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Forensics/Ph4nt0m_1ntrud3r]
└─$ tshark -r myNetworkTraffic.pcap -T fields -e frame.time -e tcp.payload 2> /dev/null | sort -n | cut -f2 -d $'\t' | xxd -r -p | base64 -d | grep -oE 'picoCTF{.*}'
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [cut - Linux manual page](https://man7.org/linux/man-pages/man1/cut.1.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [pcap - Wikipedia](https://en.wikipedia.org/wiki/Pcap)
- [Wireshark - Homepage](https://www.wireshark.org/)
- [Wireshark - tshark](https://www.wireshark.org/docs/man-pages/tshark.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
