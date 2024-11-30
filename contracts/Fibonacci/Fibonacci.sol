// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Fibonacci {
    function fibonacci(uint number) public returns(uint result) {
        if (number == 0) return 0;
        else if (number == 1) return 1;
        else return Fibonacci.fibonacci(number - 1) + Fibonacci.fibonacci(number - 2);
    }
}
