1. Set up a websocket connection in burpsuite. The websocket recieves messages with two types of content. "eval somenum" or "mate somenum". Trying to do "mate -50" (i.e. stockfish gets mated in 50 moves) results in it saying "you will never break my spirit". Doing eval -10000000 makes it despair and resign giving you the flag in the process.

```
Huh???? How can I be losing this badly... I resign... here's your flag: picoCTF{c1i3nt_s1d3_w3b_s0ck3t5_1c70436a}
```
