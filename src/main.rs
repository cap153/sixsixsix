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

    // 追加地支和五行
    // 拼接nei和wai数组为字符串
    let nei_index: String = nei.join("");
    let wai_index: String = wai.join("");
    // 匹配HunTian结构体的index，获取nei和wai的地支和五行
    let hun_tian_nei = HUN_TIAN_DATA.iter().find(|&h| h.index == nei_index).unwrap();
    let hun_tian_wai = HUN_TIAN_DATA.iter().find(|&h| h.index == wai_index).unwrap();
    // 将nei的地支和五行添加到gua_xian的前三个元素的开头
    for i in 0..3 {
        gua_xian[i] = format!("{}{}", hun_tian_nei.nei[i], gua_xian[i]);
    }
    // 将wai的地支和五行添加到gua_xian的后三个元素的开头
    for i in 0..3 {
        gua_xian[i + 3] = format!("{}{}", hun_tian_wai.wai[i], gua_xian[i + 3]);
    }

    // 获取世爻的五行
    let shi_wuxing = gua_xian[shi_idx].chars().nth(1).unwrap();

    // 根据五行生克关系判断六亲并添加到gua_xian的开头
    for i in 0..gua_xian.len() {
        let current_wuxing = gua_xian[i].chars().nth(1).unwrap();
        let liu_qin = if current_wuxing == shi_wuxing {
            "兄弟"
        } else {
            match (shi_wuxing, current_wuxing) {
                ('木', '火') => "子孙",
                ('火', '土') => "子孙",
                ('土', '金') => "子孙",
                ('金', '水') => "子孙",
                ('水', '木') => "子孙",
                ('木', '土') => "妻财",
                ('土', '水') => "妻财",
                ('水', '火') => "妻财",
                ('火', '金') => "妻财",
                ('金', '木') => "妻财",
                ('木', '金') => "官鬼",
                ('金', '火') => "官鬼",
                ('火', '水') => "官鬼",
                ('水', '土') => "官鬼",
                ('土', '木') => "官鬼",
                ('木', '水') => "父母",
                ('水', '金') => "父母",
                ('金', '土') => "父母",
                ('土', '火') => "父母",
                ('火', '木') => "父母",
                _ => "",
            }
        };
        gua_xian[i] = format!("{}{}", liu_qin, gua_xian[i]);
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
