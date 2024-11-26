use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StubsPeriodVerificationPeriodCode {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "strategy")]
    pub strategy: Strategy,
    #[serde(
        rename = "attempts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attempts: Option<Option<i64>>,
    #[serde(rename = "expire_at")]
    pub expire_at: i64,
}

impl StubsPeriodVerificationPeriodCode {
    pub fn new(
        status: Status,
        strategy: Strategy,
        expire_at: i64,
    ) -> StubsPeriodVerificationPeriodCode {
        StubsPeriodVerificationPeriodCode {
            status,
            strategy,
            attempts: None,
            expire_at,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unverified
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "email_code")]
    EmailCode,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::EmailCode
    }
}
