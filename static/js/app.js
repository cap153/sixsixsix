// Updated function to add color based on the Earthly Branch character
function addWuxingColorClass(element, branchChar) {
    // Clear any existing wuxing color classes to prevent conflicts
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

        // 展示干支信息
        const ganzhiInfo = document.createElement('div');
        ganzhiInfo.classList.add('ganzhi-info');
        ganzhiInfo.textContent = `${data.year_ganzhi}年 ${data.month_ganzhi}月 ${data.day_ganzhi}日 ${data.hour_ganzhi}时`;
        guaDiagram.appendChild(ganzhiInfo);

        data.gua_xian.reverse().forEach((gua, index) => {
            // 将正卦和变卦分开
            const [zhengGua, bianGua] = gua.split('\t');

            const guaDiv = document.createElement('div');
            guaDiv.style.display = 'flex';
            
            // 创建正卦div
            const zhengDiv = document.createElement('div');
            zhengDiv.textContent = zhengGua;
            
            // 创建变卦div
            const bianDiv = document.createElement('div');
            bianDiv.textContent = bianGua;

            // 卦名使用黑色
            if (index === 0) {
                zhengDiv.style.cssText = 'text-align: center; color: black; flex: 1';
                bianDiv.style.cssText = 'text-align: center; color: black; flex: 1';
            } else {
                zhengDiv.style.flex = '1';
                bianDiv.style.flex = '1';
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
