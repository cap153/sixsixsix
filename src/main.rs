use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, Local, Timelike};
use lunar_rust::{
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// 表示五行（金、木、水、火、土）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WuXing {
    Jin,
    Mu,
    Shui,
    Huo,
    Tu,
}

// 实现 Display trait，用于将五行枚举转换为可打印的汉字字符串（如“金”）。
impl Display for WuXing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WuXing::Jin => "金",
                WuXing::Mu => "木",
                WuXing::Shui => "水",
                WuXing::Huo => "火",
                WuXing::Tu => "土",
            }
        )
    }
}

/// 表示十二地支（子、丑、寅等）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiZhi {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

impl DiZhi {
    /// 根据地支推算其对应的五行。
    /// 这是核心规则之一，将地支与五行关联起来。
    fn wuxing(&self) -> WuXing {
        match self {
            DiZhi::Zi | DiZhi::Hai => WuXing::Shui,
            DiZhi::Yin | DiZhi::Mao => WuXing::Mu,
            DiZhi::Si | DiZhi::Wu => WuXing::Huo,
            DiZhi::Shen | DiZhi::You => WuXing::Jin,
            DiZhi::Chen | DiZhi::Xu | DiZhi::Chou | DiZhi::Wei => WuXing::Tu,
        }
    }
}

// 实现 TryFrom<&str> trait，用于安全地从字符串（如"子"）创建DiZhi枚举。
// 主要用于处理从lunar_rust库获取的干支字符串。
impl TryFrom<&str> for DiZhi {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "子" => Ok(DiZhi::Zi),
            "丑" => Ok(DiZhi::Chou),
            "寅" => Ok(DiZhi::Yin),
            "卯" => Ok(DiZhi::Mao),
            "辰" => Ok(DiZhi::Chen),
            "巳" => Ok(DiZhi::Si),
            "午" => Ok(DiZhi::Wu),
            "未" => Ok(DiZhi::Wei),
            "申" => Ok(DiZhi::Shen),
            "酉" => Ok(DiZhi::You),
            "戌" => Ok(DiZhi::Xu),
            "亥" => Ok(DiZhi::Hai),
            _ => Err("Invalid DiZhi string"),
        }
    }
}

// 实现 Display trait，用于将地支枚举转换为可打印的汉字字符串（如“子”）。
impl Display for DiZhi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DiZhi::Zi => "子",
                DiZhi::Chou => "丑",
                DiZhi::Yin => "寅",
                DiZhi::Mao => "卯",
                DiZhi::Chen => "辰",
                DiZhi::Si => "巳",
                DiZhi::Wu => "午",
                DiZhi::Wei => "未",
                DiZhi::Shen => "申",
                DiZhi::You => "酉",
                DiZhi::Xu => "戌",
                DiZhi::Hai => "亥",
            }
        )
    }
}

/// 表示六亲（兄弟、子孙、妻财、官鬼、父母）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiuQin {
    XiongDi,
    ZiSun,
    QiCai,
    GuanGui,
    FuMu,
}

// 实现 Display trait，用于将六亲枚举转换为可打印的汉字字符串（如“兄弟”）。
impl Display for LiuQin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LiuQin::XiongDi => "兄弟",
                LiuQin::ZiSun => "子孙",
                LiuQin::QiCai => "妻财",
                LiuQin::GuanGui => "官鬼",
                LiuQin::FuMu => "父母",
            }
        )
    }
}

/// 表示爻的四种状态（动爻与静爻）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Yao {
    YinChanging,  // 0 -> ⚋ x (老阴，变阳)
    YangStatic,   // 1 -> ⚊   (少阳，不变)
    YinStatic,    // 2 -> ⚋   (少阴，不变)
    YangChanging, // 3 -> ⚊ o  (老阳，变阴)
}
impl Yao {
    /// 获取爻的图形表示（如 "⚊" 或 "⚋ x"）。
    fn xiang(&self) -> &'static str {
        match self {
            Yao::YinChanging => "⚋ x",
            Yao::YangStatic => "⚊",
            Yao::YinStatic => "⚋",
            Yao::YangChanging => "⚊ o",
        }
    }

    /// 获取爻对应的数字索引字符（'1'代表阳，'2'代表阴）。
    /// 用于构成卦的六位数字索引，例如 "111111"。
    fn index_char(&self) -> char {
        match self {
            Yao::YinChanging | Yao::YinStatic => '2',
            Yao::YangStatic | Yao::YangChanging => '1',
        }
    }

    /// 获取此爻变化后的爻（动爻变为其相反的静爻，静爻不变）。
    /// 用于从正卦计算变卦。
    fn to_bian_yao(&self) -> Self {
        match self {
            Yao::YinChanging => Yao::YangStatic,
            Yao::YangChanging => Yao::YinStatic,
            static_yao => *static_yao, // 不变的爻保持原样
        }
    }
}

