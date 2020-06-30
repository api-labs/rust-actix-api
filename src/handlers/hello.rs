use actix_web::{web, HttpRequest, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Measurement {
    temperature: f64,
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn temperature(req: HttpRequest) -> impl Responder {
    let tmp = req.match_info().get("temp").unwrap_or("42.3");
    let temperature: f64 = tmp.parse().unwrap();
    web::Json(Measurement { temperature: temperature })
}