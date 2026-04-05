# Silent Stream - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 200

## Challenge Description
We recovered a suspicious packet capture file that seems to contain a transferred file. The sender was kind enough to allow us to analyze their transfer tool.

## Approach

This challenge provides two artifacts:
1. A **PCAP file** containing captured network traffic of a file transfer
2. A **binary/tool** used to perform the transfer (the "transfer tool" we can analyze)

By reverse engineering the transfer tool, we can understand the custom encoding/protocol used to transmit the file. The tool uses a simple byte-level encoding scheme:

```python
def encode_byte(b, key):
    return (b + key) % 256
```

Each byte of the file is encoded by adding a key value (default: `42`) and taking the result modulo 256. This is essentially a Caesar cipher operating on bytes.

### Reverse Engineering Steps

1. **Analyze the binary**: Using tools like Ghidra, IDA, or `strings`, we can identify the encoding function. The tool reads a file, encodes each byte by adding a key, and sends the encoded bytes over the network.

2. **Identify the encoding**: The encoding is `encoded = (original + key) % 256`. The decoding is therefore `original = (encoded - key) % 256`.

3. **Determine the key**: The default key is `42`, but it may also be visible in the binary's strings or hardcoded constants. The tool may also transmit the key as part of the protocol header.

4. **Extract from PCAP**: Using Wireshark, tshark, or Scapy, extract the TCP payload data from the packet capture. This contains the encoded file bytes.

5. **Decode**: Apply the inverse transformation to recover the original file, which contains the flag.

## Solution

1. Open the PCAP in Wireshark and identify the TCP stream carrying the file transfer.
2. Extract the raw payload bytes (Follow TCP Stream -> Save as Raw).
3. Reverse engineer the transfer tool to find the encoding key (42).
4. Decode each byte: `decoded = (encoded - 42) % 256`.
5. The decoded output is the original file containing the flag.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
