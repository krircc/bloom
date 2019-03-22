use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    api::middlewares::logger::GetRequestLogger,
    services::account::controllers,
    api::middlewares::request_id::GetRequestID,
    services::common::utils,
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, HttpRequest,
};
use futures::future::IntoFuture;
use rand::Rng;


pub fn complete_registration_post((registration_data, req): (Json<models::CompleteRegistrationBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();
    let config = state.config.clone();
    let request_id = req.request_id().0;

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 650))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::CompleteRegistration{
            id: registration_data.id.clone(),
            code: registration_data.code.clone(),
            username: registration_data.username.clone(),
            config,
            request_id,
        }).flatten()
    )
    .and_then(move |(session, token)| {
        let session_id = session.id.to_string();
        let res = api::Response::data(models::CompleteRegistrationResponse{
            token: utils::encode_session(&session_id, &token),
            id: session_id,
        });
        Ok(HttpResponse::Created().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}