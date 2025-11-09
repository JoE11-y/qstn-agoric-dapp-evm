import { HardhatUserConfig } from 'hardhat/config';
import '@nomicfoundation/hardhat-toolbox';
import { config as envConfig } from 'dotenv';

envConfig();

const { PRIVATE_KEY } = process.env;

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      {
        version: '0.8.20',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
      {
        version: '0.8.9',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
  },
  defaultNetwork: 'hardhat',
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY,
    customChains: [
      {
        network: 'base',
        chainId: 84532,
        urls: {
          apiURL: 'https://api-sepolia.base.org/api',
          browserURL: 'https://sepolia.base.org',
        },
      },
      {
        network: 'fuji',
        chainId: 43113,
        urls: {
          apiURL: 'https://api.routescan.io/v2/network/testnet/evm',
          browserURL: 'https://testnet.snowtrace.io',
        },
      },
      {
        network: 'eth',
        chainId: 11155111,
        urls: {
          apiURL: 'https://api-sepolia.etherscan.io/api',
          browserURL: 'https://sepolia.etherscan.io',
        },
      },
    ],
  },
  networks: {
    hardhat: {
      chainId: 31337,
    },
    localhost: {
      url: 'http://127.0.0.1:8545',
      chainId: 31337,
    },
    fuji: {
      url: 'https://api.avax-test.network/ext/bc/C/rpc',
      gasPrice: 225000000000,
      chainId: 43113,
      accounts: PRIVATE_KEY ? [`0x${PRIVATE_KEY}`] : [],
    },
    base: {
      url: 'https://sepolia.base.org/',
      gasPrice: 225000000000,
      chainId: 84532,
      accounts: PRIVATE_KEY ? [`0x${PRIVATE_KEY}`] : [],
    },
    eth: {
      url: 'https://sepolia.infura.io/v3/YOUR_INFURA_PROJECT_ID',
      gasPrice: 225000000000,
      chainId: 11155111,
      accounts: PRIVATE_KEY ? [`0x${PRIVATE_KEY}`] : [],
    },
  },
  mocha: {
    timeout: 20000,
  },
  sourcify: {
    enabled: true,
  },
};

export default config;
//
