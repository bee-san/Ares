# Rogue Tower - picoCTF 2026

**Category:** Forensics
**Points:** 300

## Challenge Description

A suspicious cell tower has been detected in the network. Analyze the captured network traffic to identify the rogue tower and extract the hidden data.

## Approach

This challenge simulates a **rogue cell tower (IMSI catcher / Stingray)** scenario. A rogue base station intercepts mobile communications by masquerading as a legitimate cell tower. The challenge provides a network traffic capture (PCAP) file containing GSM/cellular protocol traffic that we must analyze to identify the rogue tower and extract hidden/exfiltrated data.

### Key Concepts

1. **IMSI Catchers**: Devices that impersonate legitimate cell towers to intercept mobile communications. They exploit the fact that GSM does not require the network to authenticate itself to the phone.
2. **GSM Protocol Analysis**: GSM traffic in PCAPs can be analyzed using Wireshark with the GSM dissectors (GSM A, GSM MAP, GSMTAP, etc.).
3. **Cell Tower Identification**: Each cell tower has identifying information including:
   - **MCC (Mobile Country Code)**: Identifies the country
   - **MNC (Mobile Network Code)**: Identifies the carrier
   - **LAC (Location Area Code)**: Identifies the location area
   - **Cell ID (CID)**: Identifies the specific cell tower
4. **Rogue Tower Indicators**: A rogue tower typically has anomalous identifiers -- mismatched MCC/MNC, unusual LAC/CID, stronger signal forcing handover, or downgrade attacks (forcing 2G instead of 4G/5G).

### Analysis Strategy

1. **Open the PCAP in Wireshark** and filter for GSM/GSMTAP traffic
2. **Identify all cell towers** by extracting MCC, MNC, LAC, and Cell ID values
3. **Spot the anomaly**: The rogue tower will have identifiers that don't match the legitimate network, or will exhibit suspicious behavior such as:
   - Identity requests (IMSI catching)
   - Cipher mode commands disabling encryption (A5/0)
   - Unusual system information broadcasts
4. **Extract hidden data**: The flag is likely embedded in:
   - SMS messages sent through the rogue tower (in plaintext due to disabled encryption)
   - Custom/malformed protocol fields
   - Data exfiltrated via the rogue tower's traffic
   - Base64 or hex-encoded strings in packet payloads

### Wireshark Filters

Useful display filters for this type of analysis:
- `gsmtap` -- All GSMTAP encapsulated traffic
- `gsm_a.dtap` -- GSM A-interface DTAP messages
- `gsm_a.dtap.msg_rr_type == 0x3f` -- System Information messages
- `gsm_sms` -- SMS messages
- `gsm_a.dtap.msg_mm_type == 0x05` -- Identity Request (IMSI catching)
- `gsm_a.dtap.msg_cc_type` -- Call control messages

### Typical Data Hiding Locations

- **SMS PDU content**: The rogue tower may relay or capture SMS messages containing the flag
- **BCCH System Information**: Custom data embedded in broadcast control channel messages
- **Padding fields**: Data hidden in protocol padding bytes
- **ARFCN (Absolute Radio Frequency Channel Number)**: Unusual frequency assignments that encode data
- **Hex-encoded data in user payloads**: After the rogue tower downgrades encryption, plaintext data becomes visible

## Solution

1. **Download the PCAP file** provided by the challenge.
2. **Open in Wireshark** and apply GSM-related display filters.
3. **Enumerate cell towers**: Extract all unique (MCC, MNC, LAC, CID) tuples from System Information messages.
4. **Identify the rogue tower**: Look for mismatched or anomalous identifiers compared to the legitimate towers.
5. **Filter traffic through the rogue tower**: Isolate packets associated with the rogue tower's identifiers.
6. **Extract the flag**: Look for SMS content, plaintext data, or encoded payloads in the rogue tower's traffic. The flag may be:
   - Directly in an SMS message body
   - Encoded in hex/base64 within packet fields
   - Spread across multiple packets (requiring reassembly)
   - Hidden in custom GSMTAP header fields or user data
7. **Decode if necessary**: Apply base64/hex decoding, or reassemble fragmented data.

### Manual Approach with tshark

```bash
# List all GSMTAP packets
tshark -r capture.pcap -Y "gsmtap"

# Extract Cell IDs from the capture
tshark -r capture.pcap -Y "gsmtap" -T fields -e gsmtap.cell_id | sort -u

# Extract ARFCN values
tshark -r capture.pcap -Y "gsmtap" -T fields -e gsmtap.arfcn | sort -u

# Look for SMS content
tshark -r capture.pcap -Y "gsm_sms" -T fields -e gsm_sms.sms_text

# Extract all readable strings from specific packets
tshark -r capture.pcap -Y "gsmtap" -T fields -e data.data | xxd -r -p

# Search for the flag pattern directly
tshark -r capture.pcap -x | grep -i "pico"
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
