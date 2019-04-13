use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain,
    FOLDER_TYPE,
    BLOOM_ROOT_NAME,
    domain::file,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Move {
    pub to: uuid::Uuid,
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for Move {
    type Result = Result<(), KernelError>;
}

impl Handler<Move> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: Move, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {

            let metadata = EventMetadata{
                actor_id: Some(msg.owner_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            for file_id in msg.files.into_iter() {

                let file_to_move: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::deleted_at.is_null())
                    .filter(drive_files::dsl::trashed_at.is_null())
                    .first(&conn)?;

                let move_cmd = file::Move{
                    to: msg.to,
                    metadata: metadata.clone(),
                };
                let (file_to_move, event, _) = eventsourcing::execute(&conn, file_to_move, &move_cmd)?;
                diesel::update(&file_to_move)
                    .set(&file_to_move)
                    .execute(&conn)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}