// 从前端传入的字符（'0'~'3'）直接创建Yao枚举。
impl From<char> for Yao {
    fn from(c: char) -> Self {
        match c {
            '0' => Yao::YinChanging,
            '1' => Yao::YangStatic,
            '2' => Yao::YinStatic,
            '3' => Yao::YangChanging,
            // 在实际应用中，这里可以返回Result而不是panic，但对于内部逻辑此方式更简洁
            _ => panic!("Invalid character for Yao conversion"),
        }
    }
}

// 表示一个爻在卦中的角色（世、应或普通）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
enum YaoRole {
    Shi,    // 世爻
    Ying,   // 应爻
    Normal, // 普通爻
}

/// 表示地支间的冲或合关系。
#[derive(Debug, Clone, Copy)]
enum ChongHe {
    Chong,
    He,
}
// 实现 Display trait，用于打印 "冲" 或 "合"。
impl Display for ChongHe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChongHe::Chong => "冲",
                ChongHe::He => "合",
            }
        )
    }
}

/// 表示五行间的生或克关系。
#[derive(Debug, Clone, Copy)]
enum ShengKe {
    Sheng,
    Ke,
}
// 实现 Display trait，用于打印 "生" 或 "克"。
impl Display for ShengKe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShengKe::Sheng => "生",
                ShengKe::Ke => "克",
            }
        )
    }
}

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

// 用于表示单行卦爻信息的结构体
#[derive(Serialize)]
struct GuaLineResponse {
    base_text: String,
    role: YaoRole,
    zheng_relations_text: String,
    // 变卦部分可以简化，因为它没有角色和关系
    bian_text: String,
    bian_relations_text: String,
    is_changing: bool,
}

#[derive(Serialize)]
struct GuaResponse {
    gua_lines: Vec<GuaLineResponse>,
    year_ganzhi: String,
    month_ganzhi: String,
    day_ganzhi: String,
    hour_ganzhi: String,
}

#[derive(Deserialize)]
struct GuaRequest {
    numbers: String,
}

// 存储一个完整卦的所有信息
#[derive(Debug)]
struct Gua {
    yao_xiang: [Yao; 6],       // 爻象, e.g., ["⚊", "⚋ o", ...]
    index_str: String,         // 卦的数字索引, e.g., "122111"
    yao_roles: [YaoRole; 6],   // 每个爻都有一个角色，世、应或普通
    dizhi: [DiZhi; 6],         // 每爻的地支
    wuxing: [WuXing; 6],       // 每爻的五行
    liuqin: [LiuQin; 6],       // 每爻的六亲
    palace_name: &'static str, // 卦名，e.g., "天风姤"
}
impl Gua {
    // 创建一个新的、未填充的Gua实例
    fn new(yao_xiang: [Yao; 6]) -> Self {
        let index_str = yao_xiang.iter().map(|y| y.index_char()).collect();
        Gua {
            yao_xiang,
            index_str,
            // 初始化时，所有爻都是普通角色
            yao_roles: [YaoRole::Normal; 6],
            // 使用 Copy 特性可以直接创建数组，无需手动填充
            dizhi: [DiZhi::Zi; 6],
            wuxing: [WuXing::Jin; 6],
            liuqin: [LiuQin::XiongDi; 6],
            palace_name: "未知卦",
        }
    }
}

