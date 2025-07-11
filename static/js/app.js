//根据地支属性展现不同的颜色
function addWuxingColorClass(element, branchChar) {
    element.classList.remove('earth', 'wood', 'fire', 'metal', 'water');
    switch (branchChar) {
        case '子':
        case '亥':
            element.classList.add('water'); // 水 - Blue
            break;
        case '寅':
        case '卯':
            element.classList.add('wood');  // 木 - Green
            break;
        case '巳':
        case '午':
            element.classList.add('fire');  // 火 - Red
            break;
        case '申':
        case '酉':
            element.classList.add('metal'); // 金 - Yellow
            break;
        case '辰':
        case '戌':
        case '丑':
        case '未':
            element.classList.add('earth'); // 土 - Brown
            break;
    }
}

async function generateGuaXiang() {
    const input = document.getElementById('yaoInput').value;
    if (input.length !== 6) {
        alert("请输入6位数字（0-3）");
        return;
    }

    const response = await fetch('/generate_gua_xian', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ numbers: input }),
    });

    if (response.ok) {
        const data = await response.json();
        const guaDiagram = document.getElementById('guaDiagram');
        guaDiagram.innerHTML = '';

        // 展示干支信息 with colors
        const ganzhiInfoDiv = document.createElement('div');
        ganzhiInfoDiv.classList.add('ganzhi-info');
        const ganzhiParts = [
            { text: data.year_ganzhi + '年', branchChar: data.year_ganzhi.charAt(1) },
            { text: data.month_ganzhi + '月', branchChar: data.month_ganzhi.charAt(1) },
            { text: data.day_ganzhi + '日', branchChar: data.day_ganzhi.charAt(1) },
            { text: data.hour_ganzhi + '时', branchChar: data.hour_ganzhi.charAt(0) }
        ];
        ganzhiParts.forEach((part, index) => {
            const span = document.createElement('span');
            span.textContent = part.text;
            addWuxingColorClass(span, part.branchChar);
            ganzhiInfoDiv.appendChild(span);
            if (index < ganzhiParts.length - 1) {
                ganzhiInfoDiv.appendChild(document.createTextNode(' '));
            }
        });
        guaDiagram.appendChild(ganzhiInfoDiv);

        // 反转数组以正确顺序显示 (初爻在下，上爻在上)
        data.gua_lines.reverse().forEach((lineData, index) => {
            const guaDiv = document.createElement('div');
            guaDiv.style.display = 'flex';

            const zhengDiv = document.createElement('div');
            zhengDiv.style.flex = '1';

            const bianDiv = document.createElement('div');
            bianDiv.style.flex = '1';
            
            // 添加基础文本 (六亲、地支、五行、爻象)
            zhengDiv.appendChild(document.createTextNode(lineData.base_text));

            // 如果是世或应，创建一个独立的、带样式的 <span>
            if (lineData.role === 'Shi' || lineData.role === 'Ying') {
                const roleSpan = document.createElement('span');
                roleSpan.className = 'role-tag'; // 使用一个通用类
                roleSpan.textContent = lineData.role === 'Shi' ? ' 世' : ' 应';
                zhengDiv.appendChild(roleSpan);
            }

            // 添加关系文本 (月日冲合生克)
            zhengDiv.appendChild(document.createTextNode(lineData.relations_text));
            
            // 变卦内容依然简单
            bianDiv.textContent = lineData.bian_text;

            // 应用五行颜色到整个 zhengDiv 和 bianDiv
            if (index > 0) { // 跳过卦名行
                addWuxingColorClass(zhengDiv, lineData.base_text.charAt(2));
                addWuxingColorClass(bianDiv, lineData.bian_text.charAt(2));
            } else {
                 zhengDiv.style.textAlign = 'center';
                 bianDiv.style.textAlign = 'center';
            }

            guaDiv.appendChild(zhengDiv);
            guaDiv.appendChild(bianDiv);
            guaDiagram.appendChild(guaDiv);
        });

        document.getElementById('result').classList.add('show');
    } else {
        alert("生成卦象失败，请重试");
    }
}
