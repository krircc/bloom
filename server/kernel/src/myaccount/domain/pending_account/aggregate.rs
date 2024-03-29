use crate::db::schema::kernel_pending_accounts;
use diesel::Queryable;
use eventsourcing::Aggregate;
use serde::{Deserialize, Serialize};

#[derive(
    Aggregate,
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Serialize,
)]
#[table_name = "kernel_pending_accounts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PendingAccount {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String, // hashed password
    pub token: String,    // hashed verification code
    pub trials: i64,
    pub verified: bool,
}

impl PendingAccount {
    // create a new, unitialized PendingAccount
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return PendingAccount {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            version: 0,

            email: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            password: String::new(),
            token: String::new(),
            trials: 0,
            verified: false,
        };
    }
}

impl Default for PendingAccount {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum VerificationFailedReason {
    CodeNotValid,
    CodeExpired,
    TooManyTrials,
}

impl ToString for VerificationFailedReason {
    fn to_string(&self) -> String {
        match self {
            VerificationFailedReason::CodeNotValid => "Code is not valid.".to_string(),
            VerificationFailedReason::CodeExpired => {
                "Code has expired. Please create another account.".to_string()
            }
            VerificationFailedReason::TooManyTrials => {
                "Maximum number of trials reached. Please create another account.".to_string()
            }
        }
    }
}
