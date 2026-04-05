# rail-fence

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG
 
Description:
A type of transposition cipher is the rail fence cipher, which is described here. 

Here is one such cipher encrypted using the rail fence with 4 rails. Can you decrypt it?

Download the message here.

Put the decoded message in the picoCTF flag format, picoCTF{decoded_message}.

Hints:
1. Once you've understood how the cipher works, it's best to draw it out yourself on paper
```

Challenge link: [https://play.picoctf.org/practice/challenge/289](https://play.picoctf.org/practice/challenge/289)

## Solution

Rather than solving this manually I used [this online service at Planet Calc](https://planetcalc.com/6946/).

It "brute-forces" the number of rails up to a maximum number (with a default of 10).

Enter the given encoded text in the `Encoded message` text box and press 'CALCULATE'.  
And you will get the flag in the output with 4 rails in the `Decode table`.

For additional information, please see the references below.

## References

- [Rail fence cipher - Wikipedia](https://en.wikipedia.org/wiki/Rail_fence_cipher)
- [Transposition cipher - Wikipedia](https://en.wikipedia.org/wiki/Transposition_cipher)