// 存储六十四卦要用到的信息
struct SixtyFourGua {
    // name: &'static str,
    index: &'static str,
    nei_dizhi: [DiZhi; 3],
    // nei_wuxing: [WuXing; 3],
    wai_dizhi: [DiZhi; 3],
    // wai_wuxing: [WuXing; 3],
    palace_element: WuXing,
    gua_name: [&'static str; 8],
    gua_index: [&'static str; 8],
}

const SIXTYFOURGUA_DATA: [SixtyFourGua; 8] = [
    SixtyFourGua {
        // name: "乾",
        index: "111",
        nei_dizhi: [DiZhi::Zi, DiZhi::Yin, DiZhi::Chen],
        // nei_wuxing: [WuXing::Shui, WuXing::Mu, WuXing::Tu],
        wai_dizhi: [DiZhi::Wu, DiZhi::Shen, DiZhi::Xu],
        // wai_wuxing: [WuXing::Huo, WuXing::Jin, WuXing::Tu],
        palace_element: WuXing::Jin,
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
        nei_dizhi: [DiZhi::Zi, DiZhi::Yin, DiZhi::Chen],
        // nei_wuxing: [WuXing::Shui, WuXing::Mu, WuXing::Tu],
        wai_dizhi: [DiZhi::Wu, DiZhi::Shen, DiZhi::Xu],
        // wai_wuxing: [WuXing::Huo, WuXing::Jin, WuXing::Tu],
        palace_element: WuXing::Mu,
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
        nei_dizhi: [DiZhi::Yin, DiZhi::Chen, DiZhi::Wu],
        // nei_wuxing: [WuXing::Mu, WuXing::Tu, WuXing::Huo],
        wai_dizhi: [DiZhi::Shen, DiZhi::Xu, DiZhi::Zi],
        // wai_wuxing: [WuXing::Jin, WuXing::Tu, WuXing::Shui],
        palace_element: WuXing::Shui,
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
        nei_dizhi: [DiZhi::Chen, DiZhi::Wu, DiZhi::Shen],
        // nei_wuxing: [WuXing::Tu, WuXing::Huo, WuXing::Jin],
        wai_dizhi: [DiZhi::Xu, DiZhi::Zi, DiZhi::Yin],
        // wai_wuxing: [WuXing::Tu, WuXing::Shui, WuXing::Mu],
        palace_element: WuXing::Tu,
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
        nei_dizhi: [DiZhi::Wei, DiZhi::Si, DiZhi::Mao],
        // nei_wuxing: [WuXing::Tu, WuXing::Huo, WuXing::Mu],
        wai_dizhi: [DiZhi::Chou, DiZhi::Hai, DiZhi::You],
        // wai_wuxing: [WuXing::Tu, WuXing::Shui, WuXing::Jin],
        palace_element: WuXing::Tu,
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
        nei_dizhi: [DiZhi::Chou, DiZhi::Hai, DiZhi::You],
        // nei_wuxing: [WuXing::Tu, WuXing::Shui, WuXing::Jin],
        wai_dizhi: [DiZhi::Wei, DiZhi::Si, DiZhi::Mao],
        // wai_wuxing: [WuXing::Tu, WuXing::Huo, WuXing::Mu],
        palace_element: WuXing::Mu,
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
        nei_dizhi: [DiZhi::Mao, DiZhi::Chou, DiZhi::Hai],
        // nei_wuxing: [WuXing::Mu, WuXing::Tu, WuXing::Shui],
        wai_dizhi: [DiZhi::You, DiZhi::Wei, DiZhi::Si],
        // wai_wuxing: [WuXing::Jin, WuXing::Tu, WuXing::Huo],
        palace_element: WuXing::Huo,
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
        nei_dizhi: [DiZhi::Si, DiZhi::Mao, DiZhi::Chou],
        // nei_wuxing: [WuXing::Huo, WuXing::Mu, WuXing::Tu],
        wai_dizhi: [DiZhi::Hai, DiZhi::You, DiZhi::Wei],
        // wai_wuxing: [WuXing::Shui, WuXing::Jin, WuXing::Tu],
        palace_element: WuXing::Jin,
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

// 获取干支信息，例如乙巳年 辛巳月 壬辰日 申时
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

// 判断地支之间的冲合关系
fn get_chong_he_relation(dizhi1: DiZhi, dizhi2: DiZhi) -> Option<ChongHe> {
    use {ChongHe::*, DiZhi::*};
    match (dizhi1, dizhi2) {
        (Zi, Wu)
        | (Wu, Zi)
        | (Chou, Wei)
        | (Wei, Chou)
        | (Yin, Shen)
        | (Shen, Yin)
        | (Mao, You)
        | (You, Mao)
        | (Chen, Xu)
        | (Xu, Chen)
        | (Si, Hai)
        | (Hai, Si) => Some(Chong),
        (Zi, Chou)
        | (Chou, Zi)
        | (Yin, Hai)
        | (Hai, Yin)
        | (Mao, Xu)
        | (Xu, Mao)
        | (Chen, You)
        | (You, Chen)
        | (Si, Shen)
        | (Shen, Si)
        | (Wu, Wei)
        | (Wei, Wu) => Some(He),
        _ => None,
    }
}

// 判断地支之间的生克关系
fn get_sheng_ke_relation(wuxing1: WuXing, wuxing2: WuXing) -> Option<ShengKe> {
    use {ShengKe::*, WuXing::*};
    match (wuxing1, wuxing2) {
        (Mu, Huo) | (Huo, Tu) | (Tu, Jin) | (Jin, Shui) | (Shui, Mu) => Some(Sheng),
        (Mu, Tu) | (Huo, Jin) | (Tu, Shui) | (Jin, Mu) | (Shui, Huo) => Some(Ke),
        _ => None,
    }
}

// 确定世应并填充 (仅用于正卦)
fn determine_yao_roles(gua: &mut Gua) {
    let nei = &gua.yao_xiang[0..3];
    let wai = &gua.yao_xiang[3..6];

    let (shi_idx, ying_idx) = if nei[2].index_char() == wai[2].index_char()
        && nei[0].index_char() != wai[0].index_char()
        && nei[1].index_char() != wai[1].index_char()
    {
        (1, 4)
    } else if nei[2].index_char() != wai[2].index_char()
        && nei[0].index_char() == wai[0].index_char()
        && nei[1].index_char() == wai[1].index_char()
    {
        (4, 1)
    } else if nei[0].index_char() == wai[0].index_char()
        && nei[1].index_char() != wai[1].index_char()
        && nei[2].index_char() != wai[2].index_char()
    {
        (3, 0)
    } else if nei[0].index_char() != wai[0].index_char()
        && nei[1].index_char() == wai[1].index_char()
        && nei[2].index_char() == wai[2].index_char()
    {
        (0, 3)
    } else if nei[1].index_char() == wai[1].index_char()
        && nei[0].index_char() != wai[0].index_char()
        && nei[2].index_char() != wai[2].index_char()
    {
        (3, 0)
    } else if nei[1].index_char() != wai[1].index_char()
        && nei[0].index_char() == wai[0].index_char()
        && nei[2].index_char() == wai[2].index_char()
    {
        (2, 5)
    } else if nei[0].index_char() == wai[0].index_char()
        && nei[1].index_char() == wai[1].index_char()
        && nei[2].index_char() == wai[2].index_char()
    {
        (5, 2)
    } else {
        (2, 5)
    };

    gua.yao_roles[shi_idx] = YaoRole::Shi;
    gua.yao_roles[ying_idx] = YaoRole::Ying;
}

// 填充五行
fn append_wuxing(gua: &mut Gua) {
    for i in 0..6 {
        gua.wuxing[i] = gua.dizhi[i].wuxing();
    }
}

// 填充地支
fn append_dizhi(gua: &mut Gua) {
    let (nei_index, wai_index) = gua.index_str.split_at(3);
    if let (Some(hun_tian_nei), Some(hun_tian_wai)) = (
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == nei_index),
        SIXTYFOURGUA_DATA.iter().find(|h| h.index == wai_index),
    ) {
        gua.dizhi[0..3].copy_from_slice(&hun_tian_nei.nei_dizhi);
        gua.dizhi[3..6].copy_from_slice(&hun_tian_wai.wai_dizhi);
    }
}

// 填充六亲 (依赖五行和宫位五行)
fn append_liuqin(gua: &mut Gua, palace_element: WuXing) {
    use LiuQin::*;
    for i in 0..6 {
        gua.liuqin[i] = match (palace_element, gua.wuxing[i]) {
            (WuXing::Jin, WuXing::Jin)
            | (WuXing::Mu, WuXing::Mu)
            | (WuXing::Shui, WuXing::Shui)
            | (WuXing::Huo, WuXing::Huo)
            | (WuXing::Tu, WuXing::Tu) => XiongDi,
            (WuXing::Jin, WuXing::Shui)
            | (WuXing::Mu, WuXing::Huo)
            | (WuXing::Shui, WuXing::Mu)
            | (WuXing::Huo, WuXing::Tu)
            | (WuXing::Tu, WuXing::Jin) => ZiSun,
            (WuXing::Jin, WuXing::Mu)
            | (WuXing::Mu, WuXing::Tu)
            | (WuXing::Shui, WuXing::Huo)
            | (WuXing::Huo, WuXing::Jin)
            | (WuXing::Tu, WuXing::Shui) => QiCai,
            (WuXing::Jin, WuXing::Huo)
            | (WuXing::Mu, WuXing::Jin)
            | (WuXing::Shui, WuXing::Tu)
            | (WuXing::Huo, WuXing::Shui)
            | (WuXing::Tu, WuXing::Mu) => GuanGui,
            (WuXing::Jin, WuXing::Tu)
            | (WuXing::Mu, WuXing::Shui)
            | (WuXing::Shui, WuXing::Jin)
            | (WuXing::Huo, WuXing::Mu)
            | (WuXing::Tu, WuXing::Huo) => FuMu,
        };
    }
}

// 找到卦宫对应的五行属性用于判断六亲
fn find_palace_element(gua_index: &str) -> Option<WuXing> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .any(|&idx| idx == gua_index)
            .then_some(gua.palace_element)
    })
}

