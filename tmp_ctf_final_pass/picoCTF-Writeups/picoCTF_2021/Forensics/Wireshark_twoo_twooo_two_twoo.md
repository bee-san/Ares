# Wireshark twoo twooo two twoo...

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
shark2.pcapng.
 
Hints:
1. Did you really find _the_ flag?
2. Look for traffic that seems suspicious.
```

Challenge link: [https://play.picoctf.org/practice/challenge/110](https://play.picoctf.org/practice/challenge/110)

## Solution

### Get an overview of the traffic

We start by getting an overview of the traffic with `Protocol Hierarchy Statistics` using the commandline version of Wireshark - [tshark](https://www.wireshark.org/docs/man-pages/tshark.html)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_twoo_twooo_two_twoo]
└─$ tshark -q -z io,phs -r shark2.pcapng 

===================================================================
Protocol Hierarchy Statistics
Filter: 

eth                                      frames:4831 bytes:3355920
  ip                                     frames:4829 bytes:3355822
    tcp                                  frames:3276 bytes:3120750
      tls                                frames:71 bytes:115780
        tcp.segments                     frames:2 bytes:6576
      http                               frames:802 bytes:1879844
        tcp.segments                     frames:299 bytes:1605841
        mime_multipart                   frames:309 bytes:194144
          tcp.segments                   frames:309 bytes:194144
        data-text-lines                  frames:91 bytes:23987
          tcp.segments                   frames:90 bytes:23696
        xml                              frames:1 bytes:579
    udp                                  frames:1553 bytes:235072
      gquic                              frames:41 bytes:11668
      dns                                frames:1512 bytes:223404
  arp                                    frames:2 bytes:98
===================================================================
```

### Looking for suspicious traffic

Looking for something suspicious as suggested in the hint I first checked the `gquic` traffic but didn't see anything that stood out.

Next, I checked the DNS traffic where I found something that looked like Base64-encoded data at the beginning of the DNS-queries

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_twoo_twooo_two_twoo]
└─$ tshark -r shark2.pcapng -Y dns | head
  791   7.931626 192.168.38.104 → 8.8.8.8      DNS 93 Standard query 0x76aa A lDqoR16q.reddshrimpandherring.com
  792   7.943025      8.8.8.8 → 192.168.38.104 DNS 166 Standard query response 0x76aa No such name A lDqoR16q.reddshrimpandherring.com SOA a.gtld-servers.net
  793   7.947216 192.168.38.104 → 8.8.8.8      DNS 131 Standard query 0xcdd5 A lDqoR16q.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
  794   7.957680      8.8.8.8 → 192.168.38.104 DNS 203 Standard query response 0xcdd5 No such name A lDqoR16q.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com SOA pdns1.ultradns.net
  795   7.958549 192.168.38.104 → 8.8.8.8      DNS 109 Standard query 0x5d2d A lDqoR16q.reddshrimpandherring.com.windomain.local
  796   7.967998      8.8.8.8 → 192.168.38.104 DNS 184 Standard query response 0x5d2d No such name A lDqoR16q.reddshrimpandherring.com.windomain.local SOA a.root-servers.net
  797   7.968981 192.168.38.104 → 8.8.8.8      DNS 93 Standard query 0xc847 A 1Th0dQuT.reddshrimpandherring.com
  798   8.049550      8.8.8.8 → 192.168.38.104 DNS 166 Standard query response 0xc847 No such name A 1Th0dQuT.reddshrimpandherring.com SOA a.gtld-servers.net
  799   8.050527 192.168.38.104 → 8.8.8.8      DNS 131 Standard query 0x21a5 A 1Th0dQuT.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
  800   8.061483      8.8.8.8 → 192.168.38.104 DNS 205 Standard query response 0x21a5 No such name A 1Th0dQuT.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com SOA dns-external-master.amazon.com
```

### Extracting the data

Extracting only the query name field for the traffic to `18.217.1.57` I got

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_twoo_twooo_two_twoo]
└─$ tshark -r shark2.pcapng -Y "dns && ip.dst==18.217.1.57" -T fields -e dns.qry.name                        
cGljb0NU.reddshrimpandherring.com
cGljb0NU.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
cGljb0NU.reddshrimpandherring.com.windomain.local
RntkbnNf.reddshrimpandherring.com
RntkbnNf.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
RntkbnNf.reddshrimpandherring.com.windomain.local
M3hmMWxf.reddshrimpandherring.com
M3hmMWxf.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
M3hmMWxf.reddshrimpandherring.com.windomain.local
ZnR3X2Rl.reddshrimpandherring.com
ZnR3X2Rl.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
ZnR3X2Rl.reddshrimpandherring.com.windomain.local
YWRiZWVm.reddshrimpandherring.com
YWRiZWVm.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
YWRiZWVm.reddshrimpandherring.com.windomain.local
fQ==.reddshrimpandherring.com
fQ==.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
fQ==.reddshrimpandherring.com.windomain.local
fQ==.reddshrimpandherring.com
fQ==.reddshrimpandherring.com.us-west-1.ec2-utilities.amazonaws.com
fQ==.reddshrimpandherring.com.windomain.local
```

Let's extracted only the first part of the queries

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_twoo_twooo_two_twoo]
└─$ tshark -r shark2.pcapng -Y "dns && ip.dst==18.217.1.57" -T fields -e dns.qry.name | cut -d '.' -f1 | uniq
cGljb0NU
RntkbnNf
M3hmMWxf
ZnR3X2Rl
YWRiZWVm
fQ==
```

### Get the flag

Finally, we combine the parts and base64-decode the result to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Wireshark_twoo_twooo_two_twoo]
└─$ tshark -r shark2.pcapng -Y "dns && ip.dst==18.217.1.57" -T fields -e dns.qry.name | cut -d '.' -f1 | uniq | tr -d '\n' | base64 -d
picoCTF{REDACTED}
```

For additional information, please see the references below.

## References

- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [cut - Linux manual page](https://man7.org/linux/man-pages/man1/cut.1.html)
- [Domain Name System - Wikipedia](https://en.wikipedia.org/wiki/Domain_Name_System)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
- [uniq - Linux manual page](https://man7.org/linux/man-pages/man1/uniq.1.html)
- [Wireshark - Homepage](https://www.wireshark.org/)
- [Wireshark - tshark](https://www.wireshark.org/docs/man-pages/tshark.html)
- [Wireshark - Wikipedia](https://en.wikipedia.org/wiki/Wireshark)
