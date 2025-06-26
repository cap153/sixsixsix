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

// 存储一个完整卦的所有信息
#[derive(Debug)]
struct Gua {
    yao_xiang: [String; 6],    // 爻象, e.g., ["⚊", "⚋ o", ...]
    index_str: String,         // 卦的数字索引, e.g., "122111"
    shi_idx: Option<usize>,    // 世爻索引
    ying_idx: Option<usize>,   // 应爻索引
    dizhi: [&'static str; 6],  // 每爻的地支
    wuxing: [&'static str; 6], // 每爻的五行
    liuqin: [&'static str; 6], // 每爻的六亲
    palace_name: &'static str, // 卦名，e.g., "天风姤"
}

impl Gua {
    // 创建一个新的、未填充的Gua实例
    fn new(yao_xiang: [String; 6], index_str: String) -> Self {
        Gua {
            yao_xiang,
            index_str,
            shi_idx: None,
            ying_idx: None,
            dizhi: [""; 6],
            wuxing: [""; 6],
            liuqin: [""; 6],
            palace_name: "未知卦",
        }
    }
}

struct SixtyFourGua {
    // name: &'static str,
    index: &'static str,
    nei_dizhi: [&'static str; 3],
    nei_wuxing: [&'static str; 3],
    wai_dizhi: [&'static str; 3],
    wai_wuxing: [&'static str; 3],
    palace_element: &'static str,
    gua_name: [&'static str; 8],
    gua_index: [&'static str; 8],
}

const SIXTYFOURGUA_DATA: [SixtyFourGua; 8] = [
    SixtyFourGua {
        // name: "乾",
        index: "111",
        nei_dizhi: ["子", "寅", "辰"],
        nei_wuxing: ["水", "木", "土"],
        wai_dizhi: ["午", "申", "戌"],
        wai_wuxing: ["火", "金", "土"],
        palace_element: "金",
        gua_name: [
            "乾为天䷀(六冲)",
            "天风姤䷫",
            "天山遁䷠",
            "天地否䷋(六合)",
            "风地观䷓",
            "山地剥䷖",
            "火地晋䷢",
            "火天大有䷍",
        ],
        gua_index: [
            "111111", "211111", "221111", "222111", "222211", "222221", "222121", "111121",
        ],
    },
    SixtyFourGua {
        // name: "震",
        index: "122",
        nei_dizhi: ["子", "寅", "辰"],
        nei_wuxing: ["水", "木", "土"],
        wai_dizhi: ["午", "申", "戌"],
        wai_wuxing: ["火", "金", "土"],
        palace_element: "木",
        gua_name: [
            "震为雷䷲(六冲)",
            "雷地豫䷏(六合)",
            "雷水解䷧",
            "雷风恒䷟",
            "地风升䷭",
            "水风井䷯",
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
        nei_dizhi: ["寅", "辰", "午"],
        nei_wuxing: ["木", "土", "火"],
        wai_dizhi: ["申", "戌", "子"],
        wai_wuxing: ["金", "土", "水"],
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
        nei_dizhi: ["辰", "午", "申"],
        nei_wuxing: ["土", "火", "金"],
        wai_dizhi: ["戌", "子", "寅"],
        wai_wuxing: ["土", "水", "木"],
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
        nei_dizhi: ["未", "巳", "卯"],
        nei_wuxing: ["土", "火", "木"],
        wai_dizhi: ["丑", "亥", "酉"],
        wai_wuxing: ["土", "水", "金"],
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
        nei_dizhi: ["丑", "亥", "酉"],
        nei_wuxing: ["土", "水", "金"],
        wai_dizhi: ["未", "巳", "卯"],
        wai_wuxing: ["土", "火", "木"],
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
        nei_dizhi: ["卯", "丑", "亥"],
        nei_wuxing: ["木", "土", "水"],
        wai_dizhi: ["酉", "未", "巳"],
        wai_wuxing: ["金", "土", "火"],
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
        nei_dizhi: ["巳", "卯", "丑"],
        nei_wuxing: ["火", "木", "土"],
        wai_dizhi: ["亥", "酉", "未"],
        wai_wuxing: ["水", "金", "土"],
        palace_element: "金",
        gua_name: [
            "兑为泽䷹(六冲)",
            "泽水困䷮(六合)",
            "泽地萃䷬",
            "泽山咸䷞",
            "水山蹇䷦",
            "地山谦䷎",
            "雷山小过䷽",
            "雷泽归妹䷵",
        ],
        gua_index: [
            "112112", "212112", "222112", "221112", "221212", "221222", "221122", "112122",
        ],
    },
];

//获取干支信息，例如乙巳年 辛巳月 壬辰日 申时
fn get_ganzhi_info() -> (String, String, String, String) {
    let now = Local::now();
    let current_solar = solar::from_ymdhms(
        now.year() as i64,
        now.month() as i64,
        now.day() as i64,
        now.hour() as i64,
        now.minute() as i64,
        now.second() as i64,
    );
    let current_lunar = current_solar.get_lunar();
    (
        current_lunar.get_year_in_gan_zhi(),
        current_lunar.get_month_in_gan_zhi(),
        current_lunar.get_day_in_gan_zhi(),
        current_lunar.get_time_zhi(),
    )
}

// 辅助函数：根据地支获取五行
fn get_dizhi_wuxing(dizhi: &str) -> Option<&'static str> {
    match dizhi {
        "子" | "亥" => Some("水"),
        "寅" | "卯" => Some("木"),
        "巳" | "午" => Some("火"),
        "申" | "酉" => Some("金"),
        "辰" | "戌" | "丑" | "未" => Some("土"),
        _ => None,
    }
}

// 判断地支之间的冲合关系
fn get_chong_he_relation(dizhi1: &str, dizhi2: &str) -> Option<&'static str> {
    match (dizhi1, dizhi2) {
        ("子", "午")
        | ("午", "子")
        | ("丑", "未")
        | ("未", "丑")
        | ("寅", "申")
        | ("申", "寅")
        | ("卯", "酉")
        | ("酉", "卯")
        | ("辰", "戌")
        | ("戌", "辰")
        | ("巳", "亥")
        | ("亥", "巳") => Some("冲"),
        ("子", "丑")
        | ("丑", "子")
        | ("寅", "亥")
        | ("亥", "寅")
        | ("卯", "戌")
        | ("戌", "卯")
        | ("辰", "酉")
        | ("酉", "辰")
        | ("巳", "申")
        | ("申", "巳")
        | ("午", "未")
        | ("未", "午") => Some("合"),
        _ => None,
    }
}

// 判断地支之间的生克关系
fn get_sheng_ke_relation(element1: &str, element2: &str) -> Option<&'static str> {
    // 统一将输入转换为五行，如果输入本身就是五行，则直接使用
    let wuxing1 = get_dizhi_wuxing(element1).unwrap_or(element1);
    let wuxing2 = get_dizhi_wuxing(element2).unwrap_or(element2);

    match wuxing1 {
        "木" => match wuxing2 {
            "火" => Some("生"),
            "土" => Some("克"),
            _ => None,
        },
        "火" => match wuxing2 {
            "土" => Some("生"),
            "金" => Some("克"),
            _ => None,
        },
        "土" => match wuxing2 {
            "金" => Some("生"),
            "水" => Some("克"),
            _ => None,
        },
        "金" => match wuxing2 {
            "水" => Some("生"),
            "木" => Some("克"),
            _ => None,
        },
        "水" => match wuxing2 {
            "木" => Some("生"),
            "火" => Some("克"),
            _ => None,
        },
        _ => None,
    }
}

// 确定世应并填充 (仅用于正卦)
fn determine_shi_ying_indices(gua: &mut Gua) {
    let nei: Vec<char> = gua.index_str.chars().take(3).collect();
    let wai: Vec<char> = gua.index_str.chars().skip(3).take(3).collect();

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

    gua.shi_idx = Some(shi_idx);
    gua.ying_idx = Some(ying_idx);
}

// 填充五行
fn append_wuxing(gua: &mut Gua) {
    let (nei_index, wai_index) = gua.index_str.split_at(3);
    if let (Some(hun_tian_nei), Some(hun_tian_wai)) = (
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == nei_index),
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == wai_index),
    ) {
        for i in 0..3 {
            gua.wuxing[i] = hun_tian_nei.nei_wuxing[i];
            gua.wuxing[i + 3] = hun_tian_wai.wai_wuxing[i];
        }
    }
}

// 填充地支
fn append_dizhi(gua: &mut Gua) {
    let (nei_index, wai_index) = gua.index_str.split_at(3);
    if let (Some(hun_tian_nei), Some(hun_tian_wai)) = (
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == nei_index),
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == wai_index),
    ) {
        for i in 0..3 {
            gua.dizhi[i] = hun_tian_nei.nei_dizhi[i];
            gua.dizhi[i + 3] = hun_tian_wai.wai_dizhi[i];
        }
    }
}

// 填充六亲 (依赖五行和宫位五行)
fn append_liuqin(gua: &mut Gua, palace_element: &str) {
    for i in 0..6 {
        let wuxing_char = gua.wuxing[i].chars().next().unwrap_or(' ');
        let liuqin = match (palace_element, wuxing_char) {
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
        gua.liuqin[i] = liuqin;
    }
}

//找到卦宫对应的五行属性用于判断六亲
fn find_palace_element(gua_index: &str) -> Option<&'static str> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .any(|&idx| idx == gua_index)
            .then_some(gua.palace_element)
    })
}

//查找对应的卦宫名称
fn find_palace_name(gua_index: &str) -> Option<&'static str> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .position(|&idx| idx == gua_index)
            .map(|pos| gua.gua_name[pos])
    })
}

