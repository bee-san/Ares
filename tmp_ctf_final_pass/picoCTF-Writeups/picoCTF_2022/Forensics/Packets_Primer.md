# Packets Primer

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Forensics, pcap
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Download the packet capture file and use packet analysis software to find the flag.

Hints:
1. Wireshark, if you can install and use it, is probably the most beginner friendly packet analysis software product.
```

Challenge link: [https://play.picoctf.org/practice/challenge/286](https://play.picoctf.org/practice/challenge/286)

## Solution

Open up the PCAP-file in [Wireshark](https://www.wireshark.org/).

On easier challenges it can sometimes be worth searching for the flag in plaintext by entering a display filter of `tcp.payload contains "picoCTF"`. And it works here too. Packet number 4 matches and contains the flag.

To construct/copy the flag you can either

- See the packet's ASCII-details and construct the flag manually
- Right-click on the 60-bytes of data and select Copy -> ...as Printable Text

For additional information, please see the references below.

## References

- [Wireshark - Homepage](https://www.wireshark.org/)
- [Wireshark display filter syntax and reference](https://www.wireshark.org/docs/man-pages/wireshark-filter.html)
