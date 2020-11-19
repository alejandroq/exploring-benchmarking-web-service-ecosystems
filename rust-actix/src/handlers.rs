use crate::error::Error;
use crate::service::{Envs, Options, TracksService};
use actix_web::{error::ResponseError, get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use tracks_core::tracks::Track;

#[get("/tracks/{key_path}")]
async fn find(
    req: HttpRequest,
    web::Path(key_path): web::Path<String>,
    pool: web::Data<sqlx::MySqlPool>,
    envs: web::Data<Envs>,
) -> impl Responder {
    let client_id = get_header(&req, "x-client-id");
    let request_id = get_header(&req, "x-request-id");

    match TracksService::read(
        &pool,
        key_path.clone(),
        Options {
            service_name: envs.service_name.clone(),
            node_id: envs.node_id.clone(),
            client_id: client_id,
            request_id: request_id,
        },
    )
    .await
    {
        Ok(value) => HttpResponse::Ok().json(Track { key_path, value }),
        Err(err) => Error::ServiceError(err).error_response(),
    }
}

#[get("health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "healthy",
    }))
}

pub async fn not_found() -> impl Responder {
    Error::NotFound.error_response()
}

fn get_header<'a>(req: &'a HttpRequest, header: &str) -> Option<String> {
    let value: &str = match req.headers().get(header)?.to_str().ok() {
        Some(val) => val,
        _ => "",
    };
    if value == "" {
        None
    } else {
        Some(String::from(value))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
    cfg.service(find);
}
