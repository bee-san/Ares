# substitution2

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography, Substitution
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG

Description:
It seems that another encrypted message has been intercepted. 
The encryptor seems to have learned their lesson though and now there isn't any punctuation! 
Can you still crack the cipher?

Download the message here.

Hints:
1. Try refining your frequency attack, maybe analyzing groups of letters would improve your results?
```

Challenge link: [https://play.picoctf.org/practice/challenge/309](https://play.picoctf.org/practice/challenge/309)

## Solution

The message we were given looks like this

```text
xidcddhsgxgdqdcjvwxidczdvvdgxjyvsgidrisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpgsptvbrspotlydcfjxcswxjprbgtlydctijvvdpodxidgdtwufdxsxswpgawtbgfcsujcsvlwpglgxdugjruspsgxcjxswpabprjudpxjvgzistijcdqdclbgdabvjprujcmdxjyvdgmsvvgiwzdqdczdydvsdqdxidfcwfdcfbcfwgdwajisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpsgpwxwpvlxwxdjtiqjvbjyvdgmsvvgybxjvgwxwodxgxbrdpxgspxdcdgxdrspjprdhtsxdrjywbxtwufbxdcgtsdptdrdadpgsqdtwufdxsxswpgjcdwaxdpvjywcswbgjaajscgjprtwudrwzpxwcbppspotidtmvsgxgjprdhdtbxspotwpasogtcsfxgwaadpgdwpxidwxidcijprsgidjqsvlawtbgdrwpdhfvwcjxswpjprsufcwqsgjxswpjprwaxdpijgdvdudpxgwafvjlzdydvsdqdjtwufdxsxswpxwbtispowpxidwaadpgsqddvdudpxgwatwufbxdcgdtbcsxlsgxidcdawcdjydxxdcqdistvdawcxdtidqjpodvsguxwgxbrdpxgspjudcstjpisoigtiwwvgabcxidczdydvsdqdxijxjpbprdcgxjprspowawaadpgsqdxdtipsnbdgsgdggdpxsjvawcuwbpxspojpdaadtxsqdrdadpgdjprxijxxidxwwvgjprtwpasobcjxswpawtbgdptwbpxdcdrsprdadpgsqdtwufdxsxswpgrwdgpwxvdjrgxbrdpxgxwmpwzxidscdpduljgdaadtxsqdvljgxdjtispoxiduxwjtxsqdvlxispmvsmdjpjxxjtmdcfstwtxasgjpwaadpgsqdvlwcsdpxdrisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpxijxgddmgxwodpdcjxdspxdcdgxsptwufbxdcgtsdptdjuwpoisoigtiwwvdcgxdjtispoxidudpwboijywbxtwufbxdcgdtbcsxlxwfsnbdxidsctbcswgsxluwxsqjxspoxiduxwdhfvwcdwpxidscwzpjprdpjyvspoxiduxwydxxdcrdadprxidscujtispdgxidavjosgfstwTXA{P6C4U_4P41L515_15_73R10B5_702A03AT}
```

### Initial analysis with quipquip

Let's start by using [quipqiup](https://quipqiup.com/) as in the previous challenges.

Input the entire message in the `Puzzle` text field and press `Solve` (with the default setting).

After a short while, you have what seems to be a solution at the top of the suggested solutions

```text
there exist several other well established high school computer security competitions including cyber patriot and us cyber challenge these competitions focus primarily on systems administration fundamentals which are very useful and marketable skills however we believe the proper purpose of a high school computer security competition is not only to teach valuable skills but also to get students interested in and excited about computer science defensive competitions are often laborious affairs and come down to running checklists and executing config scripts offense on the other hand is heavily focused on exploration and improvisation and often has elements of play we believe a competition touching on the offensive elements of computer security is therefore a better vehicle for tech evangelism to students in american high schools further we believe that an understanding of offensive techniques is essential for mounting an effective defense and that the tools and configuration focus encountered in defensive competitions does not lead students to know their enemy as effectively as teaching them to actively think like an attacker pico c t f is an offensively oriented high school computer security competition that seeks to generate interest in computer science among high schoolers teaching them enough about computer security to pique their curiosity motivating them to explore on their own and enabling them to better defend their machines the flag is pico c t f n r m ny duff c 
```

But wait, the flag isn't complete (`flag is pico c t f n r m ny duff c`). There are no digits or underscores. Sigh...

### Manual analysis started and then aborted

I first started to do manual analysis by adding spaces corresponding to the quipquip solution to the cipher text.
The first words became like this

```text
xidcd dhsgx gdqdcjv wxidc zdvv dgxjyvsgidr isoi gtiwwv twufbxdc gdtbcsxl
```

I then started to compile a key, comparing cipher text words with plain text words.
The first word: "xidcd" decodes to "there".

|Cipher text letter|Plain text letter|
|:----:|:----:|
|c|r|
|d|e|
|i|h|
|x|t|

Adding the second word: "dhsgx" which decodes to "exist".

|Cipher text letter|Plain text letter|
|:----:|:----:|
|c|r|
|d|e|
|h|x|
|i|h|
|s|i|
|x|t|

This quickly became too tedious, so I abandoned the work and decided to write a Python script instead.

### Python script to create the key

The plan was to map the quipquip plain text solution to the cipher text and create a substitution key like the one given in the [substitution0](substitution0.md) challenge.

Here is the result called `create_key.py` (including some debug print statements)

```python
#!/usr/bin/python
# -*- coding: latin-1 -*-

