use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Local, Timelike};
use lunar_rust::{
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize}; // For getting the current time

// 嵌入整个 static 目录（递归所有文件）
#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

// 通用处理函数，根据请求路径返回嵌入的文件
async fn embedded_file(path: web::Path<String>) -> impl Responder {
    // 如果请求为空，则默认返回 index.html
    let file_path = if path.is_empty() {
        "index.html".to_string()
    } else {
        path.into_inner()
    };

    match Asset::get(&file_path) {
        Some(content) => {
            // 根据文件后缀可以进一步设置 Content-Type，此处省略，直接返回 body
            HttpResponse::Ok().body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("File not found"),
    }
}

#[derive(Serialize, Deserialize)]
struct GuaRequest {
    numbers: String,
}

#[derive(Serialize, Deserialize)]
struct GuaResponse {
    gua_xian: Vec<String>,
    year_ganzhi: String,
    month_ganzhi: String,
    day_ganzhi: String,
    hour_ganzhi: String,
}

struct SixtyFourGua {
    // name: &'static str,
    index: &'static str,
    nei: [&'static str; 3],
    wai: [&'static str; 3],
    palace_element: &'static str,
    gua_name: [&'static str; 8],
    gua_index: [&'static str; 8],
}

// 新增八个本宫卦的地支和五行数据
const SIXTYFOURGUA_DATA: [SixtyFourGua; 8] = [
    SixtyFourGua {
        // name: "乾",
        index: "111",
        nei: ["子水", "寅木", "辰土"],
        wai: ["午火", "申金", "戌土"],
        palace_element: "金",
        gua_name: [
            "乾为天䷀ (六冲)",
            "天风姤䷫ ",
            "天山遁䷠ ",
            "天地否䷋ (六合)",
            "风地观䷓ ",
            "山地剥䷖ ",
            "火地晋䷢ ",
            "火天大有䷍",
        ],
        gua_index: [
            "111111", "211111", "221111", "222111", "222211", "222221", "222121", "111121",
        ],
    },

    SixtyFourGua {
        // name: "震",
        index: "122",
        nei: ["子水", "寅木", "辰土"],
        wai: ["午火", "申金", "戌土"],
        palace_element: "木",
        gua_name: [
            "震为雷䷲ (六冲)",
            "雷地豫䷏ (六合)",
            "雷水解䷧ ",
            "雷风恒䷟ ",
            "地风升䷭ ",
            "水风井䷯ ",
            "泽风大过䷛",
            "泽雷随䷐",
        ],
        gua_index: [
            "122122", "222122", "212122", "211122", "211222", "211212", "211112", "122112",
        ],
    },
    SixtyFourGua {
        // name: "坎",
        index: "212",
        nei: ["寅木", "辰土", "午火"],
        wai: ["申金", "戌土", "子水"],
        palace_element: "水",
        gua_name: [
            "坎为水䷜(六冲)",
            "水泽节䷻(六合)",
            "水雷屯䷂",
            "水火既济䷾",
            "泽火革䷰",
            "雷火丰䷶",
            "地火明夷䷣",
            "地水师䷆",
        ],
        gua_index: [
            "212212", "112212", "122212", "121212", "121112", "121122", "121222", "212222",
        ],
    },
    SixtyFourGua {
        // name: "艮",
        index: "221",
        nei: ["辰土", "午火", "申金"],
        wai: ["戌土", "子水", "寅木"],
        palace_element: "土",
        gua_name: [
            "艮为山䷳(六冲)",
            "山火贲䷕(六合)",
            "山天大畜䷙",
            "山泽损䷨",
            "火泽睽䷥",
            "天泽履䷉",
            "风泽中孚䷼",
            "风山渐䷴",
        ],
        gua_index: [
            "221221", "121221", "111221", "112221", "112121", "112111", "112211", "221211",
        ],
    },
    SixtyFourGua {
        // name: "坤",
        index: "222",
        nei: ["未土", "巳火", "卯木"],
        wai: ["丑土", "亥水", "酉金"],
        palace_element: "土",
        gua_name: [
            "坤为地䷁(六冲)",
            "地雷复䷗(六合)",
            "地泽临䷒",
            "地天泰䷊(六合)",
            "雷天大壮䷡(六冲)",
            "泽天夬䷪",
            "水天需䷄",
            "水地比䷇",
        ],
        gua_index: [
            "222222", "122222", "112222", "111222", "111122", "111112", "111212", "222212",
        ],
    },
    SixtyFourGua {
        // name: "巽",
        index: "211",
        nei: ["丑土", "亥水", "酉金"],
        wai: ["未土", "巳火", "卯木"],
        palace_element: "木",
        gua_name: [
            "巽为风䷸(六冲)",
            "风天小畜䷈",
            "风火家人䷤",
            "风雷益䷩",
            "天雷无妄䷘(六冲)",
            "火雷噬嗑䷔",
            "山雷颐䷚",
            "山风蛊䷑",
        ],
        gua_index: [
            "211211", "111211", "121211", "122211", "122111", "122121", "122221", "211221",
        ],
    },
    SixtyFourGua {
        // name: "离",
        index: "121",
        nei: ["卯木", "丑土", "亥水"],
        wai: ["酉金", "未土", "巳火"],
        palace_element: "火",
        gua_name: [
            "离为火䷝(六冲)",
            "火山旅䷷(六合)",
            "火风鼎䷱",
            "火水未济䷿",
            "山水蒙䷃",
            "风水涣䷺",
            "天水讼䷅",
            "天火同人䷌",
        ],
        gua_index: [
            "121121", "221121", "211121", "212121", "212221", "212211", "212111", "121111",
        ],
    },
    SixtyFourGua {
        // name: "兑",
        index: "112",
        nei: ["巳火", "卯木", "丑土"],
        wai: ["亥水", "酉金", "未土"],
        palace_element: "金",
        gua_name: [
            "兑为泽(六冲)",
            "泽水困(六合)",
            "泽地萃",
            "泽山咸",
            "水山蹇",
            "地山谦",
            "雷山小过",
            "雷泽归妹",
        ],
        gua_index: [
            "112112", "212112", "222112", "221112", "221212", "221222", "221122", "112122",
        ],
    },
];

//获取干支信息，例如乙巳年 辛巳月 壬辰日 申时
fn get_ganzhi_info() -> (String, String, String, String) {
    //获取本地的日期和时间
    let now = Local::now();
    let current_solar = solar::from_ymdhms(
        now.year() as i64,   // Cast i32 to i64
        now.month() as i64,  // Cast u32 to i64
        now.day() as i64,    // Cast u32 to i64
        now.hour() as i64,   // Cast u32 to i64
        now.minute() as i64, // Cast u32 to i64
        now.second() as i64, // Cast u32 to i64
    );
    //转换成lunar可以识别的日期
    let current_lunar = current_solar.get_lunar();
    //获取干支信息
    let year_ganzhi = current_lunar.get_year_in_gan_zhi();
    let month_ganzhi = current_lunar.get_month_in_gan_zhi();
    let day_ganzhi = current_lunar.get_day_in_gan_zhi();
    let hour_ganzhi = current_lunar.get_time_zhi();
    //返回干支信息
    (year_ganzhi, month_ganzhi, day_ganzhi, hour_ganzhi)
}

// 确定世爻和应爻的位置
fn determine_shi_ying_indices(
    nei: &[String],
    wai: &[String],
    gua_xian: &mut Vec<String>,
) -> (usize, usize) {
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
    //把世和应的索引返回用于判断是否存在冲克关系
    (shi_idx, ying_idx)
}

//追加地支和五行
fn append_dizhi_wuxing(nei: &[String], wai: &[String], gua_xian: &mut Vec<String>) {
    // 拼接nei和wai数组为字符串
    let nei_index: String = nei.join("");
    let wai_index: String = wai.join("");
    // 匹配HunTian结构体的index，获取nei和wai的地支和五行
    let hun_tian_nei = SIXTYFOURGUA_DATA
        .iter()
        .find(|&h| h.index == nei_index)
        .unwrap();
    let hun_tian_wai = SIXTYFOURGUA_DATA
        .iter()
        .find(|&h| h.index == wai_index)
        .unwrap();
    // 将nei的地支和五行添加到gua_xian的前三个元素
    for i in 0..3 {
        gua_xian[i] = format!("{}{}", hun_tian_nei.nei[i], gua_xian[i]);
    }
    // 将wai的地支和五行添加到gua_xian的后三个元素
    for i in 0..3 {
        gua_xian[i + 3] = format!("{}{}", hun_tian_wai.wai[i], gua_xian[i + 3]);
    }
}

//找到卦宫对应的五行属性用于判断六亲
fn find_palace_element(zheng_gua: &str) -> Option<&'static str> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .find(|&&idx| idx == zheng_gua)
            .map(|_| gua.palace_element)
    })
}

//查找对应的卦宫名称
fn find_palace_name(zheng_gua: &str) -> Option<&'static str> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .position(|&idx| idx == zheng_gua)
            .map(|pos| gua.gua_name[pos])
    })
}

