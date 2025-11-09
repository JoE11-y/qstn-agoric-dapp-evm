const {
  defaultAxelarChainInfo,
  AxelarRelayerService,
} = require('./agoric-to-axelar-local/packages/axelar-local-dev-cosmos/dist/index.js');
const {
  evmRelayer,
  createNetwork,
  deployContract,
  relay,
  RelayerType,
} = require('./agoric-to-axelar-local/packages/axelar-local-dev/dist/index.js');

const runRelay = async () => {
  const axelarRelayer = await AxelarRelayerService.create(
    defaultAxelarChainInfo,
  );

  const QuizzlerV2 = require('../solidity/artifacts/contracts/QuizzlerV2.sol/QuizzlerV2.json');

  const ethereumNetwork = await createNetwork({ name: 'Ethereum' });

  // Deploy factory contract
  const quizzlerContract = await deployContract(
    ethereumNetwork.userWallets[0],
    QuizzlerV2,
    [
      ethereumNetwork.gateway.address,
      // ethereumNetwork.gasService.address,
      'Sender',
      'Ethereum',
    ],
  );
  console.log(
    'Quizzler contract deployed at address:',
    quizzlerContract.address,
  );

  // Deploy tokens
  const tokenContract = await ethereumNetwork.deployToken(
    'USDC',
    'aUSDC',
    6,
    BigInt(100_000e6),
  );
  console.log('Token contract deployed at address:', tokenContract.address);

  evmRelayer.setRelayer(RelayerType.Agoric, axelarRelayer);

  while (true) {
    await relay({
      agoric: axelarRelayer,
    });

    await relay({
      evm: evmRelayer,
    });
  }
};

runRelay();
