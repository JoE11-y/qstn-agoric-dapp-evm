#!/bin/bash

# This script is used to verify the QuizzlerGMP contract
# By verifying the contract, we can attach its source code to a public
# explorer (e.g snowtrace, basescan, etherscan) which can be used later by a debugger (e.g tenderly) to make
# a human readable stack trace. This is helpful for figuring out why a
# certain contract call failed.
#
# For verification we will need:
# 1. The address of the contract
# 2. The network where the contract is deployed
# The constructor arguments (gateway, gasService, chainName) are automatically
# fetched from the network configuration


# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/network-config.sh"

if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <network> <contract_address>"
    echo "Supported networks:"
    echo "  Mainnets: avax, arb, opt, pol"
    echo "  Testnets: eth, eth-sepolia, fuji, base, base-sepolia, opt-sepolia, arb-sepolia"
    echo ""
    echo "Example:"
    echo "  $0 fuji 0x123..."
    exit 0
fi

network=$1
contract_address=$2

get_network_config "$network"

echo "Verifying QuizzlerGMP contract at: $contract_address"
echo "Network: $network"
echo "Gateway: $GATEWAY"
echo "Gas Service: $GAS_SERVICE"
echo "Chain Name: $CHAIN_NAME"

npx hardhat verify --network "$network" \
    "$contract_address" \
    "$GATEWAY" \
    "$GAS_SERVICE" \
    "$CHAIN_NAME" \
    --contract "contracts/QuizzlerGMP.sol:QuizzlerGMP"
