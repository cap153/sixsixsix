你好，我需要写一个六爻排盘网页，实现输入阳面个数根据我提供的逻辑排盘，并有调用api访问大语言模型占卜分析的功能，打算使用Rust + Actix实现，现在开始逐步完成前端界面编写：存在提示“输入六次占卜阳面的个数（0-3），系统将自动生成六爻卦象和详细解读“；存在文本框用于输入阳面个数；存在生成卦象按钮。
---
现在已经完成index.html的编写，需要编写css美化一下。
---
现在需要完成排盘的逻辑，打算使用Rust + Actix实现，已经初始化rust项目，根据已经实现的文本框输入的阳面个数，先实现点击'生成卦象'按钮生成简单的排盘：阳面个数为2使用'━━ ━━'，阳面个数为1使用'━━━━━'，阳面个数为3使用'━━━━━ o'，阳面个数为0使用'━━ ━━ x'，生成卦象的逻辑在rust代码中实现，并把卦象传递给前端从下到上排出6个爻在网页绘制出来。(阅读现有代码并完善功能,输入限制已经在index.html实现,不要重复)。
---
现在已经生成简单卦象，需要编辑css美化一下，适配竖屏。
---
接下来要确定世爻和应爻的位置，在rust代码中对应的爻追加。例如"━━ ━━ x 世","━━━━━ 应"。寻找世爻和应爻对应逻辑如下：
6个爻先保存成一个数组b(原来就是数组则无需额外创建，动爻需过滤符号,可考虑直接用输入的阳面个数转换)，然后前3个保存为数组b，后3个保存为数组c
b[2] == c[2] 与 b[0] != c[0] 与 b[1] != c[1] 都为真则a[1]为世爻a[4]为应爻
b[2] != c[2] 与 b[0] == c[0] 与 b[1] == c[1] 都为真则a[4]为世爻a[1]为应爻

b[0] == c[0] 与 b[1] != c[1] 与 b[2] != c[2] 都为真则a[3]为世爻a[0]为应爻
b[0] != c[0] 与 b[1] == c[1] 与 b[2] == c[2] 都为真则a[0]为世爻a[3]为应爻

b[1] == c[1] 与 b[0] != c[0] 与 b[2] != c[2] 都为真则a[3]为世爻a[0]为应爻
b[1] != c[1] 与 b[0] == c[0] 与 b[2] == c[2] 都为真则a[2]为世爻a[5]为应爻