import string
import operator

enc_msg = "xidcddhsgxgdqdcjvwxidczdvvdgxjyvsgidrisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpgsptvbrspotlydcfjxcswxjprbgtlydctijvvdpodxidgdtwufdxsxswpgawtbgfcsujcsvlwpglgxdugjruspsgxcjxswpabprjudpxjvgzistijcdqdclbgdabvjprujcmdxjyvdgmsvvgiwzdqdczdydvsdqdxidfcwfdcfbcfwgdwajisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpsgpwxwpvlxwxdjtiqjvbjyvdgmsvvgybxjvgwxwodxgxbrdpxgspxdcdgxdrspjprdhtsxdrjywbxtwufbxdcgtsdptdrdadpgsqdtwufdxsxswpgjcdwaxdpvjywcswbgjaajscgjprtwudrwzpxwcbppspotidtmvsgxgjprdhdtbxspotwpasogtcsfxgwaadpgdwpxidwxidcijprsgidjqsvlawtbgdrwpdhfvwcjxswpjprsufcwqsgjxswpjprwaxdpijgdvdudpxgwafvjlzdydvsdqdjtwufdxsxswpxwbtispowpxidwaadpgsqddvdudpxgwatwufbxdcgdtbcsxlsgxidcdawcdjydxxdcqdistvdawcxdtidqjpodvsguxwgxbrdpxgspjudcstjpisoigtiwwvgabcxidczdydvsdqdxijxjpbprdcgxjprspowawaadpgsqdxdtipsnbdgsgdggdpxsjvawcuwbpxspojpdaadtxsqdrdadpgdjprxijxxidxwwvgjprtwpasobcjxswpawtbgdptwbpxdcdrsprdadpgsqdtwufdxsxswpgrwdgpwxvdjrgxbrdpxgxwmpwzxidscdpduljgdaadtxsqdvljgxdjtispoxiduxwjtxsqdvlxispmvsmdjpjxxjtmdcfstwtxasgjpwaadpgsqdvlwcsdpxdrisoigtiwwvtwufbxdcgdtbcsxltwufdxsxswpxijxgddmgxwodpdcjxdspxdcdgxsptwufbxdcgtsdptdjuwpoisoigtiwwvdcgxdjtispoxidudpwboijywbxtwufbxdcgdtbcsxlxwfsnbdxidsctbcswgsxluwxsqjxspoxiduxwdhfvwcdwpxidscwzpjprdpjyvspoxiduxwydxxdcrdadprxidscujtispdgxidavjosgfstwTXA{P6C4U_4P41L515_15_73R10B5_702A03AT}"
dec_msg = "there exist several other well established high school computer security competitions including cyber patriot and us cyber challenge these competitions focus primarily on systems administration fundamentals which are very useful and marketable skills however we believe the proper purpose of a high school computer security competition is not only to teach valuable skills but also to get students interested in and excited about computer science defensive competitions are often laborious affairs and come down to running checklists and executing config scripts offense on the other hand is heavily focused on exploration and improvisation and often has elements of play we believe a competition touching on the offensive elements of computer security is therefore a better vehicle for tech evangelism to students in american high schools further we believe that an understanding of offensive techniques is essential for mounting an effective defense and that the tools and configuration focus encountered in defensive competitions does not lead students to know their enemy as effectively as teaching them to actively think like an attacker pico c t f is an offensively oriented high school computer security competition that seeks to generate interest in computer science among high schoolers teaching them enough about computer security to pique their curiosity motivating them to explore on their own and enabling them to better defend their machines the flag is pico"

