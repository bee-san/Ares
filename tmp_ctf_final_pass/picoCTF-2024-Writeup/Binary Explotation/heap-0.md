# Description

Are overflows just a stack concern? <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc tethys.picoctf.net 58412

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-heap-0).

Connect with this command: `nc tethys.picoctf.net 58412`

These are the given options:

```
1. Print Heap:          (print the current state of the heap)
2. Write to buffer:     (write to your own personal block of data on the heap)
3. Print safe_var:      (I'll even let you look at my variable on the heap, I'm confident it can't be modified)
4. Print Flag:          (Try to print the flag, good luck)
```

From looking at the source code the amount of 'A' characters needed to be written could be seen to be 32. Also, this could be done from testing. After going with the second option and writing 32 'A' characters, the safe_var will be empty. Any more characters and it will overwrite safe_var. Once this can be seen the 4th option could be used to get the flag.

Flag: `picoCTF{my_first_heap_overflow_0c47...}`
