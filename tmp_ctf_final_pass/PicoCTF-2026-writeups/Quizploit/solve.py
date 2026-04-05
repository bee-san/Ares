#!/usr/bin/env python3
"""
Quizploit - picoCTF 2026
Category: Binary Exploitation | Points: 50

Solve the quiz by reading the source code and providing correct answers.

Usage:
    python3 solve.py                     # Run against local binary
    python3 solve.py REMOTE_HOST PORT    # Run against remote server

Before running:
    1. Download the binary and source code from the challenge page
    2. Read the source code to determine the correct quiz answers
    3. Update the ANSWERS list below with the correct responses
    4. chmod +x quizploit (if running locally)
"""

import sys
from pwn import *

# ============================================================
# CONFIGURATION - UPDATE THESE BASED ON THE SOURCE CODE
# ============================================================

BINARY = "./quizploit"

# Read the source code and fill in answers here.
# These are EXAMPLE answers -- replace them with the real ones
# found in the downloaded source file.
ANSWERS = [
    b"64",           # Example: Q1 - buffer size
    b"0xdeadbeef",   # Example: Q2 - secret value in hex
    b"win",          # Example: Q3 - name of the win function
    # Add more answers as needed based on the actual quiz questions
]

# ============================================================
# EXPLOIT
# ============================================================

def solve():
    # Connect to remote or run locally
    if len(sys.argv) >= 3:
        host = sys.argv[1]
        port = int(sys.argv[2])
        log.info(f"Connecting to {host}:{port}")
        p = remote(host, port)
    else:
        log.info(f"Running local binary: {BINARY}")
        p = process(BINARY)

    # Send each answer when prompted
    for i, answer in enumerate(ANSWERS):
        log.info(f"Question {i+1}: sending answer '{answer.decode()}'")
        # Wait for the question prompt (adjust the expected text as needed)
        p.recvuntil(b"? ")
        p.sendline(answer)

    # Receive the flag
    response = p.recvall(timeout=3).decode(errors="ignore")
    print("\n" + "=" * 50)
    print("Server response:")
    print(response)
    print("=" * 50)

    # Try to extract the flag
    import re
    flag_match = re.search(r'picoCTF\{[^}]+\}', response)
    if flag_match:
        print(f"\nFLAG: {flag_match.group(0)}")
    else:
        print("\nFlag not found in output. Check the response above.")
        print("You may need to update the ANSWERS list based on the source code.")

    p.close()


def read_source():
    """Helper: read and display the source code if available."""
    import os
    source_files = [f for f in os.listdir('.') if f.endswith(('.c', '.h'))]
    if source_files:
        print("Found source files:", source_files)
        for sf in source_files:
            print(f"\n{'=' * 50}")
            print(f"Contents of {sf}:")
            print('=' * 50)
            with open(sf, 'r') as f:
                print(f.read())
        print("\nUpdate the ANSWERS list in this script based on the source above.")
    else:
        print("No source files found in current directory.")
        print("Download them from the challenge page first.")


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "--read-source":
        read_source()
    else:
        solve()
