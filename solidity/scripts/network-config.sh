#!/bin/bash

# Network configuration for contract deployment and verification
# This script provides gateway and gas service addresses for different networks

get_network_config() {
    local network=$1

    case $network in
        fuji)
            CHAIN_NAME='Avalanche'
            GATEWAY='0xC249632c2D40b9001FE907806902f63038B737Ab'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        base)
            CHAIN_NAME='Base'
            GATEWAY='0xB8Cd93C83A974649D76B1c19f311f639e62272BC'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        eth)
            CHAIN_NAME='Ethereum'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        eth-sepolia)
            CHAIN_NAME='Ethereum'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        base-sepolia)
            CHAIN_NAME='Base'
            GATEWAY='0xB8Cd93C83A974649D76B1c19f311f639e62272BC'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        avax)
            CHAIN_NAME='Avalanche'
            GATEWAY='0x5029C0EFf6C34351a0CEc334542cDb22c7928f78'
            GAS_SERVICE='0x2d5d7d31F671F86C782533cc367F14109a082712'
            ;;
        arb)
            CHAIN_NAME='Arbitrum'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0x2d5d7d31F671F86C782533cc367F14109a082712'
            ;;
        arb-sepolia)
            CHAIN_NAME='Arbitrum'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        opt)
            CHAIN_NAME='Optimism'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0x2d5d7d31F671F86C782533cc367F14109a082712'
            ;;
        opt-sepolia)
            CHAIN_NAME='Optimism'
            GATEWAY='0xe432150cce91c13a887f7D836923d5597adD8E31'
            GAS_SERVICE='0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6'
            ;;
        pol)
            CHAIN_NAME='Polygon'
            GATEWAY='0x6f015F16De9fC8791b234eF68D486d2bF203FBA8'
            GAS_SERVICE='0x2d5d7d31F671F86C782533cc367F14109a082712'
            ;;
        *)
            echo "Invalid network specified: $network"
            echo "Supported networks:"
            echo "  Mainnets: avax, arb, opt, pol"
            echo "  Testnets: eth, eth-sepolia, fuji, base, base-sepolia, opt-sepolia, arb-sepolia"
            exit 1
            ;;
    esac

    export CHAIN_NAME
    export GATEWAY
    export GAS_SERVICE
}
