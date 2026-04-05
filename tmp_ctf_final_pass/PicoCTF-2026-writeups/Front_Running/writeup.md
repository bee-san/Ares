# Front_Running - picoCTF 2026

**Category:** Blockchain
**Points:** 300

## Challenge Description

A mysterious vault has been discovered on the blockchain. It's programmed to release a secret flag to anyone who can provide the correct password. (Blockchain front-running attack)

## Approach

This challenge introduces a **blockchain front-running attack**, where the goal is to intercept a pending transaction from the mempool, extract sensitive data from it (in this case, a password), and submit your own transaction with higher gas to get it mined first.

### What is Front-Running?

On Ethereum and similar blockchains, when a user sends a transaction, it enters the **mempool** (memory pool) -- a holding area for unconfirmed transactions waiting to be included in a block. Every node on the network can see these pending transactions, including their full calldata.

If a transaction contains a plaintext password being sent to a vault contract's `unlock(string password)` function, **anyone watching the mempool can read that password** before the transaction is confirmed. An attacker can then:

1. Observe the pending transaction in the mempool
2. Decode the calldata to extract the password
3. Submit their own `unlock()` transaction with the same password but **higher gas price**
4. Miners/validators will prioritize the higher-gas transaction, so the attacker's transaction gets mined first

### The Vulnerable Pattern

```solidity
pragma solidity ^0.8.0;

contract Vault {
    bytes32 private secretHash;
    bool public isUnlocked;
    string public flag;

    constructor(bytes32 _secretHash, string memory _flag) {
        secretHash = _secretHash;
        flag = _flag;
    }

    function unlock(string memory _password) public {
        require(!isUnlocked, "Already unlocked");
        require(keccak256(abi.encodePacked(_password)) == secretHash, "Wrong password");
        isUnlocked = true;
        // Flag is now accessible
    }

    function getFlag() public view returns (string memory) {
        require(isUnlocked, "Vault is locked");
        return flag;
    }
}
```

The vulnerability is that the password is sent in **plaintext** in the transaction's calldata. Even though `secretHash` is stored on-chain, the password itself is visible to anyone monitoring pending transactions.

### Attack Strategy

The challenge likely works in one of these ways:

1. **Mempool monitoring**: A bot or other user submits a transaction with the correct password. You need to watch the mempool, decode the pending transaction, extract the password, and front-run it.

2. **Pending transaction inspection**: The challenge infrastructure submits periodic unlock attempts. You subscribe to pending transactions, filter for ones targeting the vault contract, decode the ABI-encoded calldata, and extract the password argument.

3. **Direct calldata reading**: A transaction has already been submitted (possibly still pending or in a recent block). You need to inspect its input data to recover the password.

### Key Techniques

- **`web3.eth.subscribe('pendingTransactions')`** or **`web3.eth.get_block('pending')`**: Monitor the mempool for new pending transactions
- **ABI decoding**: Parse the calldata of a pending transaction to extract function arguments
- **Gas price manipulation**: Submit your transaction with a higher gas price to ensure it gets mined first
- **`txpool_content`** or **`txpool_inspect`** RPC methods: Enumerate all pending transactions in the node's transaction pool

## Solution

1. **Connect to the RPC endpoint** provided by the challenge. You will typically receive an RPC URL, a contract address for the vault, and your player's private key.

2. **Monitor the mempool** for pending transactions targeting the vault contract. Use `eth_subscribe` with `newPendingTransactions` or poll with `txpool_content`.

3. **Decode the calldata** of any transaction targeting the vault. The function signature for `unlock(string)` is `0x7e4ac72a` (first 4 bytes of `keccak256("unlock(string)")`). The remaining bytes contain the ABI-encoded password string.

4. **Extract the password** from the decoded calldata.

5. **Submit your own `unlock()` transaction** with the same password but a **higher gas price** to front-run the original sender.

6. **Call `getFlag()`** after your unlock transaction is confirmed to retrieve the flag.

Alternatively, if the challenge uses a simpler model where the password is already visible in a past transaction or in the pending pool at the time you connect, you can simply read the transaction data and use the password directly.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
