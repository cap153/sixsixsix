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

    //chars是用于过滤动卦的中间字符(有些逻辑不需要动卦判断)
    let mut chars = Vec::new();
    for c in numbers.chars() {
        let gua = match c {
            '0' => "2".to_string(),
            '1' => "1".to_string(),
            '2' => "2".to_string(),
            '3' => "1".to_string(),
            _ => "".to_string(),
        };
        chars.push(gua);
    }
    //nei表示内卦，wai表示外卦
    let (nei, wai) = chars.split_at(3);

    // 确定世爻和应爻的位置
    let (shi_idx, ying_idx) = if nei[2] == wai[2] && nei[0] != wai[0] && nei[1] != wai[1] {
        (1, 4)
    } else if nei[2] != wai[2] && nei[0] == wai[0] && nei[1] == wai[1] {
        (4, 1)
    } else if nei[0] == wai[0] && nei[1] != wai[1] && nei[2] != wai[2] {
        (3, 0)
    } else if nei[0] != wai[0] && nei[1] == wai[1] && nei[2] == wai[2] {
        (0, 3)
    } else if nei[1] == wai[1] && nei[0] != wai[0] && nei[2] != wai[2] {
        (3, 0)
    } else if nei[1] != wai[1] && nei[0] == wai[0] && nei[2] == wai[2] {
        (2, 5)
    } else if nei[0] == wai[0] && nei[1] == wai[1] && nei[2] == wai[2] {
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
