use actix_web::{web, HttpResponse, Responder};

use crate::domain::ports::incoming::InstanceTypesService;

pub async fn list<S: InstanceTypesService>(service: web::Data<S>) -> impl Responder {
  match service.list() {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
  }
}

pub fn routes<S: InstanceTypesService + 'static>(config: &mut web::ServiceConfig) {
  config.route("/api/instance_types", web::get().to(list::<S>));
}