//处理卦
fn process_gua(gua: &mut Gua, palace_element: &'static str) {
    // 正卦和变卦的六亲都是根据正卦的宫位五行来定的，所以 palace_element 需要传入
    append_dizhi(gua);
    append_wuxing(gua);
    append_liuqin(gua, palace_element);
    gua.palace_name = find_palace_name(&gua.index_str).unwrap_or("未知卦");
}

async fn generate_gua_xian(req: web::Json<GuaRequest>) -> impl Responder {
    let numbers = &req.numbers;
    // 获取干支信息
    let (year_ganzhi, month_ganzhi, day_ganzhi, hour_ganzhi) = get_ganzhi_info();

    // 1. === 初始化正卦 (Zheng Gua) ===
    let mut zheng_yao_xiang = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut zheng_index_vec = Vec::with_capacity(6);
    for (i, c) in numbers.chars().enumerate() {
        let (yao, index_char) = match c {
            '0' => ("⚋ x".to_string(), "2"),
            '1' => ("⚊".to_string(), "1"),
            '2' => ("⚋".to_string(), "2"),
            '3' => ("⚊ o".to_string(), "1"),
            _ => ("".to_string(), ""),
        };
        if i < 6 {
            zheng_yao_xiang[i] = yao;
        }
        zheng_index_vec.push(index_char);
    }
    let mut zheng_gua = Gua::new(zheng_yao_xiang, zheng_index_vec.join(""));

    // 2. === 初始化变卦 (Bian Gua) ===
    let mut bian_yao_xiang = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut bian_index_vec = Vec::with_capacity(6);
    for (i, c) in numbers.chars().enumerate() {
        let (yao, index_char) = match c {
            '0' => ("⚊".to_string(), "1"),
            '1' => ("⚊".to_string(), "1"),
            '2' => ("⚋".to_string(), "2"),
            '3' => ("⚋".to_string(), "2"),
            _ => ("".to_string(), ""),
        };
        if i < 6 {
            bian_yao_xiang[i] = yao;
        }
        bian_index_vec.push(index_char);
    }
    let mut bian_gua = Gua::new(bian_yao_xiang, bian_index_vec.join(""));

    // 3. === 数据处理和填充 ===
    let palace_element = find_palace_element(&zheng_gua.index_str).unwrap_or("未知");

    // 处理正卦
    process_gua(&mut zheng_gua, palace_element);
    determine_shi_ying_indices(&mut zheng_gua); // 世应只在正卦上

    // 处理变卦 (使用正卦的宫位五行)
    process_gua(&mut bian_gua, palace_element);

    // 获取月和日的地支
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

    // 4. === 格式化最终输出 ===
    let mut combined_lines = Vec::with_capacity(7);

    for i in 0..6 {
        // 格式化正卦的每一爻
        let mut zheng_line = format!(
            "{}{}{}{}",
            zheng_gua.liuqin[i], zheng_gua.dizhi[i], zheng_gua.wuxing[i], zheng_gua.yao_xiang[i]
        );
        if zheng_gua.shi_idx == Some(i) {
            zheng_line.push_str(" 世");
        }
        if zheng_gua.ying_idx == Some(i) {
            zheng_line.push_str(" 应");
        }
        // 判断并追加冲合关系 (月对爻)
        if let Some(relation) = get_chong_he_relation(zheng_gua.dizhi[i], month_dizhi) {
            zheng_line.push_str(&format!(" 月{}", relation));
        }
        // 判断并追加冲合关系 (日对爻)
        if let Some(relation) = get_chong_he_relation(zheng_gua.dizhi[i], day_dizhi) {
            zheng_line.push_str(&format!(" 日{}", relation));
        }
        // 判断并追加生克关系 (月对爻)
        if let Some(relation) = get_sheng_ke_relation(month_dizhi, zheng_gua.dizhi[i]) {
            zheng_line.push_str(&format!(" 月{}", relation));
        }
        // 判断并追加生克关系 (日对爻)
        if let Some(relation) = get_sheng_ke_relation(day_dizhi, zheng_gua.dizhi[i]) {
            zheng_line.push_str(&format!(" 日{}", relation));
        }

        // 格式化变卦的每一爻
        let bian_line = format!(
            "{}{}{}{}",
            bian_gua.liuqin[i], bian_gua.dizhi[i], bian_gua.wuxing[i], bian_gua.yao_xiang[i]
        );

        combined_lines.push(format!("{}\t{}", zheng_line, bian_line));
    }
    // 添加最后的卦名行
    combined_lines.push(format!(
        "{}\t{}",
        zheng_gua.palace_name, bian_gua.palace_name
    ));

    HttpResponse::Ok().json(GuaResponse {
        gua_xian: combined_lines,
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
