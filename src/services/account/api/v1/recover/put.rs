use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    services::account::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
    services::common::utils,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;
use futures::future::IntoFuture;
use rand::Rng;
use std::time::Duration;



pub fn put((password_data, req): (Json<models::ResetPassowrdBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();


    let session_id = match auth.session {
        Some(ref session) => Some(session.id),
        None => None,
    };

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::ResetPassword{
            reset_password_id: password_data.id,
            token: password_data.token.clone(),
            new_password: password_data.new_password.clone(),
            request_id,
            session_id,
        }).flatten()
    )
    .and_then(move |(session, token)| {
        let res = api::Response::data(models::SignInResponse{
            token: utils::encode_session(&session.id.to_string(), &token),
            id: session.id,
        });
        Ok(HttpResponse::Ok().json(&res))
    })
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}