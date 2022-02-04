use actix_web::{web, HttpResponse, Responder};

use crate::domain::model::SecretRequestDto;
use crate::domain::ports::incoming::SecretService;

pub async fn list<S: SecretService>(repository: web::Data<S>) -> impl Responder {
  HttpResponse::Ok().json(repository.list())
}

pub async fn get<S: SecretService>(name: web::Path<String>, repository: web::Data<S>) -> impl Responder {
  if let Some(dto) = repository.get(&name) {
    return HttpResponse::Ok().json(dto);
  }
  HttpResponse::NotFound().finish()
}

pub async fn create<S: SecretService>(request: web::Json<SecretRequestDto>, service: web::Data<S>) -> impl Responder {
  match service.create(&request.to_owned()) {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => {
      HttpResponse::InternalServerError() //To be implemented
    }
  }
}

pub async fn render<S: SecretService>(request: web::Json<SecretRequestDto>, service: web::Data<S>) -> impl Responder {
  match service.render(&request.to_owned()) {
    Ok(rendered_response) => HttpResponse::Ok().content_type("application/yaml").body(rendered_response),
    Err(error) => HttpResponse::InternalServerError().json(format!("Error rendering YAML for a SealedSecret: {}", error.to_string())),
  }
}

pub fn routes<S: SecretService + 'static>(config: &mut web::ServiceConfig) {
  config.route("/api/secrets", web::get().to(list::<S>));
  config.route("/api/secrets/{name}", web::get().to(get::<S>));
  config.route("/api/secrets", web::post().to(create::<S>));
  config.route("/api/secrets/render", web::post().to(render::<S>));
}