b[0] == c[0] 与 b[1] == c[1] 与 b[2] == c[2] 都为真则a[5]为世爻a[2]为应爻
b[0] != c[0] 与 b[1] != c[1] 与 b[2] != c[2] 都为真则a[2]为世爻a[5]为应爻
---
单独创建结构体hun_yuan用于保存八个本宫卦对应内卦和外卦的地支和五行(写在一起)，还要有索引匹配8个本宫卦，乾是111,震是122,坎是212,艮是221，坤是222,巽是211，离是121，兑是112(这个结构体的字段有index,nei,wai，使用一个数组来保存这些数据)
乾在内卦，子水、寅木、辰土；乾在外卦，午火、申金、戌土。
坎在内卦，寅木、辰土、午火；坎在外卦，申金、戌土、子水。
艮在内卦，辰土、午火、申金；艮在外卦，戌土、子水、寅木。
震在内卦，子水、寅木、辰土；震在外卦，午火、申金、戌土。
巽在内卦，丑土、亥水、酉金；巽在外卦，未土、巳火、卯木。
离在内卦，卯木、丑土、亥水；离在外卦，酉金、未土、巳火。
坤在内卦，未土、巳火、卯木；坤在外卦，丑土、亥水、酉金。
兑在内卦，巳火、卯木、丑土，兑在外卦，亥水、酉金、未土。
---
nei数组拼接成字符串匹配结构体HunTian的index拿取到HunTian的nei，gua_xian前三个元素开头加上HunTian的nei地支和五行，例如"子水━━ ━━ x"
wai数组拼接成字符串匹配结构体HunTian的index拿取到HunTian的wai，gua_xian前三个元素开头加上HunTian的wai地支和五行，例如"午火━━ ━━ x"
---
接下来根据五行给卦象的字体添加颜色，包含土为棕色，木为绿色，火为红色，金为黄色，水为蓝色，编辑js判断爻的五行，编辑css添加颜色，后端代码不要动。
---
木生火，火生土，土生金，金生水，水生木
木克土，土克水，水克火，火克金，金克木
根据世爻的五行生克关系判断六亲(**错误的，需要使用卦宫的五行作为主角**)，生我者为父母，我生者为子孙，我克者为妻财，克我者为官鬼，比和者(相同五行)为兄弟。
具体逻辑如下：gua_xian[shi_idx]为世爻，目前第二个字符为五行，遍历gua_xian数组，如果第二个字符和gua_xian[shi_idx]的第二个字符相同，则为兄弟根据五行生克关系依次类推。然后把得到的六亲添加到当前遍历元素的开头，示例效果："兄弟午火━━ ━━ x"
---
确定世爻和应爻的位置(62到80行)单独保存成方法，传入参数是nei和wai两个数组，返回值是shi_idx和ying_idx，同时保证原来的功能不变
---
告诉我六十四卦和对应卦宫
---
假设阳为1阴为2,现在需要使用一个rust结构体来记录这些数据，包括卦宫，卦宫数字(从下到上依次用1和2从左到有写不是二进制)，卦宫五行属性，每个卦宫对应的八卦名称(数组)，每个卦宫对应的八卦数字(数组，从下到上依次用1和2从左到有写不是二进制)，这个结构体也用数组装填完整的数据
---
像乘法口诀表一样把八宫六十四卦全部用python代码计算出来八纯卦、一世到五世，游魂、归魂卦，1表示阳2表示阴不需要卦名，下卦在前上卦在后(开头加上卦宫名称以区分)
---
写一个方法：传递参数数组nei和数组wai拼接成字符串，如果可以在SIXTYFOURGUA_DATA的palace_gua_index找到，返回对应的palace_element用于六亲的判断，注意比较时的类型
---
把当前判断六亲的代码(195到227行)提取成方法，参数是palace_element和gua_xian指针，保证原来的功能不变
---
追加地支和五行提取成方法(205到225行)，传入参数是数组nei和wai以及gua_xian的指针，保证原来的功能不变
---
102行的确定世爻和应爻的位置方法删除返回值，增加一个gua_xian的指针参数，把228到230行作移植到该方法
---
gua_xiang所有元素后面加一个空格再挨个拼接上bian_gua_xiang的所有元素
---
let palace_element = find_palace_element(nei, wai).unwrap_or("未知");
这行的palace_element希望提取成全局变量，判断palace_element是否已经赋值，没有赋值才执行palace_element = find_palace_element(nei, wai).unwrap_or("未知")，不要使用unsafe
---
现在的正卦和变卦是通过一个"\t"拼接在一起的，js拿到后需要分开，分别根据五行添加颜色(相关代码提取成方法复用)
---
当前正卦和变卦写在了不同行，非常不方便查看，正卦和变卦必须和原来一样分别根据各自的五行来标识颜色不然不好判断生克关系，确保这个优化一下整体外观,用两个左右的方框分别在左右绘制正卦和变卦
---
切换到竖屏的时候卦象的文字会换行，能不能自动缩小保证占满1行，不要隐藏字体
---
fn find_palace_element(nei: &[String], wai: &[String]) -> Option<&'static str> 这个方法改成输入参数为一个zheng_gua，不再需要combined，直接用zheng_gua在SIXTYFOURGUA_DATA查找；删除PALACE_ELEMENT和引用，palace_element在generate_gua_xian方法中调用新的find_palace_element赋值;process_gua方法添加palace_element参数用于判断六亲
---
需要在编译时携带上static里面的文件
---
写一个方法类似find_palace_element，传递的参数一样，在SixtyFourGua匹配到palace_gua_index返回相同索引的palace_gua例如传入"212112"返回"泽水困(六合)"；在generate_gua_xian中分别传入zheng_gua和bian_gua，返回值分别追加到zheng_xiang和bian_xiang数组结尾
---
js文件gua.split('\t')的zhengGua, bianGua的第0个元素需要居中，并且颜色为黑色
--------
现在需要在生成卦象下面展示干支信息(纪年、纪月、纪日和纪时)打算通过lunar_rust来获取，展示类似丙寅年 癸巳月 癸酉日 子时
---
颜色逻辑判断更改为通过地支来判断，干支使用不同颜色显示年月日时，例如乙巳年 辛巳月 壬辰日 申时对应的颜色是红 红 棕 黄，判断逻辑和卦名一致
---
generate_gua_xian在调用完process_zheng_gua方法后第三个字是地支，其中世爻和应爻可以根据调用完process_zheng_gua是否存在'世'或'应'字符来判断(或者调用determine_shi_ying_indices时可以拿到索引)，删除变卦符号时的0和3对应的爻是动爻；地支六冲是子午 丑未 寅申 卯酉 辰戌 巳亥 地支六合是子丑 寅亥 卯戌 辰酉 巳申 午未，现在需要把干支中的月和日对于世爻、应爻、动爻中的冲克关系追加到正卦对应爻的后面，比如月冲 月合 日冲 日合；
---
请帮我把struct SixtyFourGua中的nei和wai分别拆分成nei_dizhi、nei_wuxing和wai_dizhi、wai_wuxing
对应的SIXTYFOURGUA_DATA中的数据也拆分开，例如原来的
nei: ["子水", "寅木", "辰土"],
wai: ["午火", "申金", "戌土"],
拆分成
nei_dizhi: ["子", "寅", "辰"],
nei_wuxing: ["水", "木", "土"],
wai_dizhi: ["午", "申", "戌"],
wai_wuxing: ["火", "金", "土"],
在fn append_dizhi_wuxing中使用了里面的数据，拆分成两个fn append_dizhi和fn append_dizhi
在fn generate_gua_xian中调用了fn append_dizhi_wuxing的地方改成先后调用fn append_dizhi和fn append_dizhi
---
我写了一个结构体
struct Gua {
    shi_idx: &'static str,
    ying_idx: &'static str,
    dizhi: [&'static str; 6],
    wuxing: [&'static str; 6],
    liuqin: [&'static str; 6],
    palace_element: &'static str,
    palace_name: &'static str,
}
现在需要把fn generate_gua_xian中的zheng_gua和bian_gua的数据类型都改成Gua，
对与fn append_dizhi、fn append_wuxing和fn append_liuqin中使用的参数gua_xian: &mut [String]都改成Gua的数据(fn process_gua的参数xiang: &mut Vec<String>也改成Gua的数据，在fn generate_gua_xian调用时分别传入zheng_gua和bian_gua)，
里面使用了format的添加操作替换成把对应的数据放到Gua中，
fn generate_gua_xian中zheng_gua的追加冲合关系和bian_gua追加卦名的操作前先读取zheng_gua和bian_gua数据把format放到这里完成，
最终达到展现的效果不变，zheng_gua和bian_gua都用改成Gua的数据类型
---
五行的生克关系如下：
木生火，火生土，土生金，金生水，水生木  
木克土，土克水，水克火，火克金，金克木  
地支的五行属性如下：
| 地支 | 五行 |
|------|------|
| 子   | 水   |
| 丑   | 土   |
| 寅   | 木   |
| 卯   | 木   |
| 辰   | 土   |
| 巳   | 火   |
| 午   | 火   |
| 未   | 土   |
| 申   | 金   |
| 酉   | 金   |
| 戌   | 土   |
| 亥   | 水   |
现在需要添加一个判断生克的函数get_sheng_ke_relation，传入的参数是两个五行就直接判断，传入的参数如果存在地支，就先转化为五行再判断(格式参考原来判断冲合的get_chong_he_relation，判断部分可以参考append_liu_qin)。
然后在格式化正卦的每一爻的时候调研该函数追加上月和日的生克关系
---
现在需要把正卦中出现的`世`和`应`这两个字单独使用黑色显示，要如何修改js代码？
这里的`按空格分割字符串，以处理每个部分`把我原来的空格删除了，而且`在每个部分后面添加一个空格，除了最后一个`并没有生效
效果依然不理想，考虑别的解决方式。在后端代码发送数据的时候能否添加一些html代码包裹`世`和`应`设置颜色？
---
在此基础上，对于动爻(`enum Yao`中的`YinChanging`和`YangChanging`)需要在变卦中添加回头生、克、冲、合关系，即拿变卦和正卦的地支判断冲、合，拿变卦和正卦的五行判断生、克。然后把生、克、冲、合关系作为变卦的`relations_text`添加上去，请帮我实现这一功能
---
世应的确定改成在六十四卦结构体中直接拿取节省开销，世应的位置在游魂归魂等卦是固定的，可以写到结构体的默认值中。
















