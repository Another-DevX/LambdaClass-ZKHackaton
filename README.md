# Yo Te Presto Protocol (YTP Protocol)

## Problem

Currently, there is an issue with fragmented liquidity affecting both the user and the protocol. This problem will continue to grow as the number of rollup options increases. Additionally, there is a security concern for users when using bridges, where assets are merely locked on one chain and "created" on another (mint & burn).

## Our Solution

We are creating a lending protocol that generates storage proofs of users' collateral deposits. These proofs can be validated on any Layer 2 (L2) network, enabling users to receive a loan of up to 80% of the value of their collateral.
This approach improves the user experience and eliminates the need for potentially hackable bridges.

## How We Do

![YTPP](./ytp-diagram.jpeg)

We are using [Herodotus - Turbo](https://docs.herodotus.dev/herodotus-docs/developers/turbo) in our [CollateralDepositProof Contract](./ytp_foundry/src/CollateralDepositProof.sol) that is deployed on ZkSync to check if the user that want to receive a loan makes a deposit on L1
if this proof is valid We give funds to the user.

When the user wants to recover their assets on Ethereum, they repay their loan. We generate a proof with Risc Zero that the user has completed the repayment, and post it to Aligned for validation. This allows the assets to be recovered on the Ethereum mainnet.