// 六亲判断方法
fn append_liu_qin(palace_element: &str, gua_xian: &mut [String]) {
    for i in 0..gua_xian.len() {
        let current_wuxing = gua_xian[i].chars().nth(1).unwrap();
        let liu_qin = match (palace_element, current_wuxing) {
            ("金", '金') => "兄弟",
            ("金", '水') => "子孙",
            ("金", '木') => "妻财",
            ("金", '火') => "官鬼",
            ("金", '土') => "父母",
            ("木", '木') => "兄弟",
            ("木", '火') => "子孙",
            ("木", '土') => "妻财",
            ("木", '金') => "官鬼",
            ("木", '水') => "父母",
            ("水", '水') => "兄弟",
            ("水", '木') => "子孙",
            ("水", '火') => "妻财",
            ("水", '土') => "官鬼",
            ("水", '金') => "父母",
            ("火", '火') => "兄弟",
            ("火", '土') => "子孙",
            ("火", '金') => "妻财",
            ("火", '水') => "官鬼",
            ("火", '木') => "父母",
            ("土", '土') => "兄弟",
            ("土", '金') => "子孙",
            ("土", '水') => "妻财",
            ("土", '木') => "官鬼",
            ("土", '火') => "父母",
            _ => "未知",
        };
        gua_xian[i] = format!("{}{}", liu_qin, gua_xian[i]);
    }
}

