use std::sync::OnceLock;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

// 存储卦宫的五行属性，只会被初始化1次
static PALACE_ELEMENT: OnceLock<&'static str> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct GuaRequest {
    numbers: String,
}

#[derive(Serialize, Deserialize)]
struct GuaResponse {
    gua_xian: Vec<String>,
}

struct SixtyFourGua {
    // name: &'static str,
    index: &'static str,
    nei: [&'static str; 3],
    wai: [&'static str; 3],
    palace_element: &'static str,
    // palace_gua: [&'static str; 8],
    palace_gua_index: [&'static str; 8],
}

// 新增八个本宫卦的地支和五行数据
const SIXTYFOURGUA_DATA: [SixtyFourGua; 8] = [
    SixtyFourGua {
        // name: "乾",
        index: "111",
        nei: ["子水", "寅木", "辰土"],
        wai: ["午火", "申金", "戌土"],
        palace_element: "金",
        // palace_gua: ["乾为天","天风姤","天山遁","天地否","风地观","山地剥","火地晋","火天大有"],
        palace_gua_index: [
            "111111", "211111", "221111", "222111", "222211", "222221", "222121", "111121",
        ],
    },
    SixtyFourGua {
        // name: "震",
        index: "122",
        nei: ["子水", "寅木", "辰土"],
        wai: ["午火", "申金", "戌土"],
        palace_element: "木",
        // palace_gua: ["震为雷","雷地豫","雷水解","雷风恒","地风升","水风井","泽风大过","泽雷随"],
        palace_gua_index: [
            "122122", "222122", "212122", "211122", "211222", "211212", "211112", "122112",
        ],
    },
    SixtyFourGua {
        // name: "坎",
        index: "212",
        nei: ["寅木", "辰土", "午火"],
        wai: ["申金", "戌土", "子水"],
        palace_element: "水",
        // palace_gua: ["坎为水","水泽节","水雷屯","水火既济","泽火革","雷火丰","地火明夷","地水师"],
        palace_gua_index: [
            "212212", "112212", "122212", "121212", "121112", "121122", "121222", "212222",
        ],
    },
    SixtyFourGua {
        // name: "艮",
        index: "221",
        nei: ["辰土", "午火", "申金"],
        wai: ["戌土", "子水", "寅木"],
        palace_element: "土",
        // palace_gua: ["艮为山","山火贲","山天大畜","山泽损","火泽睽","天泽履","风泽中孚","风山渐"],
        palace_gua_index: [
            "221221", "121221", "111221", "112221", "112121", "112111", "112211", "221211",
        ],
    },
    SixtyFourGua {
        // name: "坤",
        index: "222",
        nei: ["未土", "巳火", "卯木"],
        wai: ["丑土", "亥水", "酉金"],
        palace_element: "土",
        // palace_gua: ["坤为地","地雷复","地泽临","地天泰","雷天大壮","泽天夬","水天需","水地比"],
        palace_gua_index: [
            "222222", "122222", "112222", "111222", "111122", "111112", "111212", "222212",
        ],
    },
    SixtyFourGua {
        // name: "巽",
        index: "211",
        nei: ["丑土", "亥水", "酉金"],
        wai: ["未土", "巳火", "卯木"],
        palace_element: "木",
        // palace_gua: ["巽为风","风天小畜","风火家人","风雷益","天雷无妄","火雷噬嗑","山雷颐","山风蛊"],
        palace_gua_index: [
            "211211", "111211", "121211", "122211", "122111", "122121", "122221", "211221",
        ],
    },
    SixtyFourGua {
        // name: "离",
        index: "121",
        nei: ["卯木", "丑土", "亥水"],
        wai: ["酉金", "未土", "巳火"],
        palace_element: "火",
        // palace_gua: ["离为火","火山旅","火风鼎","火水未济","山水蒙","风水涣","天水讼","天火同人"],
        palace_gua_index: [
            "121121", "221121", "211121", "212121", "212221", "212211", "212111", "121111",
        ],
    },
    SixtyFourGua {
        // name: "兑",
        index: "112",
        nei: ["巳火", "卯木", "丑土"],
        wai: ["亥水", "酉金", "未土"],
        palace_element: "金",
        // palace_gua: ["兑为泽","泽水困","泽地萃","泽山咸","水山蹇","地山谦","雷山小过","雷泽归妹"],
        palace_gua_index: [
            "112112", "212112", "222112", "221112", "221212", "221222", "221122", "112122",
        ],
    },
];

// 确定世爻和应爻的位置
fn determine_shi_ying_indices(nei: &[String], wai: &[String], gua_xian: &mut Vec<String>) {
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
}

// 新增方法：处理地支和五行追加逻辑
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
fn find_palace_element(nei: &[String], wai: &[String]) -> Option<&'static str> {
    let combined: String = nei.iter().chain(wai).map(|s| s.as_str()).collect();

    SIXTYFOURGUA_DATA.iter().find_map(|gua| {
        gua.palace_gua_index
            .iter()
            .find(|&&idx| idx == combined)
            .map(|_| gua.palace_element)
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

fn process_gua(gua: &[String], xiang: &mut Vec<String>) {
    // nei表示内卦，wai表示外卦
    let (nei, wai) = gua.split_at(3);

    // 确定世爻和应爻的位置
    determine_shi_ying_indices(nei, wai, xiang);

    // 追加地支和五行
    append_dizhi_wuxing(nei, wai, xiang);

    // 获取卦宫的五行，变卦中六亲须按正卦而推，因此使用的五行是一样的只会初始化1次
    let palace_element = PALACE_ELEMENT.get_or_init(|| find_palace_element(nei, wai).unwrap_or("未知"));

    // 判断六亲
    append_liu_qin(palace_element, xiang);
}

async fn generate_gua_xian(req: web::Json<GuaRequest>) -> impl Responder {
    let numbers = &req.numbers;
    let mut zheng_xiang = Vec::new();

    // 需要绘制的正卦
    for c in numbers.chars() {
        let gua = match c {
            '0' => "━━ ━━ x".to_string(),
            '1' => "━━━━━".to_string(),
            '2' => "━━ ━━".to_string(),
            '3' => "━━━━━ o".to_string(),
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

    //处理正卦
    process_gua(&zheng_gua, &mut zheng_xiang);

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
            "1" => "━━━━━".to_string(),
            "2" => "━━ ━━".to_string(),
            _ => "".to_string(),
        };
        bian_xiang.push(gua);
    }

    //处理变卦
    process_gua(&bian_gua, &mut bian_xiang);

    // 把正卦和变卦拼接起来
    let mut combined = Vec::new();
    for (gua, bian) in zheng_xiang.iter().zip(bian_xiang.iter()) {
        combined.push(format!("{}\t{}", gua, bian));
    }

    HttpResponse::Ok().json(GuaResponse { gua_xian: combined })
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
