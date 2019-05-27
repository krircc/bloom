use actix::{Message, Handler};
use crate::{
    db::DbActor,
    myaccount::domain::{
        PendingAccount,
        pending_account,
    },
    myaccount::notifications::emails::send_account_verification_code,
    events::EventMetadata,
    config::Config,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendNewVerificationCode {
    pub pending_account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub config: Config,
}

impl Message for SendNewVerificationCode {
    type Result = Result<(), KernelError>;
}

impl Handler<SendNewVerificationCode> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: SendNewVerificationCode, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_pending_accounts,
            kernel_pending_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };
            let resend_code_cmd = pending_account::SendNewCode{
                metadata,
            };

            let pending_account: PendingAccount = kernel_pending_accounts::dsl::kernel_pending_accounts
                .filter(kernel_pending_accounts::dsl::id.eq(msg.pending_account_id))
                .filter(kernel_pending_accounts::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (pending_account, event, non_persisted) = eventsourcing::execute(&conn, pending_account, &resend_code_cmd)?;

            // update pending_account
            diesel::update(&pending_account)
                .set(&pending_account)
                .execute(&conn)?;
            diesel::insert_into(kernel_pending_accounts_events::dsl::kernel_pending_accounts_events)
                .values(&event)
                .execute(&conn)?;

            let config = msg.config.clone();
            send_account_verification_code(
                &config,
                pending_account.email.as_str(),
                format!("{} {}", &pending_account.first_name, &pending_account.last_name).as_str(),
                pending_account.id.to_string().as_str(),
                &non_persisted.code,
            ).expect("error sending email");

            return Ok(());
        })?);
    }
}