use crate::{
    api,
    users::api::v1::models,
    log::macros::*,
    users::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
    utils,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;
use futures::future::IntoFuture;
use std::time::Duration;
use rand::Rng;


pub fn post((sign_in_data, req): (Json<models::SignInBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();

    if auth.session.is_some() || auth.user.is_some() {
        return future::result(Ok(KernelError::Unauthorized("Must not be authenticated".to_string()).error_response()))
            .responder();
    }


    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 600))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::SignIn{
            username: sign_in_data.username.clone(),
            password: sign_in_data.password.clone(),
            request_id,
        }).flatten()
    )
    .and_then(move |(session, token)| {
        let res = api::Response::data(models::SignInResponse{
            token: utils::encode_session(&session.id.to_string(), &token),
            id: session.id,
        });
        Ok(HttpResponse::Ok().json(&res))
    })
    .from_err() // MailboxError to KernelError
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}