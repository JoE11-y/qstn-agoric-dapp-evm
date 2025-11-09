use crate::error::ContractError;
use crate::msg::Manager;
use crate::state::{ManagerInfo, CONFIG, MANAGERS, USED_PROOF_TOKENS};
use cosmwasm_std::{
    Addr, BalanceResponse, BankQuery, Binary, Deps, DepsMut, Env, QuerierWrapper, QueryRequest,
    Uint128,
};
use neutron_std::types::cosmos::base::v1beta1::Coin as StdCoin;
use neutron_std::types::ibc::core::client::v1::Height;
use neutron_std::types::neutron::feerefunder::Fee;
use neutron_std::types::neutron::transfer::MsgTransfer;

const DEFAULT_TIMEOUT_HEIGHT: u64 = 10000000;

pub fn map_validate(
    receiver_prefix: &str,
    managers: &[Manager],
) -> Result<Vec<ManagerInfo>, ContractError> {
    managers
        .iter()
        .map(|admin| {
            let (_, validated_addr) = validate_account(receiver_prefix, &admin.addr)?;
            let pub_key = Binary::from_base64(&admin.pub_key)?;
            Ok(ManagerInfo {
                address: validated_addr,
                pub_key: pub_key,
                status: true,
            })
        })
        .collect()
}

pub fn validate_account(
    receiver_prefix: &str,
    receiver: &str,
) -> Result<(String, Addr), ContractError> {
    let Ok((prefix, _, _)) = bech32::decode(receiver) else {
        return Err(ContractError::InvalidAccount {
            receiver: receiver.to_string(),
        });
    };

    if prefix != receiver_prefix {
        return Err(ContractError::ExpectedAgoricAccount {
            receiver: receiver.to_string(),
        });
    }

    Ok((prefix, Addr::unchecked(receiver)))
}

pub fn auth_validations(
    deps: &mut DepsMut,
    env: &Env,
    token: String,
    message: Binary,
    pub_key: String,
    time_to_expire: u64,
    signature: String,
) -> Result<(), ContractError> {
    let pub_key = Binary::from_base64(&pub_key)?;
    let signature = Binary::from_base64(&signature)?;

    let current_block_time = env.block.time.seconds();
    if time_to_expire < current_block_time {
        return Err(ContractError::ProofExpired {});
    }

    let proof_token_exists = USED_PROOF_TOKENS
        .load(deps.storage, &token)
        .unwrap_or(false);

    if proof_token_exists {
        return Err(ContractError::TokenAlreadyUsed {});
    }

    // Verify pub key exists
    let managers = MANAGERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<(Addr, ManagerInfo)>, cosmwasm_std::StdError>>()?;

    let mut pub_key_exists = false;

    for (_addr, manager_info) in managers.iter() {
        if manager_info.pub_key == pub_key && manager_info.status {
            pub_key_exists = true;
            break;
        }
    }

    if !pub_key_exists {
        return Err(ContractError::InvalidSigner {});
    }

    let result = deps.api.ed25519_verify(&message, &signature, &pub_key)?;

    // mark proof token as used
    USED_PROOF_TOKENS.save(deps.storage, &token, &true)?;

    if !result {
        return Err(ContractError::InvalidMessageHash {});
    }

    Ok(())
}

pub fn check_is_contract_owner(deps: Deps, sender: Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();
    if config.owner != sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

pub fn query_contract_balance(
    querier: &QuerierWrapper,
    addr: &Addr,
    denom: &str,
) -> Result<Uint128, ContractError> {
    let resp: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: addr.to_string(),
        denom: denom.to_string(),
    }))?;
    Ok(resp.amount.amount)
}

pub fn create_ibc_transfer(
    deps: Deps,
    env: &Env,
    receiver: &str,
    denom: &str,
    amount: Uint128,
) -> Result<MsgTransfer, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    let _ = validate_account(&config.receiver_prefix, receiver)?;

    let ack_fee_amount = Uint128::from(1000u128);
    let timeout_fee_amount = Uint128::from(1000u128);

    let final_amount =
        Uint128::checked_sub(amount, ack_fee_amount.checked_add(timeout_fee_amount)?)?;

    let coin = StdCoin {
        denom: denom.to_string(),
        amount: final_amount.to_string(),
    };

    let fee = Fee {
        recv_fee: vec![],
        ack_fee: vec![create_coin(&denom, ack_fee_amount)],
        timeout_fee: vec![create_coin(&denom, timeout_fee_amount)],
    };

    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.channel_id,
        token: Some(coin),
        receiver: receiver.to_string(),
        timeout_height: Some(Height {
            revision_height: 2,
            revision_number: DEFAULT_TIMEOUT_HEIGHT,
        }),
        timeout_timestamp: env.block.time.plus_seconds(600).nanos(),
        fee: Some(fee),
        memo: "".to_string(),
        sender: env.contract.address.to_string(),
    };

    Ok(msg)
}

pub fn create_coin(denom: &str, amount: Uint128) -> StdCoin {
    StdCoin {
        denom: denom.to_string(),
        amount: amount.to_string(),
    }
}

pub fn ibc_message_event(context: &str) -> cosmwasm_std::Event {
    cosmwasm_std::Event::new("ibc_message_added").add_attribute("context", context)
}
