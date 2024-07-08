// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract CollateralDeposit {

    address constant collateralToken = 0xa0289cBbEB673b8787C9C61Bf03914068A033651;
    mapping(address => uint256) collateralValue;

    event CollateralDeposit(
        address indexed sender, 
        uint256 amount, 
        uint256 totalOfCollateralDeposited
    );

    event Withdrawal(
        address indexed sender, 
        uint256 amount, 
        uint256 totalOfCollateralDeposited
    );

    function depositCollateral(uint256 _amount)
        external
    {

        if (_amount == 0) {
            revert();
        }
        IERC20(collateralToken).transferFrom(msg.sender, address(this), _amount);

        collateralValue[msg.sender] += _amount;
        emit CollateralDeposit(msg.sender, _amount, collateralValue[msg.sender]);
    }

    function getBalance(address user) public view returns (uint){
        return collateralValue[user];
    }
}
