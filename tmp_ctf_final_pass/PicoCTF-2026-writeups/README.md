# picoCTF 2026 Writeups

Full writeups and solution scripts for all 70 challenges from **picoCTF 2026** (Event 79).

**Total Points: 14,500 / 14,500** | **70 / 70 Challenges Solved**

## Structure

Each challenge directory contains:
- `writeup.md` - Detailed writeup with approach, vulnerability analysis, and step-by-step solution
- `solve.py` - Python solution script with comments

## Challenges

### Binary Exploitation (8 challenges, 1650 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [Pizza Router](./Pizza_Router) | Binary Exploitation | 400 |
| [offset-cycleV2](./offset-cycleV2) | Binary Exploitation | 400 |
| [offset-cycle](./offset-cycle) | Binary Exploitation | 300 |
| [Heap Havoc](./Heap_Havoc) | Binary Exploitation | 200 |
| [Echo Escape 1](./Echo_Escape_1) | Binary Exploitation | 100 |
| [Echo Escape 2](./Echo_Escape_2) | Binary Exploitation | 100 |
| [tea-cash](./tea-cash) | Binary Exploitation | 100 |
| [Quizploit](./Quizploit) | Binary Exploitation | 50 |

### Blockchain (4 challenges, 1200 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [Reentrance](./Reentrance) | Blockchain | 400 |
| [Front_Running](./Front_Running) | Blockchain | 300 |
| [Smart_Overflow](./Smart_Overflow) | Blockchain | 300 |
| [Access_Control](./Access_Control) | Blockchain | 200 |

### Cryptography (12 challenges, 2800 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [ClusterRSA](./ClusterRSA) | Cryptography | 400 |
| [MSS_ADVANCE Revenge](./MSS_ADVANCE_Revenge) | Cryptography | 400 |
| [Not TRUe](./Not_TRUe) | Cryptography | 400 |
| [Secure Dot Product](./Secure_Dot_Product) | Cryptography | 300 |
| [Black Cobra Pepper](./Black_Cobra_Pepper) | Cryptography | 200 |
| [Related Messages](./Related_Messages) | Cryptography | 200 |
| [Small Trouble](./Small_Trouble) | Cryptography | 200 |
| [Timestamped Secrets](./Timestamped_Secrets) | Cryptography | 200 |
| [shift registers](./shift_registers) | Cryptography | 200 |
| [Shared Secrets](./Shared_Secrets) | Cryptography | 100 |
| [StegoRSA](./StegoRSA) | Cryptography | 100 |
| [cryptomaze](./cryptomaze) | Cryptography | 100 |

### Forensics (8 challenges, 1900 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [Forensics Git 2](./Forensics_Git_2) | Forensics | 400 |
| [Forensics Git 1](./Forensics_Git_1) | Forensics | 300 |
| [Rogue Tower](./Rogue_Tower) | Forensics | 300 |
| [Timeline 1](./Timeline_1) | Forensics | 300 |
| [DISKO 4](./DISKO_4) | Forensics | 200 |
| [Forensics Git 0](./Forensics_Git_0) | Forensics | 200 |
| [Binary Digits](./Binary_Digits) | Forensics | 100 |
| [Timeline 0](./Timeline_0) | Forensics | 100 |

### General Skills (17 challenges, 2450 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [bytemancy 3](./bytemancy_3) | General Skills | 400 |
| [Printer Shares 3](./Printer_Shares_3) | General Skills | 300 |
| [ABSOLUTE NANO](./ABSOLUTE_NANO) | General Skills | 200 |
| [Failure Failure](./Failure_Failure) | General Skills | 200 |
| [MultiCode](./MultiCode) | General Skills | 200 |
| [Printer Shares 2](./Printer_Shares_2) | General Skills | 200 |
| [bytemancy 2](./bytemancy_2) | General Skills | 200 |
| [KSECRETS](./KSECRETS) | General Skills | 100 |
| [Password Profiler](./Password_Profiler) | General Skills | 100 |
| [Undo](./Undo) | General Skills | 100 |
| [bytemancy 1](./bytemancy_1) | General Skills | 100 |
| [ping-cmd](./ping-cmd) | General Skills | 100 |
| [MY GIT](./MY_GIT) | General Skills | 50 |
| [Piece by Piece](./Piece_by_Piece) | General Skills | 50 |
| [Printer Shares](./Printer_Shares) | General Skills | 50 |
| [SUDO MAKE ME A SANDWICH](./SUDO_MAKE_ME_A_SANDWICH) | General Skills | 50 |
| [bytemancy 0](./bytemancy_0) | General Skills | 50 |

### Reverse Engineering (11 challenges, 2400 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [JITFP](./JITFP) | Reverse Engineering | 500 |
| [Binary Instrumentation 4](./Binary_Instrumentation_4) | Reverse Engineering | 400 |
| [Binary Instrumentation 3](./Binary_Instrumentation_3) | Reverse Engineering | 300 |
| [Autorev 1](./Autorev_1) | Reverse Engineering | 200 |
| [Secure Password Database](./Secure_Password_Database) | Reverse Engineering | 200 |
| [Silent Stream](./Silent_Stream) | Reverse Engineering | 200 |
| [The Add/On Trap](./The_Add-On_Trap) | Reverse Engineering | 200 |
| [Bypass Me](./Bypass_Me) | Reverse Engineering | 100 |
| [Gatekeeper](./Gatekeeper) | Reverse Engineering | 100 |
| [Hidden Cipher 1](./Hidden_Cipher_1) | Reverse Engineering | 100 |
| [Hidden Cipher 2](./Hidden_Cipher_2) | Reverse Engineering | 100 |

### Web Exploitation (10 challenges, 2100 pts)

| Challenge | Category | Points |
|-----------|----------|--------|
| [paper-2](./Paper2) | Web Exploitation | 500 |
| [ORDER ORDER](./ORDER_ORDER) | Web Exploitation | 300 |
| [Sql Map1](./Sql_Map1) | Web Exploitation | 300 |
| [Fool the Lockout](./Fool_the_Lockout) | Web Exploitation | 200 |
| [No FA](./No_FA) | Web Exploitation | 200 |
| [Secret Box](./Secret_Box) | Web Exploitation | 200 |
| [Credential Stuffing](./Credential_Stuffing) | Web Exploitation | 100 |
| [Hashgate](./Hashgate) | Web Exploitation | 100 |
| [North-South](./North-South) | Web Exploitation | 100 |
| [Old Sessions](./Old_Sessions) | Web Exploitation | 100 |

## Tools Used

- **pwntools** - Binary exploitation and remote interaction
- **web3.py** - Blockchain challenge interaction
- **sympy / gmpy2** - Cryptographic math (RSA, CRT, etc.)
- **requests** - Web exploitation
- **Frida** - Dynamic binary instrumentation
- **Sleuth Kit** - Disk image forensics
- **angr / Z3** - Symbolic execution and constraint solving
- **scapy / pyshark** - Packet capture analysis
- **paramiko** - SSH automation
