# Dachshund Attacks

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SARA

Description:
What if d is too small? 

Connect with nc mercury.picoctf.net 31133.

Hints:
1. What do you think about my pet? dachshund.jpg
```

Challenge link: [https://play.picoctf.org/practice/challenge/159](https://play.picoctf.org/practice/challenge/159)

## Solution

### Analyse the setup

If you google for `rsa attack small d` you will get [Wiener's attack](https://en.wikipedia.org/wiki/Wiener%27s_attack) as one of the top results. So let's aim for that.

Let's connect to the site with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Dachshund_Attacks]
└─$ nc mercury.picoctf.net 31133
Welcome to my RSA challenge!
e: 65862150127320005037038509012840129209960004410045193759750417377985394915130368181368296052667342899940009485124918084970700806881035884433562195922295472534531712858333290106078343277760803756755670572802680742457324607562251776322670688513709708316127255727360794138423450486733791623208864139985319062709
n: 111635073775282992560436932279422927892718368430046642111054384451437430092958074900936053148330367695269807776075371257916124798239989868928144454138677744106085230203964004299426988568195532162795415192136353551001934000267406108446664822033910616982418163274796786325762783581040178897937780881123797331079
c: 49549647784920592050303228298573648607199952049322535252620035695093329607635490577905002304862913428592143243603580186617698394894233841091360060436848920174536652319109898564887300408227123623221892942454805819870355745462215546218568019618578258052103881221284377372054554879956764482279714539463062518411
```

After a few seconds we get:

- the public key exponent `e`
- the modulus number `n` and
- the cipher text `c`.

### Solve with RsaCtfTool

