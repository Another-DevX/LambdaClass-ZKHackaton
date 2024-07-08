// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.19;

import {ITurboSwap, HeaderProperty} from "turbo-zksync-demo/src/turbo/interfaces/ITurboSwap.sol";

contract CollateralDeposit {

    ITurboSwap public turboSwap;
    mapping(uint256 => mapping(uint256 => mapping(HeaderProperty => bytes32)))
        public headerProperties;

    mapping(uint256 => mapping(uint256 => mapping(HeaderProperty => bool)))
        public isProven;

    // error NotProven();

    constructor(address turboSwapProxy) { // 0x7A5D2661Ed1614C853D7001F10dfB34cf74572f3
        turboSwap = ITurboSwap(turboSwapProxy);
    }

    function collateralProven(uint256 blockNumber)
        external
    {

        bytes32 depositProof = turboSwap.headers(11155111, blockNumber, HeaderProperty.TRANSACTIONS_ROOT);
        if (depositProof != bytes32(0)) {
            revert();
        }

        // Store the proven property to this contract storage
        headerProperties[11155111][blockNumber][ HeaderProperty.TRANSACTIONS_ROOT] = depositProof;
        isProven[11155111][blockNumber][ HeaderProperty.TRANSACTIONS_ROOT] = true;
        
    }
}