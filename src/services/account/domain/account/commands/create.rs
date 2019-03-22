use crate::{
    services::account::domain::account,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub avatar_url: String,
    pub metdata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for Create {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            account_accounts::dsl::*,
        };
        use diesel::QueryDsl;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(&self.password)?;
        // TODO: validate email


        // verify that an email isn't already in use
        let existing_email: i64 = account_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!("Email: {} is already in use.", &self.email)));
        }

        // verify that username isn't already in use
        let existing_username: i64 = account_accounts
            .filter(username.eq(&self.username))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_username != 0 {
            return Err(KernelError::Validation(format!("Username: {} is already in use.", &self.username)));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4();
        let mut metadata = self.metdata.clone();
        metadata.actor_id = Some(id);
        let data = account::EventData::CreatedV1(account::CreatedV1{
            id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            avatar_url: self.avatar_url.clone(),
            username: self.username.clone(),
            is_admin: false,
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: aggregate.id,
            metadata,
        }, ()));
    }
}