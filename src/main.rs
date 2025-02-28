use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuaRequest {
    numbers: String,
}

#[derive(Serialize, Deserialize)]
struct GuaResponse {
    gua_xian: Vec<String>,
}

async fn generate_gua_xian(req: web::Json<GuaRequest>) -> impl Responder {
    let numbers = &req.numbers;
    let mut gua_xian = Vec::new();

    for c in numbers.chars() {
        let gua = match c {
            '0' => "━━ ━━ x".to_string(),
            '1' => "━━━━━".to_string(),
            '2' => "━━ ━━".to_string(),
            '3' => "━━━━━ o".to_string(),
            _ => "".to_string(),
        };
        gua_xian.push(gua);
    }

    HttpResponse::Ok().json(GuaResponse { gua_xian })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate_gua_xian", web::post().to(generate_gua_xian))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}