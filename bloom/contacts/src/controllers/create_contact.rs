use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain::contact;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateContact {
    pub addresses: Vec<contact::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<contact::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<contact::Organization>,
    pub phones: Vec<contact::Phone>,
    pub websites: Vec<contact::Website>,
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for CreateContact {
    type Result = Result<contact::Contact, KernelError>;
}

impl Handler<CreateContact> for DbActor {
    type Result = Result<contact::Contact, KernelError>;

    fn handle(&mut self, msg: CreateContact, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            contacts_contacts,
            contacts_contacts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            // create Contact
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let create_cmd = contact::Create{
                addresses: msg.addresses,
                birthday: msg.birthday,
                company: msg.company,
                emails: msg.emails,
                first_name: msg.first_name,
                last_name: msg.last_name,
                notes: msg.notes,
                occupation: msg.occupation,
                organizations: msg.organizations,
                phones: msg.phones,
                websites: msg.websites,
                owner_id: msg.account_id,
                metadata,
            };
            let (note, event, _) = eventsourcing::execute(&conn, contact::Contact::new(), &create_cmd)?;

            diesel::insert_into(contacts_contacts::dsl::contacts_contacts)
                .values(&note)
                .execute(&conn)?;
            diesel::insert_into(contacts_contacts_events::dsl::contacts_contacts_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(note);
        })?);
    }
}