//处理卦
fn process_gua<'a>(
    gua: &'a [String],
    xiang: &mut Vec<String>,
    palace_element: &str,
) -> (&'a [String], &'a [String]) {
    // nei表示内卦，wai表示外卦
    let (nei, wai) = gua.split_at(3);
    // 追加地支和五行
    append_dizhi_wuxing(nei, wai, xiang);
    // 判断六亲
    append_liu_qin(palace_element, xiang);
    // 返回内外卦数字用于判断世应等操作
    (nei, wai)
}

// 判断地支之间的冲合关系
fn get_chong_he_relation(dizhi1: &str, dizhi2: &str) -> Option<&'static str> {
    match (dizhi1, dizhi2) {
        ("子", "午") | ("午", "子") => Some("冲"),
        ("丑", "未") | ("未", "丑") => Some("冲"),
        ("寅", "申") | ("申", "寅") => Some("冲"),
        ("卯", "酉") | ("酉", "卯") => Some("冲"),
        ("辰", "戌") | ("戌", "辰") => Some("冲"),
        ("巳", "亥") | ("亥", "巳") => Some("冲"),
        ("子", "丑") | ("丑", "子") => Some("合"),
        ("寅", "亥") | ("亥", "寅") => Some("合"),
        ("卯", "戌") | ("戌", "卯") => Some("合"),
        ("辰", "酉") | ("酉", "辰") => Some("合"),
        ("巳", "申") | ("申", "巳") => Some("合"),
        ("午", "未") | ("未", "午") => Some("合"),
        _ => None,
    }
}

// 追加冲合关系到正卦对应爻
fn append_chong_he_relations(
    zheng_xiang: &mut Vec<String>,
    // shi_ying_dong: Vec<usize>,
    month_ganzhi: &str,
    day_ganzhi: &str,
) {
    //获取月和日的地支
    let month_dizhi = month_ganzhi
        .chars()
        .nth(1)
        .map(|c| &month_ganzhi[c.len_utf8()..])
        .unwrap_or("");
    let day_dizhi = day_ganzhi
        .chars()
        .nth(1)
        .map(|c| &day_ganzhi[c.len_utf8()..])
        .unwrap_or("");
    // for (i, gua) in zheng_xiang.iter_mut().enumerate() {
    for gua in zheng_xiang.iter_mut() {
        //获取卦的地支
        let dizhi: String = gua
            .chars()
            .nth(2)
            .map(|c| c.to_string())
            .unwrap_or_default();
        // if shi_ying_dong.contains(&i) {
        //判断世爻、应爻、动爻对于日月的冲克关系
        if let Some(relation) = get_chong_he_relation(&dizhi, month_dizhi) {
            gua.push_str(&format!(" 月{}", relation));
        }
        if let Some(relation) = get_chong_he_relation(&dizhi, day_dizhi) {
            gua.push_str(&format!(" 日{}", relation));
        }
        // }
    }
}

