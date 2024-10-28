// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Test {
    uint256 public count;

    event increased(uint256 oldCount, uint256 newCount);

    constructor() payable {}

    function increase() external payable {
        count++;

        emit increased(count - 1, count);
    }

    function get_blockhash(uint64 n) external view returns (bytes32) {
        return blockhash(n);
    }

    function get_msg_sender() external view returns (address) {
        return msg.sender;
    }
}
