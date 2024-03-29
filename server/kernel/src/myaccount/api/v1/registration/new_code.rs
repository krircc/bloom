use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    myaccount::api::v1::models,
    myaccount::controllers,
    KernelError,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future, IntoFuture};
use rand::Rng;
use std::time::Duration;

pub fn post(
    registration_data: web::Json<models::NewCodeBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
    let logger = req.logger();
    let config = state.config.clone();
    let request_id = req.request_id().0;
    let auth = req.request_auth();

    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Either::A(ok(KernelError::Unauthorized(
            "Must not be authenticated".to_string(),
        )
        .error_response()));
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 650)))
            .into_future()
            .from_err()
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::SendNewVerificationCode {
                        pending_account_id: registration_data.id,
                        config,
                        request_id,
                    })
                    .flatten()
            })
            .from_err()
            .and_then(move |_| {
                let res = api::Response::data(api::NoData {});
                ok(HttpResponse::Ok().json(&res))
            })
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.error_response();
            })
            .from_err(),
    );
}
