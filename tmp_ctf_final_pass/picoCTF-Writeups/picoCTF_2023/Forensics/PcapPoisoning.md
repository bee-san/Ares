# PcapPoisoning

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Forensics, pcap
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
How about some hide and seek heh?
Download this file and find the flag.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/362](https://play.picoctf.org/practice/challenge/362)

## Solution

Open the PCAP-file in [Wireshark](https://www.wireshark.org/) and let's take the description more or less literally by just assuming the flag are available in plain in the packet capture.

Set a display filter of `tcp.payload contains "picoCTF"` and press Enter.

Ah, only one packet matches and the flag is indeed visible in the ASCII details of the packet.

## References

- [Wireshark - Display Filters](https://wiki.wireshark.org/DisplayFilters)
- [Wireshark - Homepage](https://www.wireshark.org/)
