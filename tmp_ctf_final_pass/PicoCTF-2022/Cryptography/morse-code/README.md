# morse-code

## Challenge

Morse code is well known. Can you decrypt this? Download the file [here](https://artifacts.picoctf.net/c/235/morse_chal.wav). Wrap your answer with picoCTF{}, put underscores in place of pauses, and use all lowercase.

## Solution

Use a website like [this morse code decoder](https://morsecode.world/international/decoder/audio-decoder-adaptive.html). Upload the file. The output was `WH47 H47H 90D W20U9H7`. We replace the spaces with underscores to get the flag.

### Flag

`picoCTF{WH47_H47H_90D_W20U9H7}`