// 查找对应的卦宫名称
fn find_palace_name(gua_index: &str) -> Option<&'static str> {
    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.gua_index
            .iter()
            .position(|&idx| idx == gua_index)
            .map(|pos| gua.gua_name[pos])
    })
}

// 处理卦
fn process_gua(gua: &mut Gua, palace_element: WuXing) {
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
    let mut zheng_yao_xiang = [Yao::YinStatic; 6];
    for (i, c) in numbers.chars().enumerate().take(6) {
        zheng_yao_xiang[i] = Yao::from(c);
    }
    let mut zheng_gua = Gua::new(zheng_yao_xiang);

    // 2. === 初始化变卦 (Bian Gua) ===
    let mut bian_yao_xiang = [Yao::YinStatic; 6];
    for i in 0..6 {
        bian_yao_xiang[i] = zheng_gua.yao_xiang[i].to_bian_yao();
    }
    let mut bian_gua = Gua::new(bian_yao_xiang);

    // 3. === 数据处理和填充 ===
    let palace_element = find_palace_element(&zheng_gua.index_str).unwrap_or(WuXing::Jin); // 默认为金

    // 处理正卦
    process_gua(&mut zheng_gua, palace_element);
    determine_yao_roles(&mut zheng_gua); // 世应只在正卦上

    // 处理变卦 (使用正卦的宫位五行)
    process_gua(&mut bian_gua, palace_element);

    // 获取月和日的地支
    let month_dizhi_str = month_ganzhi
        .chars()
        .nth(1)
        .map(|c| &month_ganzhi[c.len_utf8()..])
        .unwrap_or("");
    let day_dizhi_str = day_ganzhi
        .chars()
        .nth(1)
        .map(|c| &day_ganzhi[c.len_utf8()..])
        .unwrap_or("");
    let month_dizhi = DiZhi::try_from(month_dizhi_str).ok();
    let day_dizhi = DiZhi::try_from(day_dizhi_str).ok();

    // 4. === 格式化最终输出 ===
    let mut gua_lines = Vec::with_capacity(7);

    for i in 0..6 {
        // 构建正卦的基础文本（不含世应）
        let base_text = format!(
            "{}{}{}{}",
            zheng_gua.liuqin[i],
            zheng_gua.dizhi[i],
            zheng_gua.wuxing[i],
            zheng_gua.yao_xiang[i].xiang()
        );
        // 构建正卦关系文本
        let mut zheng_relations_text = String::new();
        // 判断月的影响
        if let Some(md) = month_dizhi {
            // 判断并追加冲合关系 (月对爻)
            if let Some(relation) = get_chong_he_relation(zheng_gua.dizhi[i], md) {
                zheng_relations_text.push_str(&format!(" 月{}", relation));
            }
            // 判断并追加生克关系 (月对爻)
            if let Some(relation) = get_sheng_ke_relation(md.wuxing(), zheng_gua.wuxing[i]) {
                zheng_relations_text.push_str(&format!(" 月{}", relation));
            }
        }
        // 判断日的影响
        if let Some(dd) = day_dizhi {
            // 判断并追加冲合关系 (日对爻)
            if let Some(relation) = get_chong_he_relation(zheng_gua.dizhi[i], dd) {
                zheng_relations_text.push_str(&format!(" 日{}", relation));
            }
            // 判断并追加生克关系 (日对爻)
            if let Some(relation) = get_sheng_ke_relation(dd.wuxing(), zheng_gua.wuxing[i]) {
                zheng_relations_text.push_str(&format!(" 日{}", relation));
            }
        }

        // 构建变卦关系文本
        let mut bian_relations_text = String::new();
        // 只有当正卦的爻是动爻时，才计算回头关系
        let is_changing = matches!(zheng_gua.yao_xiang[i], Yao::YinChanging | Yao::YangChanging);
        if is_changing {
            // 判断月的影响
            if let Some(md) = month_dizhi {
                // 判断并追加冲合关系 (月对爻)
                if let Some(relation) = get_chong_he_relation(bian_gua.dizhi[i], md) {
                    bian_relations_text.push_str(&format!(" 月{}", relation));
                }
                // 判断并追加生克关系 (月对爻)
                if let Some(relation) = get_sheng_ke_relation(md.wuxing(), bian_gua.wuxing[i]) {
                    bian_relations_text.push_str(&format!(" 月{}", relation));
                }
            }
            // 判断日的影响
            if let Some(dd) = day_dizhi {
                // 判断并追加冲合关系 (日对爻)
                if let Some(relation) = get_chong_he_relation(bian_gua.dizhi[i], dd) {
                    bian_relations_text.push_str(&format!(" 日{}", relation));
                }
                // 判断并追加生克关系 (日对爻)
                if let Some(relation) = get_sheng_ke_relation(dd.wuxing(), bian_gua.wuxing[i]) {
                    bian_relations_text.push_str(&format!(" 日{}", relation));
                }
            }
            // 变爻回头生克 (变爻的五行 -> 正爻的五行)
            if let Some(relation) = get_sheng_ke_relation(bian_gua.wuxing[i], zheng_gua.wuxing[i]) {
                bian_relations_text.push_str(&format!(" 回头{}", relation));
            }
            // 变爻回头冲合 (变爻的地支 vs 正爻的地支)
            if let Some(relation) = get_chong_he_relation(bian_gua.dizhi[i], zheng_gua.dizhi[i]) {
                bian_relations_text.push_str(&format!("{}", relation));
            }
        }

        // 构建变卦的基础文本
        let bian_text = format!(
            "{}{}{}{}",
            bian_gua.liuqin[i],
            bian_gua.dizhi[i],
            bian_gua.wuxing[i],
            bian_gua.yao_xiang[i].xiang()
        );

        gua_lines.push(GuaLineResponse {
            base_text,
            role: zheng_gua.yao_roles[i],
            zheng_relations_text,
            bian_text,
            bian_relations_text,
            is_changing,
        });
    }
    // 单独处理卦名 离为火䷝(六冲)震为雷䷲(六) 等
    let name_line = GuaLineResponse {
        base_text: zheng_gua.palace_name.to_string(),
        role: YaoRole::Normal,
        zheng_relations_text: String::new(),
        bian_text: bian_gua.palace_name.to_string(),
        bian_relations_text: String::new(),
        is_changing: false,
    };
    gua_lines.push(name_line);

    HttpResponse::Ok().json(GuaResponse {
        gua_lines,
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
    .bind("[::]:8080")?
    .run()
    .await
}
