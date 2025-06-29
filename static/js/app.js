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

        data.gua_xian.reverse().forEach((gua, index) => {
            const [zhengGua, bianGua] = gua.split('\t');

            const guaDiv = document.createElement('div');
            guaDiv.style.display = 'flex';
            
            const zhengDiv = document.createElement('div');
            const bianDiv = document.createElement('div');

            if (index === 0) { // 卦名行，使用 textContent 即可
                zhengDiv.textContent = zhengGua;
                bianDiv.textContent = bianGua;
                zhengDiv.style.cssText = 'text-align: center; color: black; flex: 1';
                bianDiv.style.cssText = 'text-align: center; color: black; flex: 1';
            } else { // 爻辞行，需要处理 HTML
                zhengDiv.style.flex = '1';
                bianDiv.style.flex = '1';
                // 使用 innerHTML 替代 textContent 来渲染 HTML 标签
                zhengDiv.innerHTML = zhengGua;
                bianDiv.innerHTML = bianGua;
                addWuxingColorClass(zhengDiv, zhengGua.charAt(2));
                addWuxingColorClass(bianDiv, bianGua.charAt(2));
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
