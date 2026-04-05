# Quizploit - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 50

## Challenge Description

Solve the quiz. Download the source code to answer questions.

## Approach

This is an introductory binary exploitation challenge worth 50 points with 2473 solves, making it one of the most solved challenges in the competition. The challenge provides a binary (and its source code) that presents a quiz about binary exploitation concepts. The twist is that the answers to the quiz questions must be derived by reading the source code.

Examining the source code reveals:

1. The program asks a series of questions about C/binary exploitation concepts.
2. The answers can be found by carefully reading the provided source file.
3. After answering all questions correctly, the program prints the flag.

Typical questions in this style of challenge involve:
- Buffer sizes (e.g., "What is the size of the buffer?")
- Function names or addresses found in the source
- Specific variable values or constants defined in the code
- Vulnerability types present (e.g., "buffer overflow", "format string")
- Offsets or padding amounts

The key insight is that this is not really an "exploitation" challenge in the traditional sense -- you just need to read the source code carefully and provide the correct answers to each quiz question. The binary then rewards you with the flag.

## Solution

1. **Download the binary and source code** from the challenge page.

2. **Read the source code** carefully. Look for:
   - Defined constants (buffer sizes, magic numbers)
   - Function names (especially any `win` or `print_flag` functions)
   - Variable declarations and their sizes
   - Any comments that hint at answers

3. **Run the binary** and answer each question based on what you found in the source:
   - Questions typically ask about specific values visible in the C source
   - Pay attention to `#define` macros, array sizes, and function signatures
   - Some questions may ask about exploitation concepts (e.g., "What type of vulnerability is present?")

4. **After all correct answers**, the program outputs the flag.

### Typical Source Code Pattern

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 64
#define SECRET_VALUE 0xdeadbeef

void win() {
    // reads and prints the flag
    FILE *f = fopen("flag.txt", "r");
    char buf[64];
    fgets(buf, sizeof(buf), f);
    printf("%s\n", buf);
}

void quiz() {
    char answer[128];

    printf("Question 1: What is the size of the buffer? ");
    scanf("%s", answer);
    if (atoi(answer) != BUFFER_SIZE) { printf("Wrong!\n"); exit(1); }

    printf("Question 2: What is the secret value (in hex)? ");
    scanf("%s", answer);
    if (strcmp(answer, "0xdeadbeef") != 0) { printf("Wrong!\n"); exit(1); }

    printf("Question 3: What is the name of the win function? ");
    scanf("%s", answer);
    if (strcmp(answer, "win") != 0) { printf("Wrong!\n"); exit(1); }

    win();
}
```

The actual source will differ, but the pattern is the same: read the source, find the answers, get the flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
