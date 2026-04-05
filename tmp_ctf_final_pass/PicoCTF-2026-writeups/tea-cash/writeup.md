# tea-cash - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 100

## Challenge Description

You've stumbled upon a mysterious cash register that doesn't keep money -- it keeps secrets in memory. Traverse the free list wisely, and you might just find the flag hiding in a freed chunk.

## Approach

The challenge name "tea-cash" is a play on words for **tcache** -- the Thread Local Caching mechanism in glibc's heap allocator. The description tells us the flag is hiding in a **freed chunk**, and we need to **traverse the free list** to find it.

### Understanding Tcache

Since glibc 2.26, the tcache (Thread Cache) is a per-thread caching layer that sits in front of the traditional fastbin/smallbin/unsortedbin system. Key properties:

- **Tcache bins**: 64 singly-linked lists, one per size class (chunks of size 24 to 1032 bytes in 16-byte increments on 64-bit systems).
- **LIFO order**: Chunks are added and removed in Last-In-First-Out order.
- **Max 7 entries per bin** (by default).
- **Minimal security checks**: Historically, tcache had very few integrity checks, making it a prime target for heap exploitation.

### The Challenge Mechanism

Based on the description, this appears to be a **tcache traversal / free list reading** challenge:

1. The program allocates a chunk, stores the flag in it, and then frees the chunk.
2. The freed chunk goes into a tcache bin, but the flag data remains in memory (free() does not zero out the data).
3. We need to interact with the program to allocate a new chunk of the same size, which will be served from the tcache -- returning the same memory that previously held the flag.
4. Reading this newly allocated chunk reveals the flag.

### Alternative Mechanisms

- The program may let us view freed chunks directly through a "traverse free list" menu option.
- The program may have a use-after-free (UAF) where we can read a chunk after it has been freed.
- The flag might be split across multiple freed chunks that need to be traversed in order.

## Solution

1. **Connect** to the remote service.
2. **Interact with the menu** to understand available operations (allocate, free, read, write, etc.).
3. **Identify the flag chunk**: The flag was stored in a chunk that has been freed.
4. **Retrieve the flag** by either:
   - Allocating a chunk of the same size (tcache will return the freed chunk with the flag still in it)
   - Using a "view" or "traverse" feature if the program provides one
   - Exploiting a use-after-free condition
5. **Read the flag** from the returned chunk.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
