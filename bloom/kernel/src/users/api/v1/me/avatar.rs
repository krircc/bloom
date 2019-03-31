use crate::{
    api,
    log::macros::*,
    users::controllers,
    users::api::v1::models,
    users,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use actix_web::{
    ResponseError, AsyncResponder, Error, HttpMessage, FutureResponse,
    HttpRequest, HttpResponse, dev, multipart, error,
};
use futures::{Future, Stream, IntoFuture};
use futures::future;


pub fn put(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.user.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return req.multipart()
        .map_err(error::ErrorInternalServerError)
        .map(handle_multipart_item)
        .flatten()
        .collect()
        .into_future()
        .map_err(|_| KernelError::Validation("file too large".to_string()))
        .and_then(move |avatar| {
            state.db
            .send(controllers::UpdateAvatar{
                user: auth.user.expect("unwrapping non none user"),
                avatar: avatar.get(0).expect("processing file").to_vec(),
                s3_bucket: state.config.aws_s3_bucket(),
                s3_region: state.config.aws_region(),
                s3_client: state.s3_client.clone(),
                request_id,
                session_id: auth.session.expect("unwraping non none session").id,
            }).flatten()
        })
        .and_then(move |user| {
            let res = models::MeResponse{
                id: user.id,
                created_at: user.created_at,
                first_name: user.first_name,
                last_name: user.last_name,
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
            };
            let res = api::Response::data(res);
            Ok(HttpResponse::Ok().json(&res))
        })
        .from_err()
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err;
        })
        .from_err()
        .responder();
}

fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = Vec<u8>, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(read_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => {
            Box::new(
                mp.map_err(error::ErrorInternalServerError)
                    .map(handle_multipart_item)
                    .flatten(),
            )
        },
    }
}

fn read_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = Vec<u8>, Error = Error>> {
    Box::new(
        field
        .fold(Vec::new(), |mut acc, bytes| -> future::FutureResult<_, error::MultipartError> {
            acc.extend_from_slice(&bytes);
            if acc.len() > users::AVATAR_MAX_SIZE {
                return future::err(error::MultipartError::Payload(error::PayloadError::Overflow))
            }
            future::ok(acc)
        })
        .map_err(|e| error::ErrorInternalServerError(e)),
    )
}