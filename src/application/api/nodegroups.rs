use actix_web::{web, HttpResponse, Responder};

use crate::domain::model::{NodeGroupDto, NodegroupRequestDto, ResponseStatusDto};
use crate::domain::ports::incoming::NodegroupService;

pub async fn list<S: NodegroupService<NodeGroupDto>>(service: web::Data<S>) -> impl Responder {
  HttpResponse::Ok().json(service.list())
}

pub async fn get<S: NodegroupService<NodeGroupDto>>(name: web::Path<String>, service: web::Data<S>) -> impl Responder {
  if let Some(dto) = service.get(&name) {
    return HttpResponse::Ok().json(dto);
  }
  HttpResponse::NotFound().finish()
}

pub async fn create<S: NodegroupService<NodeGroupDto>>(request: web::Json<NodegroupRequestDto>, service: web::Data<S>) -> impl Responder {
  match service.create(&request.to_owned()) {
    Ok(_) =>HttpResponse::Ok().json(ResponseStatusDto{status: String::from("ok")}),
    Err(_) => {
      HttpResponse::InternalServerError().finish() //To be implemented
    }
  }
}

pub fn routes<S: NodegroupService<NodeGroupDto> + 'static>(config: &mut web::ServiceConfig) {
  config.route("/api/nodegroups", web::post().to(create::<S>));
  config.route("/api/nodegroups", web::get().to(list::<S>));
  config.route("/api/nodegroups/{name}", web::get().to(get::<S>));
}

