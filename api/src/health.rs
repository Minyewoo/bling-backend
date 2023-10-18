use actix_web::{get, HttpResponse, Responder, web};
use std::sync::Mutex;

#[get("/live")]
pub async fn live() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("cache-control", "no-cache"))
        .finish()
}

pub struct AppReadiness {
    pub is_ready: Mutex<bool>,
}

#[get("/ready")]
pub async fn ready(readiness: web::Data<AppReadiness>) -> impl Responder {
    match readiness.is_ready.lock() {
        Ok(is_ready) => match *is_ready {
            true => HttpResponse::Ok()
                            .insert_header(("cache-control", "no-cache"))
                            .finish(),
            false => HttpResponse::ServiceUnavailable()
                .insert_header(("cache-control", "no-cache"))
                .finish(),
        },
        Err(_) => HttpResponse::InternalServerError()
            .insert_header(("cache-control", "no-cache"))
            .finish(),
    }
}