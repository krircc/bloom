use serde::{Serialize, Deserialize};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::domain::contact;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateWebsites {
    pub websites: Vec<contact::Website>,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateWebsites {
    type Aggregate = contact::Contact;
    type Event = contact::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = contact::EventData::WebsitesUpdatedV1(contact::WebsitesUpdatedV1{
            websites: self.websites.clone(),
        });

        return  Ok((contact::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}