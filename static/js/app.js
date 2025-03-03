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

            // 创建包含正卦和变卦的父div
            const guaDiv = document.createElement('div');
            guaDiv.style.display = 'flex';
            guaDiv.style.justifyContent = 'space-between';
            guaDiv.style.alignItems = 'center';
            guaDiv.style.marginBottom = '1rem';

            // 创建正卦的div
            const zhengDiv = document.createElement('div');
            zhengDiv.style.borderRadius = '8px';
            zhengDiv.style.padding = '0.5rem';
            zhengDiv.style.flex = '1';
            zhengDiv.style.marginRight = '0.5rem';
            zhengDiv.textContent = zhengGua;
            addWuxingColorClass(zhengDiv, zhengGua);
            guaDiv.appendChild(zhengDiv);

            // 创建变卦的div
            const bianDiv = document.createElement('div');
            bianDiv.style.borderRadius = '8px';
            bianDiv.style.padding = '0.5rem';
            bianDiv.style.flex = '1';
            bianDiv.style.marginLeft = '0.5rem';
            bianDiv.textContent = bianGua;
            addWuxingColorClass(bianDiv, bianGua);
            guaDiv.appendChild(bianDiv);

            guaDiagram.appendChild(guaDiv);
        });

        document.getElementById('result').classList.add('show');
    } else {
        alert("生成卦象失败，请重试");
    }
}
