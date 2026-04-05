#!/usr/bin/env python3
"""
Rogue Tower - picoCTF 2026 (Forensics, 300 pts)

Analyze a PCAP containing GSM/cellular network traffic to identify a rogue
cell tower and extract the hidden flag.

Approach:
  1. Parse GSMTAP packets to identify all cell towers (by Cell ID, ARFCN, etc.)
  2. Identify the rogue tower by anomalous identifiers
  3. Extract hidden data (SMS content, encoded payloads, etc.)
  4. Decode the flag

Usage:
    python3 solve.py <capture.pcap>

Dependencies:
    pip install scapy pyshark
"""

import sys
import os
import struct
import re
import binascii
import subprocess
from collections import defaultdict, Counter

# Try to import pyshark (preferred for GSM dissection)
try:
    import pyshark
    HAS_PYSHARK = True
except ImportError:
    HAS_PYSHARK = False

# Try to import scapy as fallback
try:
    from scapy.all import rdpcap, raw, UDP
    HAS_SCAPY = True
except ImportError:
    HAS_SCAPY = False


# GSMTAP header format (from libosmocore)
# https://osmocom.org/projects/baseband/wiki/GSMTAP
GSMTAP_PORT = 4729
GSMTAP_HDR_FMT = ">BBBBHHBBxxxxI"
GSMTAP_HDR_SIZE = 16

# GSMTAP types
GSMTAP_TYPE_UM = 0x01
GSMTAP_TYPE_ABIS = 0x02
GSMTAP_TYPE_UM_BURST = 0x03
GSMTAP_TYPE_SIM = 0x04
GSMTAP_TYPE_GB_LLC = 0x08


def parse_gsmtap_header(data):
    """Parse a GSMTAP header and return a dict of fields."""
    if len(data) < GSMTAP_HDR_SIZE:
        return None

    (version, hdr_len, pdu_type, timeslot,
     arfcn, signal_dbm, snr_db, subtype,
     frame_number) = struct.unpack(GSMTAP_HDR_FMT, data[:GSMTAP_HDR_SIZE])

    actual_hdr_len = hdr_len * 4  # header length is in 32-bit words

    return {
        'version': version,
        'hdr_len': actual_hdr_len,
        'type': pdu_type,
        'timeslot': timeslot,
        'arfcn': arfcn & 0x3FFF,  # mask out uplink flag
        'uplink': bool(arfcn & 0x4000),
        'signal_dbm': signal_dbm,
        'snr_db': snr_db,
        'subtype': subtype,
        'frame_number': frame_number,
    }


def extract_strings(data, min_length=4):
    """Extract printable ASCII strings from binary data."""
    result = []
    current = []
    for byte in data:
        if 32 <= byte < 127:
            current.append(chr(byte))
        else:
            if len(current) >= min_length:
                result.append(''.join(current))
            current = []
    if len(current) >= min_length:
        result.append(''.join(current))
    return result


def try_decode_sms_7bit(data):
    """Attempt to decode GSM 7-bit encoded SMS data."""
    # GSM 7-bit default alphabet (basic set)
    gsm_alphabet = (
        "@\u00a3$\u00a5\u00e8\u00e9\u00f9\u00ec\u00f2\u00c7\n\u00d8\u00f8\r\u00c5\u00e5"
        "\u0394_\u03a6\u0393\u039b\u03a9\u03a0\u03a8\u03a3\u0398\u039e\x1b\u00c6\u00e6"
        "\u00df\u00c9 !\"#\u00a4%&'()*+,-./0123456789:;<=>?"
        "\u00a1ABCDEFGHIJKLMNOPQRSTUVWXYZ\u00c4\u00d6\u00d1\u00dc\u00a7"
        "\u00bfabcdefghijklmnopqrstuvwxyz\u00e4\u00f6\u00f1\u00fc\u00e0"
    )

    try:
        result = []
        bits = 0
        buf = 0
        for byte in data:
            buf |= (byte << bits)
            bits += 8
            while bits >= 7:
                char_idx = buf & 0x7F
                if char_idx < len(gsm_alphabet):
                    result.append(gsm_alphabet[char_idx])
                buf >>= 7
                bits -= 7
        return ''.join(result)
    except Exception:
        return None


