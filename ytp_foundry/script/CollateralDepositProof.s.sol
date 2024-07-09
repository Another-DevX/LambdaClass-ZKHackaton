// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {CollateralDepositProof} from "../src/CollateralDepositProof.sol";

contract CollateralDepositProofDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        CollateralDepositProof checker = new CollateralDepositProof(
            vm.envAddress("TURBO_SWAP_PROXY_ADDRESS")
        );

        console2.log(
            "Deployed CollateralDepositProof at address: %s",
            address(checker)
        );

        vm.stopBroadcast();
    }
}