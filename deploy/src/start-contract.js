/// <reference types="@agoric/vats/src/core/types-ambient"/>

import {
  deeplyFulfilledObject,
  makeTracer,
  NonNullish,
} from '@agoric/internal';
import { E } from '@endo/far';

/**
 * @import {Issuer} from '@agoric/ertp';
 * @import {Installation, Instance} from '@agoric/zoe/src/zoeService/utils.js';
 * @import {CosmosChainInfo, Denom, DenomDetail} from '@agoric/orchestration';
 * @import {start as StartFn} from 'contract/src/qstn.router.js';
 */

const trace = makeTracer('start qstn contract', true);

/**
 * @param {BootstrapPowers & {
 *   installation: {
 *     consume: {
 *       qstnRouter: Installation<StartFn>;
 *     };
 *   };
 *   instance: {
 *     produce: {
 *       qstnRouter: Producer<Instance<StartFn>>
 *     };
 *   };
 *   issuer: {
 *     consume: {
 *       BLD: Issuer<'nat'>;
 *       IST: Issuer<'nat'>;
 *     };
 *   };
 * }} powers
 * @param {{
 *   options: {
 *     chainInfo: Record<string, CosmosChainInfo>;
 *     assetInfo: [Denom, DenomDetail & { brandKey?: string }][];
 *   };
 * }} config
 */
export const startQstnRouter = async (
  {
    consume: {
      agoricNames,
      board,
      chainStorage,
      chainTimerService,
      cosmosInterchainService,
      localchain,
      startUpgradable,
    },
    installation: {
      consume: { qstnRouter },
    },
    instance: {
      produce: { qstnRouter: produceInstance },
    },
    issuer: {
      consume: { BLD, IST },
    },
  },
  { options: { chainInfo, assetInfo } },
) => {
  trace(startQstnRouter.name);

  const marshaller = await E(board).getReadonlyMarshaller();

  trace('Setting privateArgs');

  const privateArgs = await deeplyFulfilledObject(
    harden({
      agoricNames,
      localchain,
      marshaller,
      orchestrationService: cosmosInterchainService,
      storageNode: E(NonNullish(await chainStorage)).makeChildNode(
        'qstnRouter',
      ),
      timerService: chainTimerService,
      chainInfo,
      assetInfo,
    }),
  );

  /** @param {() => Promise<Issuer>} p */
  const safeFulfill = async (p) =>
    E.when(
      p(),
      (i) => i,
      () => undefined,
    );

  const axlIssuer = await safeFulfill(() =>
    E(agoricNames).lookup('issuer', 'AXL'),
  );

  const atomIssuer = await safeFulfill(() =>
    E(agoricNames).lookup('issuer', 'ATOM'),
  );

  // const ntrnIssuer = await safeFulfill(() =>
  //   E(agoricNames).lookup('issuer', 'NTRN'),
  // );

  const issuerKeywordRecord = harden({
    BLD: await BLD,
    IST: await IST,
    ...(axlIssuer && { AXL: axlIssuer }),
    ...(atomIssuer && { ATOM: atomIssuer }),
    // ...(ntrnIssuer && { untrn: ntrnIssuer }),
  });
  trace('issuerKeywordRecord', issuerKeywordRecord);

  trace('Starting contract instance');
  const { instance } = await E(startUpgradable)({
    label: 'qstnRouter',
    installation: qstnRouter,
    issuerKeywordRecord,
    privateArgs,
  });
  produceInstance.resolve(instance);
  trace('done');
};
harden(startQstnRouter);

export const getManifest = ({ restoreRef }, { installationRef, options }) => {
  return {
    manifest: {
      [startQstnRouter.name]: {
        consume: {
          agoricNames: true,
          board: true,
          chainTimerService: true,
          chainStorage: true,
          cosmosInterchainService: true,
          localchain: true,
          startUpgradable: true,
        },
        installation: {
          consume: { qstnRouter: true },
        },
        instance: {
          produce: { qstnRouter: true },
        },
        issuer: {
          consume: { BLD: true, IST: true, AXL: true, ATOM: true },
        },
      },
    },
    installations: {
      qstnRouter: restoreRef(installationRef),
    },
    options,
  };
};
