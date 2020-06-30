use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    id: i32,
    payload: String,
    time_sent: i64,
    sleep_intv: i32
}

pub async fn echo_request(req: web::Query<Payload>) -> impl Responder {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    let sleep_intv: i32 = req.sleep_intv as i32;
    if sleep_intv > 0 {
        sleep(Duration::from_millis(sleep_intv as u64));
    }
    let payload = json!({
        "id": req.id,
        "time_sent": req.time_sent,
        "message": req.payload,
        "time_recv": in_ms
    });
    HttpResponse::Ok().json(payload)
}