# Access_Control - picoCTF 2026

**Category:** Blockchain
**Points:** 200

## Challenge Description

We've created a simple contract to store a secret flag. But you currently are not the owner of the contract... Only the owner can access the flag. Can you figure out how to become the owner?

## Approach

This is a classic **smart contract access control vulnerability** challenge. The contract stores a secret flag and restricts access to the "owner," but the access control mechanism has a flaw that allows anyone to become the owner.

### Common Blockchain Access Control Vulnerabilities

1. **Unprotected state-changing functions**: A function that sets the owner lacks an access modifier (e.g., `onlyOwner`), allowing anyone to call it.
2. **tx.origin vs msg.sender confusion**: Using `tx.origin` for authentication instead of `msg.sender` enables phishing attacks.
3. **Public visibility on sensitive functions**: Functions intended to be internal/private are accidentally left public.
4. **Initialization race condition**: The constructor or initializer can be called by anyone, or there is a separate `initialize()` function that was never called (or can be called again).
5. **Storage slot reading**: Even "private" variables in Solidity are readable on-chain -- nothing on the blockchain is truly private.

### Typical Contract Pattern

```solidity
pragma solidity ^0.8.0;

contract AccessControl {
    address public owner;
    string private flag;

    constructor(string memory _flag) {
        owner = msg.sender;
        flag = _flag;
    }

    // VULNERABILITY: No access control on this function!
    function setOwner(address _newOwner) public {
        owner = _newOwner;
    }

    // Or alternatively, a flawed modifier:
    // function setOwner(address _newOwner) public {
    //     require(tx.origin == owner);  // vulnerable to phishing
    //     owner = _newOwner;
    // }

    function getFlag() public view returns (string memory) {
        require(msg.sender == owner, "Not the owner!");
        return flag;
    }
}
```

### Attack Vectors

**Approach A -- Unprotected `setOwner` / `changeOwner`**: Simply call the function that changes ownership with your own address.

**Approach B -- Read storage directly**: Even if you cannot become the owner, the flag stored as a "private" variable can be read directly from the blockchain storage slot using `web3.eth.getStorageAt()`.

**Approach C -- Unprotected initializer**: Call an `initialize()` or `init()` function that was left unprotected.

## Solution

1. **Connect to the provided RPC endpoint** using the challenge credentials (typically an RPC URL, contract address, and a funded wallet private key).
2. **Read the contract ABI or source code** (usually provided or discoverable via the challenge).
3. **Identify the vulnerability**: Look for an unprotected function that modifies the `owner` state variable.
4. **Call the vulnerable function** to set yourself as the owner.
5. **Call `getFlag()`** to retrieve the flag.
6. Alternatively, **read the storage slot** directly to bypass access control entirely.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
