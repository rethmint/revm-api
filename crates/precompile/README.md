### REVM Precompiles - Ethereum compatible precompiled contracts

Used inside REVM and passes all eth/tests. It supports all Ethereum hardforks.

# Precompile Contract Development Guidelines

This document provides guidelines for developing precompile contracts to be embedded on the REVM platform.

The `contract` directory should include all the necessary and helpful contracts and features, such as secp2561 signing. These Solidity (`sol`) files will be compiled and stored in the `binaries` directory.

Please refer to the following guidelines to ensure smooth development and integration of precompile contracts.

1. **Contract Directory Structure**: Organize the contract directory in a logical manner, grouping related contracts and features together.

2. **Solidity Files**: Write the necessary Solidity files for the precompile contracts, ensuring proper documentation and adherence to best practices.

3. **Compilation**: Use a suitable Solidity compiler to compile the contracts into bytecode. Store the compiled binaries in the `binaries` directory.

4. **Testing**: Thoroughly test the precompile contracts to ensure their correctness and functionality. Write comprehensive test cases and consider edge cases.

5. **Documentation**: Provide clear and concise documentation for each precompile contract, explaining its purpose, usage, and any specific considerations.

6. **Integration**: Integrate the precompile contracts into the REVM platform, ensuring proper dependencies and compatibility.

7. **Security Considerations**: Pay attention to security best practices when developing precompile contracts. Perform code reviews and security audits to identify and mitigate potential vulnerabilities.

By following these guidelines, you can develop robust and reliable precompile contracts for embedding on the REVM platform.

// TODO: fill the processes that build the precompile contract 