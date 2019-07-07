use crate::{error::KernelError, myaccount::domain::pending_email};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::{Deserialize, Serialize};
use eventsourcing::{Event, EventTs};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delete {}

impl eventsourcing::Command for Delete {
    type Aggregate = pending_email::PendingEmail;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation(
                "PDeleteing email has already been deleted.".to_string(),
            ));
        }
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = pending_email::PendingEmail;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            deleted_at: Some(self.timestamp),
            ..aggregate
        };
    }
}
