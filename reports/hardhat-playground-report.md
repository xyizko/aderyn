# Aderyn Analysis Report

This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities.
# Table of Contents

- [Summary](#summary)
  - [Files Summary](#files-summary)
  - [Files Details](#files-details)
  - [Issue Summary](#issue-summary)
- [High Issues](#high-issues)
  - [H-1: Using `delegatecall` in loop](#h-1-using-delegatecall-in-loop)
  - [H-2: `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`](#h-2-abiencodepacked-should-not-be-used-with-dynamic-types-when-passing-the-result-to-a-hash-function-such-as-keccak256)
  - [H-3: Uninitialized State Variables](#h-3-uninitialized-state-variables)
  - [H-4: Delegatecall made by the function without checks on any adress.](#h-4-delegatecall-made-by-the-function-without-checks-on-any-adress)
- [Low Issues](#low-issues)
  - [L-1: `ecrecover` is susceptible to signature malleability](#l-1-ecrecover-is-susceptible-to-signature-malleability)
  - [L-2: Unsafe ERC20 Operations should not be used](#l-2-unsafe-erc20-operations-should-not-be-used)
  - [L-3: Solidity pragma should be specific, not wide](#l-3-solidity-pragma-should-be-specific-not-wide)
  - [L-4: Missing checks for `address(0)` when assigning values to address state variables](#l-4-missing-checks-for-address0-when-assigning-values-to-address-state-variables)
  - [L-5: `public` functions not used internally could be marked `external`](#l-5-public-functions-not-used-internally-could-be-marked-external)
  - [L-6: Event is missing `indexed` fields](#l-6-event-is-missing-indexed-fields)
  - [L-7: PUSH0 is not supported by all chains](#l-7-push0-is-not-supported-by-all-chains)
  - [L-8: Contract still has TODOs](#l-8-contract-still-has-todos)
  - [L-9: Potentially unused `private` / `internal` state variables found.](#l-9-potentially-unused-private--internal-state-variables-found)


# Summary

## Files Summary

| Key | Value |
| --- | --- |
| .sol Files | 7 |
| Total nSLOC | 140 |


## Files Details

| Filepath | nSLOC |
| --- | --- |
| contracts/Counter.sol | 20 |
| contracts/ExtendedInheritance.sol | 17 |
| contracts/IContractInheritance.sol | 4 |
| contracts/InheritanceBase.sol | 8 |
| contracts/KeccakContract.sol | 21 |
| contracts/Lock.sol | 20 |
| contracts/StateVariables.sol | 50 |
| **Total** | **140** |


## Issue Summary

| Category | No. of Issues |
| --- | --- |
| High | 4 |
| Low | 9 |


# High Issues

## H-1: Using `delegatecall` in loop

When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.

<details><summary>1 Found Instances</summary>


- Found in contracts/ExtendedInheritance.sol [Line: 16](../tests/hardhat-js-playground/contracts/ExtendedInheritance.sol#L16)

	```solidity
	            target.delegatecall(abi.encodeWithSignature("doSomething(uint256)", i));
	```

</details>



## H-2: `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`

Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).
If all arguments are strings and or bytes, `bytes.concat()` should be used instead.

<details><summary>3 Found Instances</summary>


- Found in contracts/KeccakContract.sol [Line: 18](../tests/hardhat-js-playground/contracts/KeccakContract.sol#L18)

	```solidity
	        return keccak256(abi.encodePacked(a, b));
	```

- Found in contracts/KeccakContract.sol [Line: 22](../tests/hardhat-js-playground/contracts/KeccakContract.sol#L22)

	```solidity
	        return keccak256(abi.encodePacked(a, b));
	```

- Found in contracts/KeccakContract.sol [Line: 26](../tests/hardhat-js-playground/contracts/KeccakContract.sol#L26)

	```solidity
	        return keccak256(abi.encodePacked(a, b));
	```

</details>



## H-3: Uninitialized State Variables

Solidity does initialize variables by default when you declare them, however it's good practice to explicitly declare an initial value. For example, if you transfer money to an address we must make sure that the address has been initialized.

<details><summary>3 Found Instances</summary>


- Found in contracts/StateVariables.sol [Line: 8](../tests/hardhat-js-playground/contracts/StateVariables.sol#L8)

	```solidity
	    uint256 private staticPrivateNumber;
	```

- Found in contracts/StateVariables.sol [Line: 9](../tests/hardhat-js-playground/contracts/StateVariables.sol#L9)

	```solidity
	    uint256 internal staticInternalNumber;
	```

- Found in contracts/StateVariables.sol [Line: 10](../tests/hardhat-js-playground/contracts/StateVariables.sol#L10)

	```solidity
	    uint256 public staticPublicNumber;
	```

</details>



## H-4: Delegatecall made by the function without checks on any adress.

Introduce checks on the address

<details><summary>1 Found Instances</summary>


- Found in contracts/ExtendedInheritance.sol [Line: 14](../tests/hardhat-js-playground/contracts/ExtendedInheritance.sol#L14)

	```solidity
	    function doSomethingElse(address target) external {
	```

</details>



# Low Issues

## L-1: `ecrecover` is susceptible to signature malleability

The `ecrecover` function is susceptible to signature malleability. This means that the same message can be signed in multiple ways, allowing an attacker to change the message signature without invalidating it. This can lead to unexpected behavior in smart contracts, such as the loss of funds or the ability to bypass access control. Consider using OpenZeppelin's ECDSA library instead of the built-in function.

<details><summary>1 Found Instances</summary>


- Found in contracts/ExtendedInheritance.sol [Line: 21](../tests/hardhat-js-playground/contracts/ExtendedInheritance.sol#L21)

	```solidity
	        return ecrecover(theHash, v, r, s);
	```

</details>



## L-2: Unsafe ERC20 Operations should not be used

ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.

<details><summary>1 Found Instances</summary>


- Found in contracts/Lock.sol [Line: 32](../tests/hardhat-js-playground/contracts/Lock.sol#L32)

	```solidity
	        owner.transfer(address(this).balance);
	```

</details>



## L-3: Solidity pragma should be specific, not wide

Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`

<details><summary>4 Found Instances</summary>


- Found in contracts/Counter.sol [Line: 2](../tests/hardhat-js-playground/contracts/Counter.sol#L2)

	```solidity
	pragma solidity ^0.8.13;
	```

- Found in contracts/IContractInheritance.sol [Line: 2](../tests/hardhat-js-playground/contracts/IContractInheritance.sol#L2)

	```solidity
	pragma solidity >=0.8.0;
	```

- Found in contracts/InheritanceBase.sol [Line: 2](../tests/hardhat-js-playground/contracts/InheritanceBase.sol#L2)

	```solidity
	pragma solidity ^0.8.0;
	```

- Found in contracts/Lock.sol [Line: 2](../tests/hardhat-js-playground/contracts/Lock.sol#L2)

	```solidity
	pragma solidity ^0.8.20;
	```

</details>



## L-4: Missing checks for `address(0)` when assigning values to address state variables

Check for `address(0)` when assigning values to address state variables.

<details><summary>1 Found Instances</summary>


- Found in contracts/StateVariables.sol [Line: 58](../tests/hardhat-js-playground/contracts/StateVariables.sol#L58)

	```solidity
	        addr = newAddr;
	```

</details>



## L-5: `public` functions not used internally could be marked `external`

Instead of marking a function as `public`, consider marking it as `external` if it is not used internally.

<details><summary>7 Found Instances</summary>


- Found in contracts/Counter.sol [Line: 7](../tests/hardhat-js-playground/contracts/Counter.sol#L7)

	```solidity
	    function setNumber(uint256 newNumber) public {
	```

- Found in contracts/Lock.sol [Line: 23](../tests/hardhat-js-playground/contracts/Lock.sol#L23)

	```solidity
	    function withdraw() public {
	```

- Found in contracts/StateVariables.sol [Line: 47](../tests/hardhat-js-playground/contracts/StateVariables.sol#L47)

	```solidity
	    function setAddrNoZeroError(address newAddr) public {
	```

- Found in contracts/StateVariables.sol [Line: 52](../tests/hardhat-js-playground/contracts/StateVariables.sol#L52)

	```solidity
	    function setAddrNoZeroRequire(address newAddr) public {
	```

- Found in contracts/StateVariables.sol [Line: 57](../tests/hardhat-js-playground/contracts/StateVariables.sol#L57)

	```solidity
	    function setAddrNoCheck(address newAddr) public {
	```

- Found in contracts/StateVariables.sol [Line: 61](../tests/hardhat-js-playground/contracts/StateVariables.sol#L61)

	```solidity
	    function setEmptyAlteredNumbers(uint256 _emptyAlteredPrivateNumber, uint256 _emptyAlteredInternalNumber, uint256 _emptyAlteredPublicNumber) public {
	```

- Found in contracts/StateVariables.sol [Line: 67](../tests/hardhat-js-playground/contracts/StateVariables.sol#L67)

	```solidity
	    function setNonEmptyAlteredNumbers(uint256 _nonEmptyAlteredPrivateNumber, uint256 _nonEmptyAlteredInternalNumber, uint256 _nonEmptyAlteredPublicNumber) public {
	```

</details>



## L-6: Event is missing `indexed` fields

Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.

<details><summary>3 Found Instances</summary>


- Found in contracts/ExtendedInheritance.sol [Line: 7](../tests/hardhat-js-playground/contracts/ExtendedInheritance.sol#L7)

	```solidity
	    event DoSomethingElse(uint256 somethingElse);
	```

- Found in contracts/InheritanceBase.sol [Line: 7](../tests/hardhat-js-playground/contracts/InheritanceBase.sol#L7)

	```solidity
	    event Do(uint256 something);
	```

- Found in contracts/Lock.sol [Line: 11](../tests/hardhat-js-playground/contracts/Lock.sol#L11)

	```solidity
	    event Withdrawal(uint amount, uint when);
	```

</details>



## L-7: PUSH0 is not supported by all chains

Solc compiler version 0.8.20 switches the default target EVM version to Shanghai, which means that the generated bytecode will include PUSH0 opcodes. Be sure to select the appropriate EVM version in case you intend to deploy on a chain other than mainnet like L2 chains that may not support PUSH0, otherwise deployment of your contracts will fail.

<details><summary>7 Found Instances</summary>


- Found in contracts/Counter.sol [Line: 2](../tests/hardhat-js-playground/contracts/Counter.sol#L2)

	```solidity
	pragma solidity ^0.8.13;
	```

- Found in contracts/ExtendedInheritance.sol [Line: 2](../tests/hardhat-js-playground/contracts/ExtendedInheritance.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

- Found in contracts/IContractInheritance.sol [Line: 2](../tests/hardhat-js-playground/contracts/IContractInheritance.sol#L2)

	```solidity
	pragma solidity >=0.8.0;
	```

- Found in contracts/InheritanceBase.sol [Line: 2](../tests/hardhat-js-playground/contracts/InheritanceBase.sol#L2)

	```solidity
	pragma solidity ^0.8.0;
	```

- Found in contracts/KeccakContract.sol [Line: 2](../tests/hardhat-js-playground/contracts/KeccakContract.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

- Found in contracts/Lock.sol [Line: 2](../tests/hardhat-js-playground/contracts/Lock.sol#L2)

	```solidity
	pragma solidity ^0.8.20;
	```

- Found in contracts/StateVariables.sol [Line: 2](../tests/hardhat-js-playground/contracts/StateVariables.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

</details>



## L-8: Contract still has TODOs

Contract contains comments with TODOS

<details><summary>1 Found Instances</summary>


- Found in contracts/Counter.sol [Line: 4](../tests/hardhat-js-playground/contracts/Counter.sol#L4)

	```solidity
	contract Counter {
	```

</details>



## L-9: Potentially unused `private` / `internal` state variables found.

State variable appears to be unused. No analysis has been performed to see if any inilne assembly references it. So if that's not the case, consider removing this unused variable.

<details><summary>6 Found Instances</summary>


- Found in contracts/StateVariables.sol [Line: 8](../tests/hardhat-js-playground/contracts/StateVariables.sol#L8)

	```solidity
	    uint256 private staticPrivateNumber;
	```

- Found in contracts/StateVariables.sol [Line: 9](../tests/hardhat-js-playground/contracts/StateVariables.sol#L9)

	```solidity
	    uint256 internal staticInternalNumber;
	```

- Found in contracts/StateVariables.sol [Line: 13](../tests/hardhat-js-playground/contracts/StateVariables.sol#L13)

	```solidity
	    uint256 private staticNonEmptyPrivateNumber = 1;
	```

- Found in contracts/StateVariables.sol [Line: 14](../tests/hardhat-js-playground/contracts/StateVariables.sol#L14)

	```solidity
	    uint256 internal staticNonEmptyInternalNumber = 2;
	```

- Found in contracts/StateVariables.sol [Line: 28](../tests/hardhat-js-playground/contracts/StateVariables.sol#L28)

	```solidity
	    uint256 private constant PRIVATE_CONSTANT = 1;
	```

- Found in contracts/StateVariables.sol [Line: 29](../tests/hardhat-js-playground/contracts/StateVariables.sol#L29)

	```solidity
	    uint256 internal constant INTERNAL_CONSTANT = 2;
	```

</details>


