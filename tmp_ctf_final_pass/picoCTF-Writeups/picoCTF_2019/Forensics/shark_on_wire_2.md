# shark on wire 2

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY
 
Description:
We found this packet capture. Recover the flag that was pilfered from the network.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/84](https://play.picoctf.org/practice/challenge/84)

## Solution

Open up the PCAP-file in [Wireshark](https://www.wireshark.org/).

Let's start following the UDP streams and look for the flag as in the [previous challenge](shark_on_wire_1.md).

A number of streams contains what could be partial flag data, for example these:

|Stream number|Contents|
|----|----|
|7|icoCTF{StaT31355e|
|23|_|
|25|}|

But I never made any sense of these.

Also, UDP Stream #32 contains the text `start`. And after that a number of streams (e.g. stream #33 - #52) contains what seems to be a random number of `a`s. Finally, UDP Stream #60 contains the text `end`.

These connections are all between source `10.0.0.66` and destination `10.0.0.1`, except for the `end` connection which has a source of `10.0.0.80`. All connections also have a destination port of `22`.

Lets analyse these connections further

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Shark_on_wire_2]
└─$ tshark -nr capture.pcap  -Y 'udp.dstport == 22'
 1104 991.587437    10.0.0.66 → 10.0.0.1     UDP 60 5000 → 22 Len=5
 1106 993.672341    10.0.0.66 → 10.0.0.1     UDP 60 5112 → 22 Len=5
 1118 1006.227400    10.0.0.66 → 10.0.0.1     UDP 60 5105 → 22 Len=5
 1122 1008.323546    10.0.0.66 → 10.0.0.1     UDP 60 5099 → 22 Len=5
 1124 1010.428768    10.0.0.66 → 10.0.0.1     UDP 60 5111 → 22 Len=5
 1129 1012.535515    10.0.0.66 → 10.0.0.1     UDP 60 5067 → 22 Len=5
 1131 1014.627130    10.0.0.66 → 10.0.0.1     UDP 60 5084 → 22 Len=5
 1133 1016.719657    10.0.0.66 → 10.0.0.1     UDP 60 5070 → 22 Len=5
 1135 1018.807279    10.0.0.66 → 10.0.0.1     UDP 60 5123 → 22 Len=5
 1137 1020.899193    10.0.0.66 → 10.0.0.1     UDP 60 5112 → 22 Len=5
 1139 1022.991480    10.0.0.66 → 10.0.0.1     UDP 60 5049 → 22 Len=5
 1141 1025.083748    10.0.0.66 → 10.0.0.1     UDP 60 5076 → 22 Len=5
 1143 1027.167730    10.0.0.66 → 10.0.0.1     UDP 60 5076 → 22 Len=5
 1145 1029.255106    10.0.0.66 → 10.0.0.1     UDP 60 5102 → 22 Len=5
 1147 1031.334799    10.0.0.66 → 10.0.0.1     UDP 60 5051 → 22 Len=5
 1162 1043.850969    10.0.0.66 → 10.0.0.1     UDP 60 5114 → 22 Len=5
 1164 1045.934960    10.0.0.66 → 10.0.0.1     UDP 60 5051 → 22 Len=5
 1166 1048.019181    10.0.0.66 → 10.0.0.1     UDP 60 5100 → 22 Len=5
 1172 1054.255069    10.0.0.66 → 10.0.0.1     UDP 60 5095 → 22 Len=5
 1178 1060.507360    10.0.0.66 → 10.0.0.1     UDP 60 5100 → 22 Len=5
 1180 1062.619741    10.0.0.66 → 10.0.0.1     UDP 60 5097 → 22 Len=5
 1187 1066.779955    10.0.0.66 → 10.0.0.1     UDP 60 5116 → 22 Len=5
 1189 1068.867478    10.0.0.66 → 10.0.0.1     UDP 60 5097 → 22 Len=5
 1192 1070.959143    10.0.0.66 → 10.0.0.1     UDP 60 5095 → 22 Len=5
 1196 1073.043525    10.0.0.66 → 10.0.0.1     UDP 60 5118 → 22 Len=5
 1199 1075.127069    10.0.0.66 → 10.0.0.1     UDP 60 5049 → 22 Len=5
 1267 1139.786992    10.0.0.66 → 10.0.0.1     UDP 60 5097 → 22 Len=5
 1272 1141.870974    10.0.0.66 → 10.0.0.1     UDP 60 5095 → 22 Len=5
 1274 1143.955404    10.0.0.66 → 10.0.0.1     UDP 60 5115 → 22 Len=5
 1276 1146.043247    10.0.0.66 → 10.0.0.1     UDP 60 5116 → 22 Len=5
 1284 1154.383039    10.0.0.66 → 10.0.0.1     UDP 60 5051 → 22 Len=5
 1286 1156.475039    10.0.0.66 → 10.0.0.1     UDP 60 5103 → 22 Len=5
 1296 1166.882937    10.0.0.66 → 10.0.0.1     UDP 60 5048 → 22 Len=5
 1301 1168.975486    10.0.0.66 → 10.0.0.1     UDP 60 5125 → 22 Len=5
 1303 1171.059146    10.0.0.80 → 10.0.0.1     UDP 60 5000 → 22 Len=3
```

These transmissions could be the flag but how is it encoded?  

The source port seems to be changing and if we subtract `5000` we get

|Source port|Src port - 5000|ASCII char|
|----|----|----|
|5112|112|p|
|5105|105|i|
|5099|99|c|
|5111|111|o|
|5067|67|C|
|5084|84|T|

This looks really good!

Lets write a small Python decoder script with the help of [Scapy](https://pypi.org/project/scapy/)

```python
#!/usr/bin/python

from scapy.all import *

packets = rdpcap('capture.pcap')

flag = ''
for packet in packets:
    if UDP in packet and packet[UDP].dport == 22:
        flag += chr(packet[UDP].sport - 5000)
        
print(flag)
```

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Shark_on_wire_2]
└─$ ./decode.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [pcap - Wikipedia](https://en.wikipedia.org/wiki/Pcap)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Scapy - Docs](https://scapy.readthedocs.io/en/latest/)
- [Scapy - Homepage](https://scapy.net/)
- [Scapy - PyPI](https://pypi.org/project/scapy/)
- [Wireshark - Documentation](https://gitlab.com/wireshark/wireshark/-/wikis/home)
- [Wireshark - Homepage](https://www.wireshark.org/)
- [Wireshark - tshark](https://www.wireshark.org/docs/man-pages/tshark.html)
- [Wireshark - Wikipedia](https://en.wikipedia.org/wiki/Wireshark)