def solve_with_pyshark(pcap_path):
    """Use pyshark (tshark backend) for full GSM protocol dissection."""
    print("[*] Analyzing with pyshark (tshark backend)...")

    # ---- Phase 1: Identify all cell towers ----
    print("\n[*] Phase 1: Enumerating cell towers...")
    towers = defaultdict(lambda: {'count': 0, 'arfcns': set(), 'types': Counter()})
    sms_texts = []
    all_payloads = []
    flag_candidates = []

    try:
        cap = pyshark.FileCapture(pcap_path, display_filter='gsmtap')
    except Exception as e:
        print(f"[!] pyshark error: {e}")
        return None

    for pkt in cap:
        try:
            if hasattr(pkt, 'gsmtap'):
                gsmtap = pkt.gsmtap
                arfcn = int(getattr(gsmtap, 'arfcn', 0))
                cell_id = getattr(gsmtap, 'cell_id', 'unknown')
                pdu_type = getattr(gsmtap, 'type', 'unknown')

                key = f"CID:{cell_id}_ARFCN:{arfcn}"
                towers[key]['count'] += 1
                towers[key]['arfcns'].add(arfcn)
                towers[key]['types'][str(pdu_type)] += 1

            # Check for SMS content
            for layer in pkt.layers:
                layer_name = layer.layer_name.lower()
                if 'sms' in layer_name:
                    for field_name in layer.field_names:
                        val = getattr(layer, field_name, '')
                        if val and 'pico' in str(val).lower():
                            flag_candidates.append(str(val))
                        sms_texts.append(f"{field_name}: {val}")

            # Extract raw payload data
            if hasattr(pkt, 'data'):
                raw_data = getattr(pkt.data, 'data', '')
                if raw_data:
                    all_payloads.append(raw_data.replace(':', ''))

        except Exception:
            continue

    cap.close()

    # Print tower summary
    print(f"\n[*] Found {len(towers)} unique cell tower identifiers:")
    for tower_id, info in sorted(towers.items(), key=lambda x: x[1]['count'], reverse=True):
        print(f"    {tower_id}: {info['count']} packets")

    # ---- Phase 2: Identify the rogue tower ----
    print("\n[*] Phase 2: Identifying the rogue tower...")

    if len(towers) > 1:
        # The rogue tower typically has fewer packets (it's intermittent)
        # or has an unusual ARFCN/Cell ID
        avg_count = sum(t['count'] for t in towers.values()) / len(towers)
        for tower_id, info in towers.items():
            if info['count'] < avg_count * 0.3 or info['count'] > avg_count * 3:
                print(f"    [!] Anomalous tower: {tower_id} ({info['count']} packets)")

    # ---- Phase 3: Extract flag ----
    print("\n[*] Phase 3: Searching for flag...")

    # Check SMS content
    if sms_texts:
        print(f"    Found {len(sms_texts)} SMS fields")
        for text in sms_texts:
            if 'pico' in text.lower():
                print(f"    [+] Potential flag in SMS: {text}")

    # Check payloads for flag pattern
    for payload_hex in all_payloads:
        try:
            payload_bytes = bytes.fromhex(payload_hex)
            decoded = payload_bytes.decode('utf-8', errors='ignore')
            if 'picoCTF' in decoded:
                flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                if flag_match:
                    return flag_match.group(0)
        except Exception:
            continue

    # Check flag candidates
    for candidate in flag_candidates:
        flag_match = re.search(r'picoCTF\{[^}]+\}', candidate)
        if flag_match:
            return flag_match.group(0)

    return None


