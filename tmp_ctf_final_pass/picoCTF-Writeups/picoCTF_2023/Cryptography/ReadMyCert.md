# ReadMyCert

- [Challenge information](#challenge-information)
- [CyberChef solution](#cyberchef-solution)
- [Use an online CSR site solution](#use-an-online-csr-site-solution)
- [Openssl solution](#openssl-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SUNDAY JACOB NWANYIM

Description:
How about we take you on an adventure on exploring certificate signing requests

Take a look at this CSR file here.

Hints:
1. Download the certificate signing request and try to read it.
```

Challenge link: [https://play.picoctf.org/practice/challenge/367](https://play.picoctf.org/practice/challenge/367)

There are several ways to solve this challenge. Below are three of them.

But first let's see how the file looks in a common text editor.

```text
-----BEGIN CERTIFICATE REQUEST-----
MIICpzCCAY8CAQAwPDEmMCQGA1UEAwwdcGljb0NURntyZWFkX215Y2VydF81YWVi
MGQ0Zn0xEjAQBgNVBCkMCWN0ZlBsYXllcjCCASIwDQYJKoZIhvcNAQEBBQADggEP
ADCCAQoCggEBAMCkf11rmV8rgqPvC2ZiPA6W+5RfOTwU6u3WpGvLA+2YFzocBPut
aATTxTPB+uaN2ZN3Z5J2CTFGmPzI4sUQfSqhZGuAqbfMyDDR8pRswmIYVJ6s0Apc
Toi7H8m3IShSbeE0pZUSIJpbK1a7V6lJqgwFMDI1qrgNhGgZaMA/l+d2J0vC3EYd
AijwSs8APcp6woWbFGYwdw5KaBsjn23oVz2G4h3/TmdB5g5e6Oq+kgi38NEpRDS0
ylXo9mUko3FqS4I6y9gOtDEI4uZaCJZuXHDmBpqZ04MfXbIVlHjF9NMOjDvXLonN
650oaANBm4bhBlgid0Fx48Z36tbtAVivZEcCAwEAAaAmMCQGCSqGSIb3DQEJDjEX
MBUwEwYDVR0lBAwwCgYIKwYBBQUHAwIwDQYJKoZIhvcNAQELBQADggEBAHZx6h9r
G/SE7RCoX6ndk5BOJprRiHpxOqPLAWcDyKHfStln0/HcQZzIrRVRsmoHiOmch+md
PBA1b+M5aj+3BWtPR9jOY4vht+ZmHAKa0WfQxwb2dBxsRPKTTDea0wN2u8BHLlSM
PbWPNuz+TKySL41xfwFuM4VN/ywn58GTvdb7HXgwNZCGgo2N1WhRq/dBMiagXMah
yb6gX4erugCu61T5tyD80hgsNBjaqyIdy/whRfC/Pmn3QHmdkqB5ZCPezwb2OLm4
5RDGv3WOB5q0BofoUGhVq757QE8qhL3oTvV2WlLoi3YWaZkJMCeR3vnH92cKC1Ov
FxdQuLOH8GMvl7U=
-----END CERTIFICATE REQUEST-----
```

As hinted by the '=' at the end of the long and cryptic string this is base64 encoded data.

## CyberChef solution

The "quick and dirty" way to get the flag is to use the ['From Base64' recipe in CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true,false)).

Copy the long base64 string but leave out the lines with `BEGIN CERTIFICATE REQUEST` and `END CERTIFICATE REQUEST`.

Most of the output is garbage but you will see the flag in the first line.

## Use an online CSR site solution

A neater way to read the flag is to use a CSR online site such as [digicert's certcentral](https://www.digicert.com/ssltools/view-csr/).

Please note that this time you should copy ALL of the lines into the text field.

The flag is visible in the `Common name` field of the certificate.

## Openssl solution

The "advanced" way is the read the file with `openssl` like this

```text
Z:\CTFs\picoCTF\picoCTF_2023\Cryptography\ReadMyCert>openssl req -in readmycert.csr -noout -text
Certificate Request:
    Data:
        Version: 0 (0x0)
        Subject: CN=picoCTF{<REDACTED>}/name=ctfPlayer
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (2048 bit)
                Modulus:
                    00:c0:a4:7f:5d:6b:99:5f:2b:82:a3:ef:0b:66:62:
                    3c:0e:96:fb:94:5f:39:3c:14:ea:ed:d6:a4:6b:cb:
                    03:ed:98:17:3a:1c:04:fb:ad:68:04:d3:c5:33:c1:
                    fa:e6:8d:d9:93:77:67:92:76:09:31:46:98:fc:c8:
                    e2:c5:10:7d:2a:a1:64:6b:80:a9:b7:cc:c8:30:d1:
                    f2:94:6c:c2:62:18:54:9e:ac:d0:0a:5c:4e:88:bb:
                    1f:c9:b7:21:28:52:6d:e1:34:a5:95:12:20:9a:5b:
<---snip--->
```

The flag is visible (but redacted here) on the fourth line of the output which shows the `CN`, i.e. the `Common Name` field as before.

For additional information, please see the references below.

## References

- [Certificate signing request - Wikipedia](https://en.wikipedia.org/wiki/Certificate_signing_request)
- [openssl - Linux manual page](https://linux.die.net/man/1/openssl)
