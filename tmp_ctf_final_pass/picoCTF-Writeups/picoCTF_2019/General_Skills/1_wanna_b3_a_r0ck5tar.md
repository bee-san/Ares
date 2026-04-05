# 1_wanna_b3_a_r0ck5tar

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX BUSHKIN

Description:
The Rockstar language has changed since this problem was released! 
Use this Wayback Machine URL to use an older version of Rockstar, here.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/82](https://play.picoctf.org/practice/challenge/82)

## Solution

This challenge is a continuation of the [mus1c challenge](mus1c.md).

Let's start by checking the contents of the `lyrics.txt` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/1_wanna_b3_a_r0ck5tar]
└─$ cat lyrics.txt 
Rocknroll is right              
Silence is wrong                
A guitar is a six-string        
Tommy's been down               
Music is a billboard-burning razzmatazz!
Listen to the music             
If the music is a guitar                  
Say "Keep on rocking!"                
Listen to the rhythm
If the rhythm without Music is nothing
Tommy is rockin guitar
Shout Tommy!                    
Music is amazing sensation 
Jamming is awesome presence
Scream Music!                   
Scream Jamming!                 
Tommy is playing rock           
Scream Tommy!       
They are dazzled audiences                  
Shout it!
Rock is electric heaven                     
Scream it!
Tommy is jukebox god            
Say it!                                     
Break it down
Shout "Bring on the rock!"
Else Whisper "That ain't it, Chief"                 
Break it down   
```

If we try to run the program in the [online emulator](https://codewithrockstar.com/online) we get a popup window expecting input. Entering something like `test` doesn't produce any output at all.

Time to study the [language documentation](https://codewithrockstar.com/docs).

Going through the script line by line gave me the following understanding of what happens

```text
Rocknroll is right                                      # Unnecessary,  variable not used later on    
Silence is wrong                                        # Unnecessary,  variable not used later on  
A guitar is a six-string                                # A guitar = 19 (unnecessary, never outputed)
Tommy's been down                                       # Tommy = 44 (unnecessary, never outputed)
Music is a billboard-burning razzmatazz!                # Music = ??? (unnecessary, never outputed)
Listen to the music                                     # Input to music variable (unnecessary)          
If the music is a guitar                                # Comparison (unnecessary)
Say "Keep on rocking!"                                  # Print "Keep on rocking!" (unnecessary)
Listen to the rhythm                                    # Input to rhythm variable (unnecessary)
If the rhythm without Music is nothing                  # Comparison (unnecessary)
Tommy is rockin guitar                                  # Tommy = 66
Shout Tommy!                                            # Output Tommy, i.e. 66
Music is amazing sensation                              # Music = 79
Jamming is awesome presence                             # Jamming = 78
Scream Music!                                           # Output Music, i.e. 79
Scream Jamming!                                         # Output Jamming, i.e. 78
Tommy is playing rock                                   # Tommy = 74
Scream Tommy!                                           # Output Tommy, i.e. 74
They are dazzled audiences                              # Tommy = 79
Shout it!                                               # Output Tommy, i.e. 79
Rock is electric heaven                                 # Rock = 86
Scream it!                                              # Output rock, i.e. 86
Tommy is jukebox god                                    # Tommy = 73
Say it!                                                 # Output Tommy, i.e. 73
Break it down
Shout "Bring on the rock!"
Else Whisper "That ain't it, Chief"                 
Break it down 
```

Then I removed unused code and cleaned up the script to a minimal version that looks likte this

```text
Tommy is rockin guitar
Shout Tommy
Music is amazing sensation
Jamming is awesome presence
Scream Music
Scream Jamming
Tommy is playing rock
Scream Tommy
Cajac is dazzled audiences
Shout Cajac 
Rock is electric heaven
Scream Rock
Tommy is jukebox god
Say Tommy
```

Running this code in the emulator produces this output

```text
66
79
78
74
79
86
73
```

Ah, ASCII-numbers as in the previous challenge.

Let's write a small python script to decode it.

```python
#!/usr/bin/python

ascii = [66, 79, 78, 74, 79, 86, 73]

print(f"picoCTF{{{''.join(map(chr, ascii))}}}")
```

Finally we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/1_wanna_b3_a_r0ck5tar]
└─$ ./decode.py 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Rockstar - Esolang](https://esolangs.org/wiki/Rockstar)
- [Rockstar Online Emulator](https://codewithrockstar.com/online)
