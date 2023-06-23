use crate::{Allocation, AllocationID, Claim, ClaimID, DataCap};
use fvm_ipld_encoding::tuple::*;
use fvm_shared::bigint::bigint_ser;
use fvm_shared::ActorID;

// XXX the verifier's datacap balance is different units,

/// Indicates that a verifier's balance of datacap has a new value.
/// Note: this event does not guarantee that the verifier's prior balance was different.
pub const EVENT_VERIFIER_BALANCE: &str = "VerifierBalanceChanged";
#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct VerifierBalanceEvent {
    pub verifier: ActorID,
    #[serde(with = "bigint_ser")]
    pub new_balance: DataCap,
}

/// Indicates that a datacap allocation has been made.
pub const EVENT_ALLOCATION: &str = "Allocation";
#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct AllocationEvent {
    pub id: AllocationID,
    pub allocation: Allocation,
}

/// Indicates that a datacap allocation has been claimed.
pub const EVENT_CLAIM: &str = "Claim";
#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct ClaimEvent {
    pub id: ClaimID,
    pub claim: Claim,
}
