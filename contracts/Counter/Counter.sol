// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Counter {
    uint256 public count;

    event increased(uint256 oldCount, uint256 newCount);

    constructor() payable {
    }

    function increase() external payable {
        count++;

        emit increased(count - 1, count);
    }
}