def solve_with_scapy(pcap_path):
    """Use scapy to parse the PCAP and extract GSMTAP data."""
    print("[*] Analyzing with scapy...")

    packets = rdpcap(pcap_path)
    print(f"[*] Loaded {len(packets)} packets")

    towers = defaultdict(lambda: {'count': 0, 'payloads': []})
    all_payloads = []

    for pkt in packets:
        # Look for GSMTAP (typically UDP port 4729)
        if pkt.haslayer(UDP):
            udp = pkt[UDP]
            if udp.dport == GSMTAP_PORT or udp.sport == GSMTAP_PORT:
                payload = bytes(udp.payload)
                gsmtap = parse_gsmtap_header(payload)
                if gsmtap:
                    key = f"ARFCN:{gsmtap['arfcn']}_TS:{gsmtap['timeslot']}"
                    towers[key]['count'] += 1
                    # Extract the payload after GSMTAP header
                    gsm_payload = payload[gsmtap['hdr_len']:]
                    towers[key]['payloads'].append(gsm_payload)
                    all_payloads.append(gsm_payload)

        # Also check raw packet data for flag strings
        raw_bytes = raw(pkt)
        if b'picoCTF' in raw_bytes:
            match = re.search(rb'picoCTF\{[^}]+\}', raw_bytes)
            if match:
                return match.group(0).decode()

    # Print tower summary
    print(f"\n[*] Found {len(towers)} unique GSMTAP sources:")
    for tower_id, info in sorted(towers.items(), key=lambda x: x[1]['count'], reverse=True):
        print(f"    {tower_id}: {info['count']} packets")

    # Search all GSMTAP payloads for the flag
    print("\n[*] Searching GSMTAP payloads for flag...")
    for tower_id, info in towers.items():
        combined_payload = b''.join(info['payloads'])

        # Direct string search
        if b'picoCTF' in combined_payload:
            match = re.search(rb'picoCTF\{[^}]+\}', combined_payload)
            if match:
                print(f"    [+] Flag found in {tower_id}!")
                return match.group(0).decode()

        # Extract and check ASCII strings
        strings = extract_strings(combined_payload)
        for s in strings:
            if 'picoCTF' in s:
                match = re.search(r'picoCTF\{[^}]+\}', s)
                if match:
                    return match.group(0)

        # Try GSM 7-bit decoding
        decoded = try_decode_sms_7bit(combined_payload)
        if decoded and 'picoCTF' in decoded:
            match = re.search(r'picoCTF\{[^}]+\}', decoded)
            if match:
                return match.group(0)

    # Try concatenating all payloads (flag might span packets)
    print("[*] Trying payload reassembly...")
    all_data = b''.join(all_payloads)

    # Check hex-encoded data
    hex_str = all_data.hex()
    try:
        decoded_hex = bytes.fromhex(hex_str).decode('utf-8', errors='ignore')
        if 'picoCTF' in decoded_hex:
            match = re.search(r'picoCTF\{[^}]+\}', decoded_hex)
            if match:
                return match.group(0)
    except Exception:
        pass

    # Check base64 patterns in the data
    b64_pattern = re.compile(rb'[A-Za-z0-9+/]{20,}={0,2}')
    for match in b64_pattern.finditer(all_data):
        try:
            import base64
            decoded = base64.b64decode(match.group(0)).decode('utf-8', errors='ignore')
            if 'picoCTF' in decoded:
                flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                if flag_match:
                    return flag_match.group(0)
        except Exception:
            continue

    return None


