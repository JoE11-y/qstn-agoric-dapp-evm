use cosmwasm_std::{from_json, to_json_vec, Addr, Binary, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use neutron_std::types::neutron::feerefunder::Fee;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::SudoPayload;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ManagerInfo {
    pub address: Addr,
    pub pub_key: Binary,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub owner: Addr,
    pub receiver_prefix: String,
    pub channel_id: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// Managers
pub const MANAGERS: Map<&Addr, ManagerInfo> = Map::new("managers");

// Store IBC packet information
pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");

pub const IBC_SUDO_ID_RANGE_START: u64 = 1_000_000_000;

pub const IBC_SUDO_ID_RANGE_SIZE: u64 = 1_000;

pub const IBC_SUDO_ID_RANGE_END: u64 = IBC_SUDO_ID_RANGE_START + IBC_SUDO_ID_RANGE_SIZE;

pub const IBC_FEE: Item<Option<Fee>> = Item::new("ibc_fee");

pub const REPLY_QUEUE_ID: Map<u64, Vec<u8>> = Map::new("reply_queue_id");

const REPLY_ID: Item<u64> = Item::new("reply_id");

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SurveyInfo {
    pub survey_creator: Addr,
    pub participants_limit: u32,
    pub reward_amount: u128,
    pub participants_rewarded: u32,
    pub survey_hash: String,
    pub is_cancelled: bool,
    pub reward_denom: String,
}

pub const SURVEYS: Map<&str, SurveyInfo> = Map::new("surveys");

// Survey rewarded users
pub const SURVEY_REWARDED_USERS: Map<(&str, &Addr), bool> = Map::new("survey_rewarded_users");

// Used proof tokens
pub const USED_PROOF_TOKENS: Map<&String, bool> = Map::new("used_proof_tokens");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreateSurveyPayload<'a> {
    pub token: &'a str,
    pub time_to_expire: u64,
    pub owner: &'a str,
    pub survey_id: &'a str,
    pub participants_limit: u32,
    pub reward_amount: u128,
    pub survey_hash: &'a str,
    pub reward_denom: &'a str,
    pub domain: &'a str,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CancelSurveyPayload<'a> {
    pub token: &'a str,
    pub time_to_expire: u64,
    pub survey_id: &'a str,
    pub domain: &'a str,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PayRewardsPayload<'a> {
    pub token: &'a str,
    pub time_to_expire: u64,
    pub survey_id: Vec<String>,
    pub participants: Vec<String>,
    pub domain: &'a str,
}

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, (channel_id, seq_id), &to_json_vec(&payload)?)
}

pub fn read_sudo_payload(
    store: &dyn Storage,
    channel_id: String,
    seq_id: u64,
) -> StdResult<SudoPayload> {
    let data = SUDO_PAYLOAD.load(store, (channel_id, seq_id))?;
    from_json(Binary::new(data))
}

pub fn get_next_id(store: &mut dyn Storage) -> StdResult<u64> {
    let mut id = REPLY_ID.may_load(store)?.unwrap_or(IBC_SUDO_ID_RANGE_START);
    if id > IBC_SUDO_ID_RANGE_END {
        id = IBC_SUDO_ID_RANGE_START
    }
    REPLY_ID.save(store, &(id + 1))?;
    Ok(id)
}

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<u64> {
    let id = get_next_id(store)?;
    REPLY_QUEUE_ID.save(store, id, &to_json_vec(&payload)?)?;
    Ok(id)
}

pub fn read_reply_payload(store: &dyn Storage, id: u64) -> StdResult<SudoPayload> {
    let data = REPLY_QUEUE_ID.load(store, id)?;
    from_json(Binary::new(data))
}
