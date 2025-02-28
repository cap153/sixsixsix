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

    // 确定世爻和应爻的位置
    let a = &gua_xian;
    let b = &a[0..3];
    let c = &a[3..6];

    let (shi_idx, ying_idx) = if b[2] == c[2] && b[0] != c[0] && b[1] != c[1] {
        (1, 4)
    } else if b[2] != c[2] && b[0] == c[0] && b[1] == c[1] {
        (4, 1)
    } else if b[0] == c[0] && b[1] != c[1] && b[2] != c[2] {
        (3, 0)
    } else if b[0] != c[0] && b[1] == c[1] && b[2] == c[2] {
        (0, 3)
    } else if b[1] == c[1] && b[0] != c[0] && b[2] != c[2] {
        (3, 0)
    } else if b[1] != c[1] && b[0] == c[0] && b[2] == c[2] {
        (2, 5)
    } else if b[0] == c[0] && b[1] == c[1] && b[2] == c[2] {
        (5, 2)
    } else {
        (2, 5)
    };

    // 追加世爻和应爻的标记
    gua_xian[shi_idx].push_str(" 世");
    gua_xian[ying_idx].push_str(" 应");

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