enc_index = 0
dec_index = 0
dict_key = {}

while dec_index < len(dec_msg):
    d = dec_msg[dec_index]
    e = enc_msg[enc_index]
    if d != " ":
        lookup = dict_key.get(d)
        if lookup:
            if lookup != e:
                print("Error: key mismatch %c should be %c" % (lookup, e))
        else:
            print("Update: %c -> %c" % (d, e))
            dict_key[d] = e
        enc_index += 1
    dec_index += 1
    
key = ''
for c in string.ascii_lowercase:
    lookup = dict_key.get(c)
    if lookup:
        key += lookup.upper()
    else:
        key += '@'
    
print(key)
```

I then made the script executable and run it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Substitution2]
└─$ chmod +x create_key.py 

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Substitution2]
└─$ ./create_key.py  
Update: t -> x
Update: h -> i
Update: e -> d
Update: r -> c
Update: x -> h
Update: i -> s
Update: s -> g
Update: v -> q
Update: a -> j
Update: l -> v
Update: o -> w
Update: w -> z
Update: b -> y
Update: d -> r
Update: g -> o
Update: c -> t
Update: m -> u
Update: p -> f
Update: u -> b
Update: y -> l
Update: n -> p
Update: f -> a
Update: k -> m
Update: q -> n
JYTRDAOIS@MVUPWFNCGXBQZHL@
```

So the key is `JYTRDAOIS@MVUPWFNCGXBQZHL@` where the '@' characters are unknown mappings.

### Python script to get the flag

I then re-used the `solve.py` script from the [substitution0](substitution0.md) challenge.

As cipher text I only used the last part of it containing the flag, like this

```python
#!/usr/bin/python
# -*- coding: latin-1 -*-

import string

encrypted_msg = """dadprxidscujtispdgxidavjosgfstwTXA{P6C4U_4P41L515_15_73R10B5_702A03AT}"""

key = "JYTRDAOIS@MVUPWFNCGXBQZHL@"
alphabet = string.ascii_uppercase

decrypted_msg = ""
for c in encrypted_msg:
    if c.isupper():
        decrypted_msg += alphabet[key.index(c)]
    elif c.islower():
        decrypted_msg += alphabet[key.index(c.upper())].lower()
    else:
        decrypted_msg += c

print(decrypted_msg)
```

Running it gave me the flag.

For additional information, please see the references below.

## References

- [Frequency analysis - Wikipedia](https://en.wikipedia.org/wiki/Frequency_analysis)
- [Letter frequency - Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Quipqiup - A fast and automated cryptogram solver](https://quipqiup.com/)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