I used [RsaCtfTool](https://github.com/RsaCtfTool/RsaCtfTool) to solve this, but since I hadn't used it before I checked the arguments

```bash
┌──(kali㉿kali)-[~/Tools/RsaCtfTool]
└─$ source bin/activate

┌──(RsaCtfTool)─(kali㉿kali)-[~/Tools/RsaCtfTool]
└─$ ./RsaCtfTool.py 

__________               _______________________________ __                .__   
\______   \ ___________  \_   ___ \__    ___/\_   _____//  |_  ____   ____ |  |  
 |       _//  ___/\__  \ /    \  \/ |    |    |    __) \   __\/  _ \ /  _ \|  |  
 |    |   \\\___ \  / __ \\     \____|    |    |     \   |  | (  <_> |  <_> )  |__
 |____|_  /____  >(____  /\______  /|____|    \___  /   |__|  \____/ \____/|____/
        \/     \/      \/        \/               \/                             
        

Disclaimer: this tool is meant for educational purposes, for those doing CTF's first try:

Learning the basis of RSA math, undrestand number theory, modular arithmetric, integer factorization, fundamental theorem of arithmetic.
Read the code in this repo to see what and how it does and how to improve it, send PR's.
Avoid copy-paste-run and at last run this tool (knowking the math is more valuable than knowking how to run this tool).


usage: RsaCtfTool.py [-h] [--publickey PUBLICKEY] [--output OUTPUT] [--timeout TIMEOUT] [--createpub] [--dumpkey] [--ext] [--uncipherfile UNCIPHERFILE]
                     [--uncipher UNCIPHER] [--verbosity {CRITICAL,ERROR,WARNING,DEBUG,INFO}] [--private] [--tests] [--ecmdigits ECMDIGITS] [-n N] [-p P] [-q Q]
                     [-e E] [--key KEY] [--password PASSWORD] [--show-factors SHOW_FACTORS]

RSA CTF Tool

options:
  -h, --help            show this help message and exit
  --publickey PUBLICKEY
                        public key file. You can use wildcards for multiple keys.
  --output OUTPUT       output file for results (privates keys, plaintext data).
  --timeout TIMEOUT     Timeout for long attacks in seconds. default is 60s min: MIN_INT in C, max: MAX_INT in C, values < 1 have the same effect as MAX_INT
  --createpub           Take n and e from cli and just print a public key then exit
  --dumpkey             Just dump the RSA variables from a key - n,e,d,p,q
  --ext                 Extended dump of RSA private variables in --dumpkey mode - dp,dq,pinv,qinv).
  --uncipherfile UNCIPHERFILE
                        uncipher a file, using commas to separate multiple paths
  --uncipher UNCIPHER   uncipher a cipher, using commas to separate multiple ciphers
  --verbosity {CRITICAL,ERROR,WARNING,DEBUG,INFO}
                        verbose mode
  --private             Display private key if recovered
  --tests               Run tests on attacks
  --ecmdigits ECMDIGITS
                        Optionally an estimate as to how long one of the primes is for ECM method
  -n N                  Specify the modulus. format : int or 0xhex
  -p P                  Specify the first prime number. format : int or 0xhex
  -q Q                  Specify the second prime number. format : int or 0xhex
  -e E                  Specify the public exponent, using commas to separate multiple exponents. format : int or 0xhex
  --key KEY             Specify the private key file.
  --password PASSWORD   Private key password if needed.
  --show-factors SHOW_FACTORS
                        Show P Q, the factors of N
```

Then we run the wiener attack with the specified data we have from above

```bash
┌──(RsaCtfTool)─(kali㉿kali)-[~/Tools/RsaCtfTool]
└─$ ./RsaCtfTool.py --attack wiener -e 65862150127320005037038509012840129209960004410045193759750417377985394915130368181368296052667342899940009485124918084970700806881035884433562195922295472534531712858333290106078343277760803756755670572802680742457324607562251776322670688513709708316127255727360794138423450486733791623208864139985319062709 -n 111635073775282992560436932279422927892718368430046642111054384451437430092958074900936053148330367695269807776075371257916124798239989868928144454138677744106085230203964004299426988568195532162795415192136353551001934000267406108446664822033910616982418163274796786325762783581040178897937780881123797331079 --uncipher 49549647784920592050303228298573648607199952049322535252620035695093329607635490577905002304862913428592143243603580186617698394894233841091360060436848920174536652319109898564887300408227123623221892942454805819870355745462215546218568019618578258052103881221284377372054554879956764482279714539463062518411
private argument is not set, the private key will not be displayed, even if recovered.
['/tmp/tmpto3q8sm0']

[*] Testing key /tmp/tmpto3q8sm0.
[*] Performing wiener attack on /tmp/tmpto3q8sm0.
 23%|███████████████████████████▏                                                                                           | 142/621 [00:00<00:00, 479928.42it/s]
[*] Attack success with wiener method !

Results for /tmp/tmpto3q8sm0:

Unciphered data :
HEX : 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007069636f4354467b70726f76696e675f7769656e65725f313134363038347d
INT (big endian) : 198614235373674103788888306985643587194108045477674049828293333354607555709
INT (little endian) : 87921226389248609991416580924261379220834209570216127188744368794844983358867834124248414628711792213050850983251956877756570780708855133214828939851701104528276715881992414177645078631779504417424285365191693621622775553702005676528503892037702385734519870361795786506467713703552453301138070119563546066944
utf-8 : picoCTF{proving_<REDACTED>}
utf-16 : 瀀捩䍯䙔灻潲楶杮睟敩敮彲ㄱ㘴㠰紴
STR : b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00picoCTF{proving_<REDACTED>}'
```

And there we have the flag in UTF-8 format.

For additional information, please see the references below.

## References

- [Cryptanalysis of Short RSA Secret Exponents (PDF)](https://monge.univ-mlv.fr/~jyt/Crypto/4/10.1.1.92.5261.pdf)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [RSA (cryptosystem) - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
- [RsaCtfTool - GitHub](https://github.com/RsaCtfTool/RsaCtfTool)
- [The RSA Cryptosystem - Concepts](https://cryptobook.nakov.com/asymmetric-key-ciphers/the-rsa-cryptosystem-concepts)
- [UTF-8 - Wikipedia](https://en.wikipedia.org/wiki/UTF-8)
- [Wiener's attack - Wikipedia](https://en.wikipedia.org/wiki/Wiener%27s_attack)
