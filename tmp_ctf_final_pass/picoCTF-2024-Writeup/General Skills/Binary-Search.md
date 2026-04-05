# Description

Want to play a game? As you use more of the shell, you <br>
might be interested in how they work! Binary search is <br>
a classic algorithm used to quickly find an item in a <br>
sorted list. Can you find the flag? You'll have 1000 <br>
possibilities and only 10 guesses. <br>
Cyber security often has a huge amount of data to look <br>
through - from logs, vulnerability reports, and <br>
forensics. Practicing the fundamentals manually might <br>
help you in the future when you have to write your own <br>
tools!

You can download the challenge files here:
* challenge.zip

`ssh -p 53039 ctf-player@atlas.picoctf.net` <br>
Using the password `6dd28e9b`. Accept the fingerprint <br>
with yes, and ls once connected to begin. Remember, <br>
in a shell, passwords are hidden!

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-binary-search). It also has a dedicated [Binary Search Tree tool](https://picoctfsolutions.com/tools/binary-search-tree) that walks you through the challenge visually.

The `challenege.zip` contains [guessing_game.sh](https://picoctfsolutions.com/picoctf-2024-binary-search/guessing_game.sh) which shows how the game works once you connect via SSH.

SSH into the challenge based on the instructions in the description. 

Welcome to the Binary Search Game! <br>
I'm thinking of a number between 1 and 1000.

The first guess is going to always be 500 because it is the middle of 1 and 1000.

For each guess after 500 the known lower bound is taken and the known higher bound is taken and divided by two for the next guess.

**Example Sequence:**

Enter your guess: 500 <br>
Lower! Try again.

(1 + 500) / 2 = 250.5

Enter your guess: 251 <br>
Lower! Try again.

(1 + 250) / 2 = 125.5

Enter your guess: 125 <br>
Lower! Try again.

(1 + 125) / 2 = 63

Enter your guess: 63 <br>
Lower! Try again.

(1 + 63) / 2 = 32

Enter your guess: 32 <br>
Higher! Try again.

(32 + 63) / 2 = 47.5

Enter your guess: 47 <br>
Lower! Try again.

(32 + 47) / 2 = 39.5

Enter your guess: 39 <br>
Lower! Try again.

(32 + 39) / 2 = 35.5

Enter your guess: 36 <br>
Higher! Try again.

(36 + 39) / 2 = 37.5

Enter your guess: 38 <br>
Congratulations! You guessed the correct number: 38

Flag: `picoCTF{g00d_gu355_de95...}`
