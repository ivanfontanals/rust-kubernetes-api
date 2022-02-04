use crate::domain::ports::incoming::probes::ProbesService;
use actix_web::{web, HttpResponse, Responder};

pub async fn liveness() -> impl Responder {
  HttpResponse::Ok().json("ok")
}

pub async fn readiness<S: ProbesService>(probe_service: web::Data<S>) -> impl Responder {
  match probe_service.is_ready() {
    true => HttpResponse::Ok().json("ok"),
    false => HttpResponse::ServiceUnavailable().json("Application not ready"),
  }
}

pub fn routes<S: ProbesService + 'static>(config: &mut web::ServiceConfig) {
  config.route("/liveness", web::get().to(liveness));
  config.route("/readiness", web::get().to(readiness::<S>));
}
