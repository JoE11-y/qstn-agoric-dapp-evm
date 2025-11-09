// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

import { buildModule } from '@nomicfoundation/hardhat-ignition/modules';
import { config } from 'dotenv';

config();

const { GATEWAY_CONTRACT, GAS_SERVICE, CHAIN_NAME } = process.env;

if (!GATEWAY_CONTRACT || !GAS_SERVICE || !CHAIN_NAME) {
  throw new Error(
    'GATEWAY_CONTRACT, GAS_SERVICE, or CHAIN_NAME is not defined',
  );
}

console.log(`GATEWAY_CONTRACT: ${GATEWAY_CONTRACT}`);
console.log(`CHAIN_NAME: ${CHAIN_NAME}`);
console.log(`GAS_SERVICE: ${GAS_SERVICE}`);

export default buildModule('AxelarProxyModule', (m) => {
  const gateway = m.getParameter('gateway_', GATEWAY_CONTRACT);
  const gasContract = m.getParameter('gasService_', GAS_SERVICE);
  const chainName = m.getParameter('chainName_', CHAIN_NAME);
  const QuizzlerGMP = m.contract('QuizzlerGMP', [
    gateway,
    gasContract,
    chainName,
  ]);
  return { QuizzlerGMP };
});