async fn generate_gua_xian(req: web::Json<GuaRequest>) -> impl Responder {
    let numbers = &req.numbers;
    let mut zheng_xiang = Vec::new();
    // 获取干支信息
    let (year_ganzhi, month_ganzhi, day_ganzhi, hour_ganzhi) = get_ganzhi_info();

    // 需要绘制的正卦卦象
    for c in numbers.chars() {
        let gua = match c {
            '0' => "⚋ x".to_string(),
            '1' => "⚊".to_string(),
            '2' => "⚋".to_string(),
            '3' => "⚊ o".to_string(),
            _ => "".to_string(),
        };
        zheng_xiang.push(gua);
    }
    //删除变卦符号的正卦
    let mut zheng_gua = Vec::new();
    for c in numbers.chars() {
        let gua = match c {
            '0' => "2".to_string(),
            '1' => "1".to_string(),
            '2' => "2".to_string(),
            '3' => "1".to_string(),
            _ => "".to_string(),
        };
        zheng_gua.push(gua);
    }
    // 获取卦宫五行
    let palace_element = find_palace_element(&(zheng_gua.join(""))).unwrap_or("未知");
    // 处理正卦(卦象添加地支、五行和六亲)
    let (nei, wai) = process_gua(&zheng_gua, &mut zheng_xiang, palace_element);
    // 确定世爻和应爻的位置
    // let (shi_idx, ying_idx) =
    determine_shi_ying_indices(nei, wai, &mut zheng_xiang);
    // 需要判断冲克关系的爻的索引(世爻、应爻、动爻)考虑到暗动改成所有爻都判断
    // let mut shi_ying_dong: Vec<usize> = numbers
    //     .chars()
    //     .enumerate()
    //     .filter(|&(_, c)| c == '0' || c == '3')
    //     .map(|(i, _)| i)
    //     .collect();
    // shi_ying_dong.push(shi_idx);
    // shi_ying_dong.push(ying_idx);
    // 追加冲合关系
    append_chong_he_relations(
        &mut zheng_xiang,
        // shi_ying_dong,
        &month_ganzhi,
        &day_ganzhi,
    );
    //追加卦名
    if let Some(name) = find_palace_name(&zheng_gua.join("")) {
        zheng_xiang.push(name.to_string());
    }

    //根据动爻生成变卦
    let mut bian_gua = Vec::new();
    for c in numbers.chars() {
        let gua = match c {
            '0' => "1".to_string(),
            '1' => "1".to_string(),
            '2' => "2".to_string(),
            '3' => "2".to_string(),
            _ => "".to_string(),
        };
        bian_gua.push(gua);
    }
    // 需要绘制的变卦
    let mut bian_xiang = Vec::new();
    for num in &bian_gua {
        let gua = match num.as_str() {
            "1" => "⚋".to_string(),
            "2" => "⚊".to_string(),
            _ => "".to_string(),
        };
        bian_xiang.push(gua);
    }
    //处理变卦
    process_gua(&bian_gua, &mut bian_xiang, palace_element);
    //追加卦名
    if let Some(name) = find_palace_name(&bian_gua.join("")) {
        bian_xiang.push(name.to_string());
    }
    // 把正卦和变卦拼接起来
    let mut combined = Vec::new();
    for (gua, bian) in zheng_xiang.iter().zip(bian_xiang.iter()) {
        combined.push(format!("{}\t{}", gua, bian));
    }
    //json格式
    HttpResponse::Ok().json(GuaResponse {
        gua_xian: combined,
        year_ganzhi,
        month_ganzhi,
        day_ganzhi,
        hour_ganzhi,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate_gua_xian", web::post().to(generate_gua_xian))
            // 捕获static所有文件路径请求，注意这里的正则表达式
            .route("/{filename:.*}", web::get().to(embedded_file))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
