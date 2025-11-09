use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] cw_utils::PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Survey Not Found")]
    SurveyNotFound {},

    #[error("Survey Already Exists")]
    SurveyAlreadyExists {},

    #[error("Survey Already Cancelled")]
    SurveyAlreadyCancelled {},

    #[error("Invalid Manager")]
    InvalidManager {},

    #[error("Only Creator Or Manager")]
    OnlyCreatorOrManager {},

    #[error("All Participants Rewarded")]
    AllParticipantsRewarded {},

    #[error("Array Length Mismatch")]
    ArrayLengthMismatch {},

    #[error("User Already Rewarded")]
    UserAlreadyRewarded {},

    #[error("Invalid Message Hash")]
    InvalidMessageHash {},

    #[error("Token Already Used")]
    TokenAlreadyUsed {},

    #[error("Proof Expired")]
    ProofExpired {},

    #[error("Insufficient Funds")]
    InsufficientContractBalance {},

    #[error("Survey Creation Failed")]
    SurveyCreationFailed {},

    #[error("Reward Payment Failed")]
    RewardPaymentFailed {},

    #[error("Survey Cancellation Failed")]
    SurveyCancellationFailed {},

    #[error("Invalid Signer")]
    InvalidSigner {},

    #[error("Invalid Reward Amount")]
    InvalidRewardAmount {},

    #[error("Invalid Transaction Value")]
    InvalidTransactionValue {},

    #[error("Arithmetic Error")]
    ArithmeticError {},

    #[error("Invalid Address")]
    InvalidAddress {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Signature Verification Failed")]
    SignatureVerificationFailed(String),

    #[error("Invalid Account: {receiver}")]
    InvalidAccount { receiver: String },

    #[error("Expected Agoric Account: {receiver}")]
    ExpectedAgoricAccount { receiver: String },

    #[error("Sequence not found in request packet")]
    SequenceNotFound {},

    #[error("Channel ID not found in request packet")]
    ChannelIdNotFound {},

    #[error("Failed to deserialize payload: {error}")]
    PayloadDeserializationFailed { error: String },

    #[error("Acknowledgement already processed")]
    AcknowledgementAlreadyProcessed {},

    #[error("Unknown reply id: {id}")]
    UnknownReplyId { id: u64 },

    #[error("Reply error: {error}")]
    ReplyError { error: String },

    #[error("No data in reply")]
    NoDataInReply {},

    #[error("Nothing to refund")]
    NothingToRefund {},

    #[error("Failed to parse MsgSubmitTxResponse: {error}")]
    ResponseParseFailed { error: String },
}

impl From<cosmwasm_std::VerificationError> for ContractError {
    fn from(err: cosmwasm_std::VerificationError) -> Self {
        Self::SignatureVerificationFailed(err.to_string())
    }
}

impl From<cosmwasm_std::OverflowError> for ContractError {
    fn from(_: cosmwasm_std::OverflowError) -> Self {
        Self::ArithmeticError {}
    }
}
