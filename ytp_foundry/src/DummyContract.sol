// SPDX-License-Identifier: MIT
pragma solidity >=0.8.2 <0.9.0;

contract DummyContract {
    mapping(address => mapping(uint256 => uint256)) public lends;
    mapping(address => uint256) currentLendId;

    function getLend(address user, uint256 lendId)
        public
        view
        returns (uint256)
    {
        return  lends[user][lendId];
    }

    function createLend(uint256 amount) public {
        uint256 lendId = currentLendId[msg.sender];
        lends[msg.sender][lendId + 1] = amount;
        currentLendId[msg.sender] += 1;
    }
}
