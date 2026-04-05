# SideChannel

## Challenge

There's something fishy about this PIN-code checker, can you figure out the PIN and get the flag? Download the PIN checker program here [pin_checker](https://artifacts.picoctf.net/c/143/pin_checker). Once you've figured out the PIN (and gotten the checker program to accept it), connect to the master server using `nc saturn.picoctf.net 55824` and provide it the PIN to get your flag.

## Solution

1. Searching for "timing-based side-channel attacks" as mentioned in the hints finds the [Wikipedia page for Timing attack](https://en.wikipedia.org/wiki/Timing_attack) and [this article on medium](https://medium.com/spidernitt/introduction-to-timing-attacks-4e1e8c84b32b). The first part of the medium article is exactly the exploit here.

2. We can use the unix `time` command to measure how long it takes for different pine values to be validated. For example running `time echo 10000000 | ./pin_checker` displays the following:

```
Please enter your 8-digit PIN code:
8
Checking PIN...
Access denied.
echo 10000000  0.00s user 0.00s system 44% cpu 0.001 total
./pin_checker  0.13s user 0.00s system 99% cpu 0.128 total
```

3. So, it took 0.13s to check `10000000`. We can increment the first digit by one and see the execution time. Running `time echo 40000000 | ./pin_checker` shows that it takes 0.25s to execute. So, 4 is the correct first digit. We can continue on to future digits.

4. However, we write a [script.py](script.py) to automate the process. Running the solution [script.py](script.py) finds that the pin is `48390513`.

5. Running `nc saturn.picoctf.net 55824` and putting in the pin code we found prints the flag.

### Flag

`picoCTF{t1m1ng_4tt4ck_9803bd25}`
