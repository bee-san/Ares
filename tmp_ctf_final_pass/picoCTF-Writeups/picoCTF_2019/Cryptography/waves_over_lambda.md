# waves over lambda

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: INVISIBILITY/DANNY

Description:
We made a lot of substitutions to encrypt this. Can you decrypt it? 

Connect with nc jupiter.challenges.picoctf.org 43522.

Hints:
1. Flag is not in the usual flag format
```

Challenge link: [https://play.picoctf.org/practice/challenge/38](https://play.picoctf.org/practice/challenge/38)

## Solution

First we connect to the server to get the ciphertext

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Waves_over_lambda]
└─$ nc jupiter.challenges.picoctf.org 43522
-------------------------------------------------------------------------------
rqcimfjl yeme dl bqam ptfi - pmewaecrb_dl_r_qkem_tfhgnf_qiphfacmfp
-------------------------------------------------------------------------------
yfkdci yfn lqhe jdhe fj hb ndlxqlft syec dc tqcnqc, d yfn kdldjen jye gmdjdly haleah, fcn hfne lefmry fhqci jye gqqol fcn hfxl dc jye tdgmfmb meifmndci jmfclbtkfcdf; dj yfn ljmaro he jyfj lqhe pqmeocqstenie qp jye rqacjmb rqatn yfmntb pfdt jq yfke lqhe dhxqmjfcre dc neftdci sdjy f cqgtehfc qp jyfj rqacjmb. d pdcn jyfj jye ndljmdrj ye cfhen dl dc jye eujmehe eflj qp jye rqacjmb, valj qc jye gqmneml qp jymee ljfjel, jmfclbtkfcdf, hqtnfkdf fcn gaoqkdcf, dc jye hdnlj qp jye rfmxfjydfc hqacjfdcl; qce qp jye sdtnelj fcn teflj ocqsc xqmjdqcl qp eamqxe. d sfl cqj fgte jq tdiyj qc fcb hfx qm sqmo idkdci jye eufrj tqrftdjb qp jye rfljte nmfratf, fl jyeme fme cq hfxl qp jydl rqacjmb fl bej jq rqhxfme sdjy qam qsc qmncfcre lamkeb hfxl; gaj d pqacn jyfj gdljmdjz, jye xqlj jqsc cfhen gb rqacj nmfratf, dl f pfdmtb sett-ocqsc xtfre. d lyftt ecjem yeme lqhe qp hb cqjel, fl jyeb hfb mepmely hb hehqmb syec d jfto qkem hb jmfketl sdjy hdcf.

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Waves_over_lambda]
└─$ nc jupiter.challenges.picoctf.org 43522 > ciphertext.txt
```

### Subbreaker solution

To break the substitution cipher I used [Subbreaker](https://gitlab.com/guballa/SubstitutionBreaker).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Waves_over_lambda]
└─$ cat ciphertext.txt | ~/python_venvs/subbreaker/bin/subbreaker break
Alphabet: abcdefghijklmnopqrstuvwxyz
Key:      rpbvkwnzycetquaxilosfmgdjh
Fitness: 97.63
Nbr keys tried: 4550
Keys per second: 6502
Execution time (seconds): 0.7
Plaintext:
-------------------------------------------------------------------------------
congrats here is your flag - frequency_<REDACTED>
-------------------------------------------------------------------------------
alexey fyodorovitch karamazov was the third son of fyodor pavlovitch karamazov, a land owner well known in our district in his own day, and still remembered among us owing to his gloomy and tragic death, which happened thirteen years ago, and which i shall describe in its proper place. for the present i will only say that this landownerfor so we used to call him, although he hardly spent a day of his life on his own estatewas a strange type, yet one pretty frequently to be met with, a type abject and vicious and at the same time senseless. but he was one of those senseless persons who are very well capable of looking after their worldly affairs, and, apparently, after nothing else. fyodor pavlovitch, for instance, began with next to nothing; his estate was of the smallest; he ran to dine at other men's tables, and fastened on them as a toady, yet at his death it appeared that he had a hundred thousand roubles in hard cash. at the same time, he was all his life one of the most senseless, fantastical fellows in the whole district. i repeat, it was not stupiditythe majority of these fantastical fellows are shrewd and intelligent enoughbut just senselessness, and a peculiar national form of it.
```

And there we have the flag.

### quipqiup solution

An alternative is to use the [quipqiup](https://www.quipqiup.com/) online service to break the cipher.

Copy and paste the ciphertext in the `Puzzle` text field and click the `Solve`-button.  
The first suggested solution contains the flag.

For additional information, please see the references below.

## References

- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
- [SubstitutionBreaker - GitLab](https://gitlab.com/guballa/SubstitutionBreaker)
- [quipqiup - Homepage](https://www.quipqiup.com/)
