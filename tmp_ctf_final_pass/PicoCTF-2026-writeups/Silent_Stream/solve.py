#!/usr/bin/env python3
"""
Silent Stream - picoCTF 2026 (Reverse Engineering, 200 pts)

A file was transferred using a custom tool that encodes each byte as:
    encoded = (original_byte + key) % 256

The default key is 42. We need to:
  1. Extract the payload from the PCAP file
  2. Reverse the encoding to recover the original file
  3. Find the flag in the recovered data

Usage:
  python3 solve.py <pcap_file> [--key KEY]
  e.g.: python3 solve.py capture.pcap
        python3 solve.py capture.pcap --key 42

If no pcap file is given, the script searches the current directory for .pcap/.pcapng files.
"""

import sys
import os
import argparse
import re

def decode_byte(b, key):
    """Reverse the encoding: original = (encoded - key) % 256"""
    return (b - key) % 256

def extract_tcp_payload_scapy(pcap_file):
    """Extract TCP payload data from a PCAP file using Scapy."""
    try:
        from scapy.all import rdpcap, TCP, Raw
        print(f"[*] Reading PCAP file: {pcap_file}")
        packets = rdpcap(pcap_file)

        # Collect all TCP payloads, grouped by stream
        streams = {}
        for pkt in packets:
            if pkt.haslayer(TCP) and pkt.haslayer(Raw):
                # Use (src_ip, src_port, dst_ip, dst_port) as stream key
                src = pkt[TCP].sport
                dst = pkt[TCP].dport
                key = (src, dst) if src > dst else (dst, src)
                if key not in streams:
                    streams[key] = b""
                streams[key] += bytes(pkt[Raw].load)

        if not streams:
            print("[!] No TCP payload data found in PCAP")
            return None

        # Return the largest stream (most likely the file transfer)
        largest = max(streams.values(), key=len)
        print(f"[*] Found {len(streams)} TCP stream(s)")
        print(f"[*] Largest payload: {len(largest)} bytes")
        return largest

    except ImportError:
        print("[!] Scapy not installed, trying dpkt...")
        return None

def extract_tcp_payload_dpkt(pcap_file):
    """Extract TCP payload data using dpkt as fallback."""
    try:
        import dpkt

        print(f"[*] Reading PCAP file with dpkt: {pcap_file}")
        payloads = {}

        with open(pcap_file, "rb") as f:
            try:
                pcap = dpkt.pcap.Reader(f)
            except ValueError:
                f.seek(0)
                pcap = dpkt.pcapng.Reader(f)

            for ts, buf in pcap:
                try:
                    eth = dpkt.ethernet.Ethernet(buf)
                    if not isinstance(eth.data, dpkt.ip.IP):
                        continue
                    ip = eth.data
                    if not isinstance(ip.data, dpkt.tcp.TCP):
                        continue
                    tcp = ip.data
                    if tcp.data:
                        key = (tcp.sport, tcp.dport)
                        if key not in payloads:
                            payloads[key] = b""
                        payloads[key] += tcp.data
                except Exception:
                    continue

        if not payloads:
            return None

        largest = max(payloads.values(), key=len)
        print(f"[*] Found {len(payloads)} stream(s), largest: {len(largest)} bytes")
        return largest

    except ImportError:
        print("[!] dpkt not installed")
        return None

def extract_tcp_payload_tshark(pcap_file):
    """Extract TCP payload using tshark as last resort."""
    import subprocess

    print(f"[*] Extracting payload with tshark...")
    try:
        # Get raw TCP payload bytes as hex
        result = subprocess.run(
            ["tshark", "-r", pcap_file, "-T", "fields",
             "-e", "tcp.payload", "-Y", "tcp.payload"],
            capture_output=True, text=True, timeout=30
        )

        if result.returncode != 0:
            print(f"[!] tshark error: {result.stderr}")
            return None

        hex_data = result.stdout.strip().replace("\n", "").replace(":", "")
        if not hex_data:
            return None

        payload = bytes.fromhex(hex_data)
        print(f"[*] Extracted {len(payload)} bytes via tshark")
        return payload

    except FileNotFoundError:
        print("[!] tshark not found")
        return None
    except Exception as e:
        print(f"[!] tshark error: {e}")
        return None

