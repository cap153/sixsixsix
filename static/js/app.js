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
            const div = document.createElement('div');
            div.textContent = gua;

            // 根据五行添加颜色类
            if (gua.includes('土')) {
                div.classList.add('earth');
            } else if (gua.includes('木')) {
                div.classList.add('wood');
            } else if (gua.includes('火')) {
                div.classList.add('fire');
            } else if (gua.includes('金')) {
                div.classList.add('metal');
            } else if (gua.includes('水')) {
                div.classList.add('water');
            }

            guaDiagram.appendChild(div);
        });

        document.getElementById('result').classList.add('show');
    } else {
        alert("生成卦象失败，请重试");
    }
}