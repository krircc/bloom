use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    KernelError,
};

pub fn post(
    album_data: web::Json<models::CreateAlbumBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        state
            .db
            .send(controllers::CreateAlbum {
                name: album_data.name.clone(),
                account_id: auth.account.expect("error unwraping non none account").id,
                session_id: auth.session.expect("error unwraping non none session").id,
                request_id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |album| match album {
                Ok(album) => {
                    let res = models::AlbumResponse::from(album);
                    let res = api::Response::data(res);
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}
