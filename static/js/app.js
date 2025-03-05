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
                addWuxingColorClass(zhengDiv, zhengGua);
                addWuxingColorClass(bianDiv, bianGua);
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
