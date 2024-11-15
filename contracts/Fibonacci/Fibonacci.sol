// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Fibonacci{
    function fibonacci(uint256 i) public pure returns (uint256) {
        uint256 a = 0;
        uint256 b = 1;
        
        while (i != 0) {
            uint256 tmp = a;
            a = b;
            b = b + tmp;
            i -= 1;
        }
        
        return a;
    }
}
