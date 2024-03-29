mod state;

use crate::KernelError;
use actix_web::{web, web::JsonConfig, HttpResponse, ResponseError, Result as ActixResult};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod middlewares;
pub use state::State;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<BlmResponseError>,
}

#[derive(Serialize)]
pub struct BlmResponseError {
    message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HelloWorld<'a> {
    pub hello: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

impl<T: Serialize> Response<T> {
    pub fn data(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            error: None,
        };
    }

    pub fn error(err: KernelError) -> Response<T> {
        return Response {
            data: None,
            error: Some(BlmResponseError {
                message: format!("{}", err),
            }),
        };
    }
}

pub fn index() -> ActixResult<HttpResponse> {
    let res = HelloWorld { hello: "world" };
    return Ok(HttpResponse::Ok().json(Response::data(res)));
}

pub fn route_404() -> ActixResult<HttpResponse> {
    return Ok(KernelError::RouteNotFound.error_response());
}

pub fn route_disabled() -> ActixResult<HttpResponse> {
    return Ok(KernelError::Validation("Service is disabled".to_string()).error_response());
}

pub fn json_default_config() -> JsonConfig {
    return JsonConfig::default()
        .limit(256_000)
        .error_handler(|err, _| {
            KernelError::Validation(err.to_string()).into() // <- create custom error response
        });
}

static ENV_TEMPLATE: &str = r#"
window.__bloom = {
  config: {
    ENV: "{{node_env}}",
    SENTRY_URL: "{{sentry_url}}",
    HOST: "{{host}}",
    STRIPE_PUBLIC_KEY: "{{stripe_public_key}}",
    VERSION: "{{version}}",
  },
};
"#;

pub fn webapp_env(state: web::Data<State>) -> ActixResult<HttpResponse> {
    let config = state.config.clone();
    let handlebars = Handlebars::new();

    let mut data = BTreeMap::new();
    data.insert("node_env".to_string(), config.rust_env.to_string());
    data.insert(
        "sentry_url".to_string(),
        config.sentry.webapp_url.unwrap_or_default(),
    );
    data.insert("host".to_string(), config.host);
    data.insert("stripe_public_key".to_string(), config.stripe.public_key);
    data.insert("version".to_string(), config.version);

    let res = handlebars
        .render_template(ENV_TEMPLATE, &data)
        .expect("error rendering env template")
        .as_str()
        .to_string();

    return Ok(HttpResponse::Ok()
        .content_type("application/javascript; charset=UTF-8")
        .body(res));
}
