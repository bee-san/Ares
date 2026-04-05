# endianness

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, General Skills, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: NANA AMA ATOMBO-SACKEY

Description:
Know of little and big endian?

Source

nc titan.picoctf.net 55547

Hints:
1. You might want to check the ASCII table to first find the hexadecimal representation of 
   characters before finding the endianness.
2. Read more about how endianness here:
   https://levelup.gitconnected.com/little-endian-and-big-endian-74ab6441b2a7
```

Challenge link: [https://play.picoctf.org/practice/challenge/414](https://play.picoctf.org/practice/challenge/414)

## Solution

### Analyse the C source code

We start by chacking the source code. First the `main` function

```c
int main()
{
    char *challenge_word = generate_random_word();
    printf("Word: %s\n", challenge_word);
    fflush(stdout);

    char *little_endian = find_little_endian(challenge_word);
    size_t user_little_endian_size = strlen(little_endian);
    char user_little_endian[user_little_endian_size + 1];
    bool correct_flag = false;

    while (!correct_flag)
    {
        printf("Enter the Little Endian representation: ");
        fflush(stdout);
        scanf("%10s", user_little_endian);
        for (size_t i = 0; i < strlen(user_little_endian); i++)
        {
            user_little_endian[i] = toupper(user_little_endian[i]);
        }

        if (strncmp(user_little_endian, little_endian, user_little_endian_size) == 0)
        {
            printf("Correct Little Endian representation!\n");
            fflush(stdout);
            correct_flag = true;
        }
        else
        {
            printf("Incorrect Little Endian representation. Try again!\n");
            fflush(stdout);
        }
    }

    char *big_endian = find_big_endian(challenge_word);
    size_t user_big_endian_size = strlen(big_endian);
    char user_big_endian[user_big_endian_size + 1];

    bool final_flag = false;
    while (!final_flag)
    {
        printf("Enter the Big Endian representation: ");
        fflush(stdout);
        scanf("%10s", user_big_endian);
        for (size_t i = 0; i < strlen(user_big_endian); i++)
        {
            user_big_endian[i] = toupper(user_big_endian[i]);
        }

        if (strncmp(user_big_endian, big_endian, user_big_endian_size) == 0)
        {
            printf("Correct Big Endian representation!\n");
            fflush(stdout);
            final_flag = true;
        }
        else
        {
            printf("Incorrect Big Endian representation. Try again!\n");
            fflush(stdout);
        }
    }

    FILE *flag = fopen("flag.txt", "r");
    if (flag == NULL)
    {
        printf("Flag not found. Please run this on the server\n");
        fflush(stdout);
        exit(0);
    }

    char flag_content[100];
    fgets(flag_content, sizeof(flag_content), flag);
    printf("Congratulations! You found both endian representations correctly!\n");
    fflush(stdout);
    printf("Your Flag is: %s\n", flag_content);
    fflush(stdout);
    exit(0);

    return 0;
}  
```

We can see that in summaty `main` does the following:

- Generates a random word and prints it to the screen
- Asks for both the little and big [endian](https://en.wikipedia.org/wiki/Endianness) version of the word
- Keeps asking until the correct answers are given
- Prints the flag if both versions are correctly answered

Further more, we see from the verification functions and the `generate_random_word` function that the hexadecimal [ASCII](https://en.wikipedia.org/wiki/ASCII) versions of the characters are expected since:

- The use of `%02X` as the format specifier in the `snprintf`-functions
- The integer values in the `word` array where `'a'` is added

```c
char *find_little_endian(const char *word)
{
    size_t word_len = strlen(word);
    char *little_endian = (char *)malloc((2 * word_len + 1) * sizeof(char));

    for (size_t i = word_len; i-- > 0;)
    {
        snprintf(&little_endian[(word_len - 1 - i) * 2], 3, "%02X", (unsigned char)word[i]);
    }

    little_endian[2 * word_len] = '\0';
    return little_endian;
}

char *find_big_endian(const char *word)
{
    size_t length = strlen(word);
    char *big_endian = (char *)malloc((2 * length + 1) * sizeof(char));

    for (size_t i = 0; i < length; i++)
    {
        snprintf(&big_endian[i * 2], 3, "%02X", (unsigned char)word[i]);
    }

    big_endian[2 * length] = '\0';
    return big_endian;
}

char *generate_random_word()
{
    printf("Welcome to the Endian CTF!\n");
    printf("You need to find both the little endian and big endian representations of a word.\n");
    printf("If you get both correct, you will receive the flag.\n");
    srand(time(NULL));

    int word_length = 5;
    char *word = (char *)malloc((word_length + 1) * sizeof(char));

    for (int i = 0; i < word_length; i++)
    {
        word[i] = (rand() % 26) + 'a';
    }

    word[word_length] = '\0';
    return word;
}
```

### Connect with netcat

We connect to the site with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/endianness]
└─$ nc titan.picoctf.net 55547
Welcome to the Endian CTF!
You need to find both the little endian and big endian representations of a word.
If you get both correct, you will receive the flag.
Word: jyput
Enter the Little Endian representation: 
```

Let's print the ascii representation of the word as help with standard python

```bash
┌──(kali㉿kali)-[~]
└─$ python -c "for c in 'jyput': print(hex(ord(c))[2:], end=' ')"
6a 79 70 75 74 
```

or with `hex` from [pwntools](https://docs.pwntools.com/en/stable/index.html)

```bash
┌──(kali㉿kali)-[~]
└─$ ~/python_venvs/pwntools/bin/pwn hex -s ' ' jyput
6a 79 70 75 74
```

### Get the flag

Now we can convert to 16-bit little and big endian values

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/endianness]
└─$ nc titan.picoctf.net 55547
Welcome to the Endian CTF!
You need to find both the little endian and big endian representations of a word.
If you get both correct, you will receive the flag.
Word: jyput
Enter the Little Endian representation: 747570796a
Correct Little Endian representation!
Enter the Big Endian representation: 6a79707574
Correct Big Endian representation!
Congratulations! You found both endian representations correctly!
Your Flag is: picoCTF{<REDACTED>}
```

As a final note:

- `word` here means 16-bit values.
- No zero byte should be added even though the string is of odd length.
- No delimiters of any kind between the hex values should be added

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Endianness - Wikipedia](https://en.wikipedia.org/wiki/Endianness)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Little Endian and Big Endian](https://levelup.gitconnected.com/little-endian-and-big-endian-74ab6441b2a7)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