def solve_with_tshark(pcap_path):
    """Use tshark directly via subprocess as a fallback."""
    print("[*] Analyzing with tshark (subprocess)...")

    # First, search for the flag pattern directly in packet hex dumps
    try:
        result = subprocess.run(
            ['tshark', '-r', pcap_path, '-x'],
            capture_output=True, text=True, timeout=60
        )
        if 'pico' in result.stdout.lower():
            # Search through hex dump for the flag
            lines = result.stdout.split('\n')
            for line in lines:
                if 'pico' in line.lower():
                    print(f"    [!] Potential flag reference: {line.strip()}")
    except Exception as e:
        print(f"    [!] tshark hex dump failed: {e}")

    # Extract GSMTAP Cell IDs
    try:
        result = subprocess.run(
            ['tshark', '-r', pcap_path, '-Y', 'gsmtap',
             '-T', 'fields', '-e', 'gsmtap.arfcn', '-e', 'gsmtap.cell_id',
             '-e', 'gsmtap.type'],
            capture_output=True, text=True, timeout=60
        )
        if result.stdout.strip():
            print("\n[*] GSMTAP fields found:")
            lines = result.stdout.strip().split('\n')
            cell_ids = set()
            for line in lines[:20]:  # Show first 20
                print(f"    {line}")
                parts = line.split('\t')
                if len(parts) >= 2:
                    cell_ids.add(parts[1])
            print(f"\n[*] Unique Cell IDs: {cell_ids}")
    except Exception as e:
        print(f"    [!] tshark field extraction failed: {e}")

    # Try to extract SMS text
    try:
        result = subprocess.run(
            ['tshark', '-r', pcap_path, '-Y', 'gsm_sms',
             '-T', 'fields', '-e', 'gsm_sms.sms_text'],
            capture_output=True, text=True, timeout=60
        )
        if result.stdout.strip():
            print(f"\n[*] SMS texts found:")
            for line in result.stdout.strip().split('\n'):
                print(f"    {line}")
                if 'picoCTF' in line:
                    match = re.search(r'picoCTF\{[^}]+\}', line)
                    if match:
                        return match.group(0)
    except Exception as e:
        print(f"    [!] SMS extraction failed: {e}")

    # Extract all UDP payloads on GSMTAP port
    try:
        result = subprocess.run(
            ['tshark', '-r', pcap_path, '-Y', f'udp.port == {GSMTAP_PORT}',
             '-T', 'fields', '-e', 'data.data'],
            capture_output=True, text=True, timeout=60
        )
        if result.stdout.strip():
            for line in result.stdout.strip().split('\n'):
                hex_data = line.replace(':', '').strip()
                if hex_data:
                    try:
                        raw_bytes = bytes.fromhex(hex_data)
                        decoded = raw_bytes.decode('utf-8', errors='ignore')
                        if 'picoCTF' in decoded:
                            match = re.search(r'picoCTF\{[^}]+\}', decoded)
                            if match:
                                return match.group(0)
                    except Exception:
                        continue
    except Exception as e:
        print(f"    [!] Payload extraction failed: {e}")

    # Brute-force: search ALL packet bytes for the flag
    try:
        result = subprocess.run(
            ['tshark', '-r', pcap_path, '-T', 'fields', '-e', 'frame.protocols',
             '-e', 'data.data'],
            capture_output=True, text=True, timeout=60
        )
        for line in result.stdout.strip().split('\n'):
            parts = line.split('\t')
            if len(parts) >= 2:
                hex_data = parts[1].replace(':', '').strip()
                if hex_data:
                    try:
                        raw_bytes = bytes.fromhex(hex_data)
                        decoded = raw_bytes.decode('utf-8', errors='ignore')
                        if 'picoCTF' in decoded:
                            match = re.search(r'picoCTF\{[^}]+\}', decoded)
                            if match:
                                return match.group(0)
                    except Exception:
                        continue
    except Exception:
        pass

    return None


def main():
    if len(sys.argv) < 2:
        print("Rogue Tower - picoCTF 2026 (Forensics, 300 pts)")
        print()
        print("Usage: python3 solve.py <capture.pcap>")
        print()
        print("This script analyzes GSM/GSMTAP network traffic to identify a")
        print("rogue cell tower and extract the hidden flag.")
        print()
        print("Dependencies (install at least one):")
        print("  pip install pyshark    # Preferred - full protocol dissection")
        print("  pip install scapy      # Fallback - manual GSMTAP parsing")
        print("  (or have tshark in PATH)")
        sys.exit(1)

    pcap_path = sys.argv[1]
    if not os.path.isfile(pcap_path):
        print(f"[!] File not found: {pcap_path}")
        sys.exit(1)

    print(f"[*] Analyzing: {pcap_path}")
    print(f"[*] File size: {os.path.getsize(pcap_path)} bytes")
    print()

    flag = None

    # Try pyshark first (best GSM dissection via tshark)
    if HAS_PYSHARK:
        flag = solve_with_pyshark(pcap_path)

    # Try scapy as fallback
    if flag is None and HAS_SCAPY:
        flag = solve_with_scapy(pcap_path)

    # Try tshark directly as last resort
    if flag is None:
        flag = solve_with_tshark(pcap_path)

    # Final output
    print()
    if flag:
        print(f"{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("[!] Flag not found automatically.")
        print()
        print("[*] Manual investigation steps:")
        print("    1. Open the PCAP in Wireshark")
        print("    2. Filter: gsmtap")
        print("    3. Look at Statistics > Endpoints for cell tower IDs")
        print("    4. Identify anomalous Cell ID / ARFCN values")
        print("    5. Filter traffic for the rogue tower")
        print("    6. Check SMS content (gsm_sms filter)")
        print("    7. Check packet payloads for encoded flag data")
        print("    8. Try: tshark -r capture.pcap -x | grep -i pico")


if __name__ == '__main__':
    main()
