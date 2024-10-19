//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract CallTest {
  event Advance();
  function reflect() public view returns (address) {
    return msg.sender;
  }
  function mark() public view returns (uint) {
    return block.number;
  }
  function advance() public {
    emit Advance();
  }
}