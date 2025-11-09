use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct Manager {
    pub addr: String,
    pub pub_key: String,
}

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub managers: Vec<Manager>,
    pub receiver_prefix: String,
    pub channel_id: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    SetManagers {
        managers: String,
        pub_key: String,
        status: bool,
    },
    CreateSurvey {
        signature: String,
        token: String,
        time_to_expire: u64,
        owner: String,
        survey_id: String,
        participants_limit: u32,
        reward_denom: String,
        reward_amount: u128,
        survey_hash: String,
        manager_pub_key: String,
    },
    CancelSurvey {
        signature: String,
        token: String,
        time_to_expire: u64,
        survey_id: String,
        manager_pub_key: String,
    },
    PayRewards {
        signature: String,
        token: String,
        time_to_expire: u64,
        survey_ids: Vec<String>,
        participants: Vec<String>,
        manager_pub_key: String,
    },
    TransferOwnership {
        new_owner: String,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub struct SurveyResponse {
    pub survey_creator: String,
    pub participants_limit: u32,
    pub reward_amount: u128,
    pub participants_rewarded: u32,
    pub survey_hash: String,
    pub amount_to_fund: u128,
    pub is_cancelled: bool,
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    CreateSurveyProof {
        token: String,
        time_to_expire: u64,
        owner: String,
        survey_id: String,
        participants_limit: u32,
        reward_amount: u128,
        survey_hash: String,
        reward_denom: String,
    },
    #[returns(Binary)]
    CancelSurveyProof {
        token: String,
        time_to_expire: u64,
        survey_id: String,
    },
    #[returns(Binary)]
    PayRewardsProof {
        token: String,
        time_to_expire: u64,
        survey_ids: Vec<String>,
        participants: Vec<String>,
    },
    #[returns(SurveyResponse)]
    GetSurvey { survey_id: String },
    #[returns(u128)]
    GetSurveyAmountToFund { survey_id: String },
    #[returns(u128)]
    GetSurveyRewardsAmountPaid { survey_id: String },
    #[returns(crate::state::Config)]
    GetConfig {},
}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
