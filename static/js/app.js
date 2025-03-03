// 提取颜色添加逻辑为可复用的方法
function addWuxingColorClass(element, text) {
    if (text.includes('土')) {
        element.classList.add('earth');
    } else if (text.includes('木')) {
        element.classList.add('wood');
    } else if (text.includes('火')) {
        element.classList.add('fire');
    } else if (text.includes('金')) {
        element.classList.add('metal');
    } else if (text.includes('水')) {
        element.classList.add('water');
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

        data.gua_xian.reverse().forEach(gua => {
            // 将正卦和变卦分开
            const [zhengGua, bianGua] = gua.split('\t');

            // 创建包含正卦和变卦的div
            const guaDiv = document.createElement('div');
            guaDiv.style.display = 'flex';
            guaDiv.style.justifyContent = 'space-between';

            // 创建正卦的span
            const zhengSpan = document.createElement('span');
            zhengSpan.textContent = zhengGua;
            addWuxingColorClass(zhengSpan, zhengGua);
            guaDiv.appendChild(zhengSpan);

            // 创建变卦的span
            const bianSpan = document.createElement('span');
            bianSpan.textContent = bianGua;
            addWuxingColorClass(bianSpan, bianGua);
            guaDiv.appendChild(bianSpan);

            guaDiagram.appendChild(guaDiv);
        });

        document.getElementById('result').classList.add('show');
    } else {
        alert("生成卦象失败，请重试");
    }
}