def find_pcap_file():
    """Search current directory for PCAP files."""
    for f in os.listdir("."):
        if f.endswith((".pcap", ".pcapng")):
            return f
    return None

def try_all_keys(encoded_data):
    """
    Brute-force the encoding key by trying all 256 possible values
    and checking for the flag format in the decoded output.
    """
    print("[*] Brute-forcing encoding key (0-255)...")
    for key in range(256):
        decoded = bytes([decode_byte(b, key) for b in encoded_data])
        if b"picoCTF{" in decoded:
            print(f"[+] Found key: {key}")
            return key, decoded
    return None, None

def main():
    parser = argparse.ArgumentParser(description="Silent Stream solver - picoCTF 2026")
    parser.add_argument("pcap_file", nargs="?", help="Path to the PCAP file")
    parser.add_argument("--key", type=int, default=None,
                        help="Encoding key (default: auto-detect, fallback 42)")
    parser.add_argument("--output", "-o", default="recovered_file",
                        help="Output filename for recovered data")
    args = parser.parse_args()

    pcap_file = args.pcap_file
    if pcap_file is None:
        pcap_file = find_pcap_file()
        if pcap_file is None:
            print("[!] No PCAP file specified and none found in current directory")
            print("Usage: python3 solve.py <pcap_file> [--key KEY]")
            sys.exit(1)

    if not os.path.exists(pcap_file):
        print(f"[!] File not found: {pcap_file}")
        sys.exit(1)

    print("=" * 60)
    print("  Silent Stream - picoCTF 2026 Solver")
    print("=" * 60)
    print()

    # Step 1: Extract TCP payload from PCAP
    payload = None
    for extractor in [extract_tcp_payload_scapy, extract_tcp_payload_dpkt, extract_tcp_payload_tshark]:
        payload = extractor(pcap_file)
        if payload:
            break

    if payload is None:
        print("[!] Could not extract payload from PCAP")
        print("[!] Try opening in Wireshark: Follow TCP Stream -> Save as Raw")
        sys.exit(1)

    print(f"[*] Extracted {len(payload)} bytes of encoded data")

    # Step 2: Decode the payload
    if args.key is not None:
        key = args.key
        decoded = bytes([decode_byte(b, key) for b in payload])
        print(f"[*] Decoded with key={key}")
    else:
        # First try brute force to find the correct key
        key, decoded = try_all_keys(payload)
        if key is None:
            # Fall back to default key of 42
            key = 42
            decoded = bytes([decode_byte(b, key) for b in payload])
            print(f"[*] Using default key={key}")

    # Step 3: Search for the flag
    flag_match = re.search(rb"picoCTF\{[^}]+\}", decoded)
    if flag_match:
        flag = flag_match.group().decode()
        print(f"\n[+] FLAG FOUND: {flag}")
    else:
        print("[*] Flag pattern not directly found, checking decoded content...")
        # Show printable characters
        printable = "".join(chr(b) if 32 <= b < 127 else "." for b in decoded)
        print(f"[*] Decoded preview (first 500 chars):")
        print(printable[:500])

    # Step 4: Save recovered file
    with open(args.output, "wb") as f:
        f.write(decoded)
    print(f"\n[*] Recovered data saved to: {args.output}")

    # Also check if it's a known file type
    if decoded[:4] == b"\x89PNG":
        new_name = args.output + ".png"
        os.rename(args.output, new_name)
        print(f"[*] Detected PNG image, renamed to: {new_name}")
    elif decoded[:2] == b"PK":
        new_name = args.output + ".zip"
        os.rename(args.output, new_name)
        print(f"[*] Detected ZIP archive, renamed to: {new_name}")
    elif decoded[:4] == b"%PDF":
        new_name = args.output + ".pdf"
        os.rename(args.output, new_name)
        print(f"[*] Detected PDF document, renamed to: {new_name}")

    print("\n[*] Done!")

if __name__ == "__main__":
    main()
