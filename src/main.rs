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

#[derive(Serialize, Deserialize)]
struct HunTian {
    index: &'static str,
    nei: [&'static str; 3],
    wai: [&'static str; 3],
}

// 新增八个本宫卦的地支和五行数据
const HUN_TIAN_DATA: [HunTian; 8] = [
    HunTian { index: "111", nei: ["子水", "寅木", "辰土"], wai: ["午火", "申金", "戌土"] },
    HunTian { index: "122", nei: ["子水", "寅木", "辰土"], wai: ["午火", "申金", "戌土"] },
    HunTian { index: "212", nei: ["寅木", "辰土", "午火"], wai: ["申金", "戌土", "子水"] },
    HunTian { index: "221", nei: ["辰土", "午火", "申金"], wai: ["戌土", "子水", "寅木"] },
    HunTian { index: "222", nei: ["未土", "巳火", "卯木"], wai: ["丑土", "亥水", "酉金"] },
    HunTian { index: "211", nei: ["丑土", "亥水", "酉金"], wai: ["未土", "巳火", "卯木"] },
    HunTian { index: "121", nei: ["卯木", "丑土", "亥水"], wai: ["酉金", "未土", "巳火"] },
    HunTian { index: "112", nei: ["巳火", "卯木", "丑土"], wai: ["亥水", "酉金", "未土"] },
];

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
